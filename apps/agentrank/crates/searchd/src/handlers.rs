use crate::error::{ApiError, ErrorBody};
use crate::models::{SearchRequest, SearchResponse, SearchResultItem, MAX_LIMIT};
use crate::rate_limit::check_search_rate_limit;
use crate::AppState;
use agentrank_search_index::search::search_agents;
use axum::extract::connect_info::ConnectInfo;
use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::Json;
use sqlx::FromRow;
use std::net::SocketAddr;
use std::time::Instant;
use uuid::Uuid;

/// Client identity for rate limiting when `TRUST_PROXY_HEADERS` is set: first hop of
/// `X-Forwarded-For`, then `X-Real-IP`, else the TCP peer address.
/// When proxy headers are not trusted, only the TCP peer is used.
fn client_ip(headers: &HeaderMap, peer: &SocketAddr, trust_proxy_headers: bool) -> String {
    if trust_proxy_headers {
        if let Some(xff) = headers
            .get("x-forwarded-for")
            .or_else(|| headers.get("X-Forwarded-For"))
            .and_then(|v| v.to_str().ok())
        {
            if let Some(first) = xff.split(',').next() {
                let t = first.trim();
                if !t.is_empty() {
                    return t.to_string();
                }
            }
        }
        if let Some(xr) = headers
            .get("x-real-ip")
            .or_else(|| headers.get("X-Real-IP"))
            .and_then(|v| v.to_str().ok())
        {
            let t = xr.trim();
            if !t.is_empty() {
                return t.to_string();
            }
        }
    }
    peer.ip().to_string()
}

/// Liveness: process is accepting HTTP (no dependency checks).
pub async fn health() -> StatusCode {
    StatusCode::OK
}

/// Readiness: Postgres, Redis, and Tantivy index on disk are usable.
pub async fn ready(State(state): State<AppState>) -> Result<StatusCode, ApiError> {
    agentrank_data_plane::check_postgres(&state.pool)
        .await
        .map_err(|e| ApiError {
            status: StatusCode::SERVICE_UNAVAILABLE,
            body: ErrorBody {
                error: "not_ready",
                message: format!("postgres: {e}"),
            },
            retry_after_secs: None,
        })?;
    agentrank_data_plane::check_redis(&state.redis_url)
        .await
        .map_err(|e| ApiError {
            status: StatusCode::SERVICE_UNAVAILABLE,
            body: ErrorBody {
                error: "not_ready",
                message: format!("redis: {e}"),
            },
            retry_after_secs: None,
        })?;

    let path = state.index_path.clone();
    tokio::task::spawn_blocking(move || agentrank_search_index::store::probe_index_readable(&path))
        .await
        .map_err(|e| ApiError {
            status: StatusCode::SERVICE_UNAVAILABLE,
            body: ErrorBody {
                error: "not_ready",
                message: format!("index probe join: {e}"),
            },
            retry_after_secs: None,
        })?
        .map_err(|e| ApiError {
            status: StatusCode::SERVICE_UNAVAILABLE,
            body: ErrorBody {
                error: "not_ready",
                message: format!("index: {e}"),
            },
            retry_after_secs: None,
        })?;

    Ok(StatusCode::OK)
}

#[derive(Debug, FromRow)]
struct AgentApiRow {
    id: Uuid,
    name: String,
    description: String,
    endpoint_url: String,
    protocol_version: String,
    provider_display_name: Option<String>,
    trust_tier: Option<String>,
}

const AGENTS_BY_IDS_SQL: &str = r#"
SELECT
    a.id,
    a.name,
    a.description,
    a.endpoint_url,
    a.protocol_version,
    p.display_name AS provider_display_name,
    t.trust_tier AS trust_tier
FROM agents a
LEFT JOIN providers p ON a.provider_id = p.id
LEFT JOIN trust_records t ON t.agent_id = a.id
WHERE a.id = ANY($1)
"#;

pub async fn search(
    State(state): State<AppState>,
    ConnectInfo(peer): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    Json(req): Json<SearchRequest>,
) -> Result<Json<SearchResponse>, ApiError> {
    let q = req.query.trim();
    if q.is_empty() {
        return Err(ApiError {
            status: StatusCode::BAD_REQUEST,
            body: ErrorBody {
                error: "invalid_query",
                message: "query must be non-empty".into(),
            },
            retry_after_secs: None,
        });
    }

    let limit = req.limit.clamp(1, MAX_LIMIT);
    let offset = req.offset;

    let ip = client_ip(&headers, &peer, state.trust_proxy_headers);
    let mut redis = state.redis.clone();
    let allowed = check_search_rate_limit(&mut redis, &ip, state.rate_limit_per_minute)
        .await
        .map_err(|e| ApiError {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            body: ErrorBody {
                error: "rate_limit_error",
                message: e.to_string(),
            },
            retry_after_secs: None,
        })?;
    if !allowed {
        tracing::warn!(%ip, "search rate limited");
        return Err(ApiError {
            status: StatusCode::TOO_MANY_REQUESTS,
            body: ErrorBody {
                error: "rate_limited",
                message: "too many search requests; try again later".into(),
            },
            retry_after_secs: Some(60),
        });
    }

    let started = Instant::now();
    let reader = state.reader.clone();

    let need = (offset as usize).saturating_add(limit as usize);
    let hits =
        search_agents(&reader, &state.index, &state.agent_schema, q, need.max(1)).map_err(|e| {
            ApiError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                body: ErrorBody {
                    error: "search_error",
                    message: e.to_string(),
                },
                retry_after_secs: None,
            }
        })?;

    let hits: Vec<_> = hits
        .into_iter()
        .skip(offset as usize)
        .take(limit as usize)
        .collect();

    if hits.is_empty() {
        return Ok(Json(SearchResponse {
            results: vec![],
            total_hits_estimate: None,
            query_time_ms: started.elapsed().as_millis() as u64,
        }));
    }

    let ids: Vec<Uuid> = hits.iter().map(|h| h.agent_id).collect();
    let rows = sqlx::query_as::<_, AgentApiRow>(AGENTS_BY_IDS_SQL)
        .bind(&ids[..])
        .fetch_all(&state.pool)
        .await
        .map_err(|e| ApiError {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            body: ErrorBody {
                error: "db_error",
                message: e.to_string(),
            },
            retry_after_secs: None,
        })?;

    use std::collections::HashMap;
    let map: HashMap<Uuid, AgentApiRow> = rows.into_iter().map(|r| (r.id, r)).collect();

    let mut results = Vec::with_capacity(hits.len());
    for h in hits {
        let Some(row) = map.get(&h.agent_id) else {
            tracing::warn!(agent_id = %h.agent_id, "search hit missing from postgres");
            continue;
        };
        let snippet = snippet_from_description(&row.description, q);
        results.push(SearchResultItem {
            agent_id: row.id,
            name: row.name.clone(),
            description: row.description.clone(),
            endpoint_url: row.endpoint_url.clone(),
            protocol_version: row.protocol_version.clone(),
            trust_tier: row.trust_tier.clone().unwrap_or_else(|| "indexed".into()),
            provider_display_name: row.provider_display_name.clone(),
            snippet,
        });
    }

    Ok(Json(SearchResponse {
        results,
        total_hits_estimate: None,
        query_time_ms: started.elapsed().as_millis() as u64,
    }))
}

fn snippet_from_description(description: &str, _query: &str) -> Option<String> {
    let t = description.trim();
    if t.is_empty() {
        return None;
    }
    const MAX: usize = 240;
    if t.len() <= MAX {
        Some(t.to_string())
    } else {
        Some(format!(
            "{}…",
            t.chars().take(MAX.saturating_sub(1)).collect::<String>()
        ))
    }
}

#[derive(Debug, serde::Serialize)]
pub struct AgentDetailResponse {
    pub agent_id: Uuid,
    pub name: String,
    pub description: String,
    pub endpoint_url: String,
    pub protocol_version: String,
    pub trust_tier: String,
    pub provider_display_name: Option<String>,
    pub card_json: serde_json::Value,
}

pub async fn get_agent(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<AgentDetailResponse>, ApiError> {
    #[derive(FromRow)]
    struct Row {
        id: Uuid,
        name: String,
        description: String,
        endpoint_url: String,
        protocol_version: String,
        provider_display_name: Option<String>,
        trust_tier: Option<String>,
        card_json: sqlx::types::Json<serde_json::Value>,
    }

    let row = sqlx::query_as::<_, Row>(
        r#"
        SELECT
            a.id,
            a.name,
            a.description,
            a.endpoint_url,
            a.protocol_version,
            p.display_name AS provider_display_name,
            t.trust_tier AS trust_tier,
            a.card_json
        FROM agents a
        LEFT JOIN providers p ON a.provider_id = p.id
        LEFT JOIN trust_records t ON t.agent_id = a.id
        WHERE a.id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(&state.pool)
    .await
    .map_err(|e| ApiError {
        status: StatusCode::INTERNAL_SERVER_ERROR,
        body: ErrorBody {
            error: "db_error",
            message: e.to_string(),
        },
        retry_after_secs: None,
    })?
    .ok_or_else(|| ApiError {
        status: StatusCode::NOT_FOUND,
        body: ErrorBody {
            error: "not_found",
            message: format!("no agent {id}"),
        },
        retry_after_secs: None,
    })?;

    Ok(Json(AgentDetailResponse {
        agent_id: row.id,
        name: row.name,
        description: row.description,
        endpoint_url: row.endpoint_url,
        protocol_version: row.protocol_version,
        trust_tier: row.trust_tier.unwrap_or_else(|| "indexed".into()),
        provider_display_name: row.provider_display_name,
        card_json: row.card_json.0,
    }))
}
