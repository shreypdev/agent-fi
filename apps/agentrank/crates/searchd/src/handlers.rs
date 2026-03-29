use crate::error::{ApiError, ErrorBody};
use crate::fusion::rrf_fuse_weighted;
use crate::models::{
    HintsRequest, HintsResponse, SearchRequest, SearchResponse, SearchResultItem, MAX_LIMIT,
};
use crate::rate_limit::{check_hints_daily_limit, check_search_rate_limit};
use crate::AppState;
use agentrank_crawl_policy::validate_outbound_url;
use agentrank_embed::embed_text;
use agentrank_frontier::{FrontierMeta, UrlFrontier, DEFAULT_FRONTIER_KEY};
use agentrank_search_index::search::search_agents;
use agentrank_vector::search_knn;
use axum::extract::connect_info::ConnectInfo;
use axum::extract::{Path, State};
use axum::http::{HeaderMap, StatusCode};
use axum::Json;
use metrics::histogram;
use sqlx::FromRow;
use std::net::SocketAddr;
use std::time::Instant;
use url::Url;
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

    if let Some(ref qd) = state.qdrant {
        agentrank_vector::health_check(qd.as_ref())
            .await
            .map_err(|e| ApiError {
                status: StatusCode::SERVICE_UNAVAILABLE,
                body: ErrorBody {
                    error: "not_ready",
                    message: format!("qdrant: {e}"),
                },
                retry_after_secs: None,
            })?;
    }

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

const HYBRID_TOP: usize = 500;
const RRF_K: f64 = 60.0;
const W_LEX: f64 = 0.4;
const W_SEM: f64 = 0.6;

/// Hybrid / BM25 search + Postgres hydration (no rate limit). Used by REST and MCP tools.
pub(crate) async fn search_execute(
    state: &AppState,
    q: &str,
    limit: u32,
    offset: u32,
) -> Result<(Vec<SearchResultItem>, u64), ApiError> {
    let started = Instant::now();
    let reader = state.reader.clone();

    let need = (offset as usize).saturating_add(limit as usize);
    let t_fuse = Instant::now();

    let ordered_ids: Vec<Uuid> = if let Some(ref qd) = state.qdrant {
        let lex_cap = HYBRID_TOP.max(need);
        let lex_hits = search_agents(&reader, &state.index, &state.agent_schema, q, lex_cap)
            .map_err(|e| ApiError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                body: ErrorBody {
                    error: "search_error",
                    message: e.to_string(),
                },
                retry_after_secs: None,
            })?;
        let lex_ids: Vec<Uuid> = lex_hits.iter().map(|h| h.agent_id).collect();

        let qvec = embed_text(q);
        let sem = search_knn(qd.as_ref(), qvec, HYBRID_TOP as u64)
            .await
            .map_err(|e| ApiError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                body: ErrorBody {
                    error: "vector_search_error",
                    message: e.to_string(),
                },
                retry_after_secs: None,
            })?;
        let sem_ids: Vec<Uuid> = sem.iter().map(|(id, _)| *id).collect();
        histogram!("search_fusion_latency_seconds").record(t_fuse.elapsed().as_secs_f64());
        rrf_fuse_weighted(&lex_ids, &sem_ids, RRF_K, W_LEX, W_SEM)
    } else {
        let hits = search_agents(&reader, &state.index, &state.agent_schema, q, need.max(1))
            .map_err(|e| ApiError {
                status: StatusCode::INTERNAL_SERVER_ERROR,
                body: ErrorBody {
                    error: "search_error",
                    message: e.to_string(),
                },
                retry_after_secs: None,
            })?;
        hits.into_iter().map(|h| h.agent_id).collect()
    };

    let page: Vec<Uuid> = ordered_ids
        .into_iter()
        .skip(offset as usize)
        .take(limit as usize)
        .collect();

    if page.is_empty() {
        return Ok((vec![], started.elapsed().as_millis() as u64));
    }

    let ids: Vec<Uuid> = page;
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

    let mut results = Vec::with_capacity(ids.len());
    for id in ids {
        let Some(row) = map.get(&id) else {
            tracing::warn!(agent_id = %id, "search hit missing from postgres");
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

    Ok((results, started.elapsed().as_millis() as u64))
}

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

    let (results, query_time_ms) = search_execute(&state, q, limit, offset).await?;

    Ok(Json(SearchResponse {
        results,
        total_hits_estimate: None,
        query_time_ms,
    }))
}

fn hints_daily_cap(headers: &HeaderMap) -> u64 {
    let Ok(want) = std::env::var("SEARCH_API_KEY") else {
        return 5;
    };
    let want = want.trim();
    if want.is_empty() {
        return 5;
    }
    let Some(auth) = headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
    else {
        return 5;
    };
    let Some(tok) = auth.strip_prefix("Bearer ") else {
        return 5;
    };
    if tok == want {
        50
    } else {
        5
    }
}

/// `POST /v1/hints` — validate URL, enqueue to frontier (no HTTP fetch).
pub async fn post_hints(
    State(state): State<AppState>,
    ConnectInfo(peer): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    Json(req): Json<HintsRequest>,
) -> Result<Json<HintsResponse>, ApiError> {
    let u = req.url.trim();
    if u.is_empty() {
        return Err(ApiError {
            status: StatusCode::BAD_REQUEST,
            body: ErrorBody {
                error: "invalid_url",
                message: "url must be non-empty".into(),
            },
            retry_after_secs: None,
        });
    }

    let ip = client_ip(&headers, &peer, state.trust_proxy_headers);
    let mut redis = state.redis.clone();
    let cap = hints_daily_cap(&headers);
    let allowed = check_hints_daily_limit(&mut redis, &ip, cap)
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
        return Err(ApiError {
            status: StatusCode::TOO_MANY_REQUESTS,
            body: ErrorBody {
                error: "rate_limited",
                message: "hint submission daily limit exceeded".into(),
            },
            retry_after_secs: Some(86_400),
        });
    }

    let allow_localhost = std::env::var("SEARCH_HINTS_ALLOW_HTTP_LOCALHOST")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false);
    let allow_loopback_https = std::env::var("SEARCH_HINTS_ALLOW_LOOPBACK_HTTPS")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false);

    let parsed = Url::parse(u).map_err(|e| ApiError {
        status: StatusCode::BAD_REQUEST,
        body: ErrorBody {
            error: "invalid_url",
            message: e.to_string(),
        },
        retry_after_secs: None,
    })?;

    validate_outbound_url(&parsed, allow_localhost, allow_loopback_https).map_err(|e| {
        ApiError {
            status: StatusCode::BAD_REQUEST,
            body: ErrorBody {
                error: "url_not_allowed",
                message: e.to_string(),
            },
            retry_after_secs: None,
        }
    })?;

    let card_url = parsed.as_str().to_string();
    let f = UrlFrontier::new(DEFAULT_FRONTIER_KEY);
    let meta = FrontierMeta::new("searchd_hints");
    f.enqueue_with_meta(&mut redis, &card_url, 1.0, &meta)
        .await
        .map_err(|e| ApiError {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            body: ErrorBody {
                error: "enqueue_error",
                message: e.to_string(),
            },
            retry_after_secs: None,
        })?;

    let _ = req.source;

    Ok(Json(HintsResponse {
        hint_id: Uuid::new_v4().to_string(),
        queued: true,
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

pub(crate) async fn get_agent_inner(
    state: &AppState,
    id: Uuid,
) -> Result<AgentDetailResponse, ApiError> {
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

    Ok(AgentDetailResponse {
        agent_id: row.id,
        name: row.name,
        description: row.description,
        endpoint_url: row.endpoint_url,
        protocol_version: row.protocol_version,
        trust_tier: row.trust_tier.unwrap_or_else(|| "indexed".into()),
        provider_display_name: row.provider_display_name,
        card_json: row.card_json.0,
    })
}

pub async fn get_agent(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<AgentDetailResponse>, ApiError> {
    Ok(Json(get_agent_inner(&state, id).await?))
}
