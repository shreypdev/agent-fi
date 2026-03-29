//! Agent Search Console HTTP API — library surface for tests and embedding.

use axum::body::Body;
use axum::extract::{Path, Query, State};
use axum::http::{header, Request, StatusCode};
use axum::middleware::{self, Next};
use axum::response::Response;
use axum::routing::get;
use axum::{Json, Router};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::types::Json as SqlxJson;
use sqlx::PgPool;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use uuid::Uuid;

type AgentInspectorRow = (
    Uuid,
    String,
    String,
    String,
    SqlxJson<Value>,
    String,
    String,
);

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    api_key: Arc<str>,
}

impl AppState {
    pub fn new(pool: PgPool, api_key: impl Into<Arc<str>>) -> Self {
        Self {
            pool,
            api_key: api_key.into(),
        }
    }
}

/// Full HTTP app (health + authenticated `/v1/console/*`). `api_key` must be non-empty.
pub fn app(state: AppState) -> Router {
    let cors = if let Ok(origin) = std::env::var("CONSOLE_CORS_ORIGIN") {
        let hv = origin
            .parse::<axum::http::HeaderValue>()
            .expect("invalid CONSOLE_CORS_ORIGIN");
        CorsLayer::new()
            .allow_origin(hv)
            .allow_methods([
                axum::http::Method::GET,
                axum::http::Method::POST,
                axum::http::Method::OPTIONS,
            ])
            .allow_headers([
                header::AUTHORIZATION,
                header::CONTENT_TYPE,
                header::HeaderName::from_static("x-console-key"),
            ])
    } else {
        CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any)
    };

    let st = state.clone();
    let console = Router::new()
        .route("/agents/:id", get(get_agent))
        .route("/crawl-history", get(crawl_history))
        .route("/domain-claims", get(list_claims).post(create_claim))
        .layer(middleware::from_fn_with_state(st.clone(), console_auth_mw))
        .with_state(st);

    Router::new()
        .route("/health", get(health))
        .nest("/v1/console", console)
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

async fn health() -> &'static str {
    "ok"
}

async fn console_auth_mw(
    State(app): State<AppState>,
    req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let expected = app.api_key.as_ref().trim();
    if expected.is_empty() {
        return Err(StatusCode::SERVICE_UNAVAILABLE);
    }
    let headers = req.headers();
    let bearer = headers
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .map(str::trim);
    let xkey = headers
        .get("x-console-key")
        .and_then(|v| v.to_str().ok())
        .map(str::trim);
    let ok = bearer == Some(expected) || xkey == Some(expected);
    if !ok {
        return Err(StatusCode::UNAUTHORIZED);
    }
    Ok(next.run(req).await)
}

async fn get_agent(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Value>, StatusCode> {
    let row: Option<AgentInspectorRow> = sqlx::query_as(
        r#"
        SELECT a.id, a.name, a.description, a.endpoint_url, a.card_json, COALESCE(p.primary_domain, ''), a.external_id
        FROM agents a
        LEFT JOIN providers p ON a.provider_id = p.id
        WHERE a.id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(&state.pool)
    .await
    .map_err(|e| {
        tracing::error!("db: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    let Some((
        id,
        name,
        description,
        endpoint_url,
        SqlxJson(card_json),
        primary_domain,
        external_id,
    )) = row
    else {
        return Err(StatusCode::NOT_FOUND);
    };
    Ok(Json(json!({
        "id": id,
        "name": name,
        "description": description,
        "endpoint_url": endpoint_url,
        "card_json": card_json,
        "primary_domain": primary_domain,
        "external_id": external_id,
    })))
}

#[derive(Deserialize)]
struct HistoryQuery {
    domain: String,
    #[serde(default = "default_limit")]
    limit: i64,
    #[serde(default)]
    offset: i64,
}

fn default_limit() -> i64 {
    50
}

#[derive(Serialize, sqlx::FromRow)]
struct CrawlHistoryRow {
    id: i64,
    url: String,
    agent_id: Option<Uuid>,
    http_status: Option<i32>,
    fetched_at: DateTime<Utc>,
    error_code: Option<String>,
    error_detail: Option<String>,
    response_bytes: Option<i32>,
}

async fn crawl_history(
    State(state): State<AppState>,
    Query(q): Query<HistoryQuery>,
) -> Result<Json<Vec<CrawlHistoryRow>>, StatusCode> {
    let limit = q.limit.clamp(1, 200);
    let offset = q.offset.max(0);
    let domain_pat = format!("%{}%", q.domain.trim());
    let rows: Vec<CrawlHistoryRow> = sqlx::query_as(
        r#"
        SELECT ch.id, ch.url, ch.agent_id, ch.http_status, ch.fetched_at, ch.error_code, ch.error_detail, ch.response_bytes
        FROM crawl_history ch
        LEFT JOIN agents a ON ch.agent_id = a.id
        LEFT JOIN providers p ON a.provider_id = p.id
        WHERE p.primary_domain = $1 OR ch.url ILIKE $2
        ORDER BY ch.fetched_at DESC
        LIMIT $3 OFFSET $4
        "#,
    )
    .bind(q.domain.trim())
    .bind(&domain_pat)
    .bind(limit)
    .bind(offset)
    .fetch_all(&state.pool)
    .await
    .map_err(|e| {
        tracing::error!("db: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    Ok(Json(rows))
}

#[derive(Deserialize)]
struct CreateClaimBody {
    domain: String,
}

#[derive(Serialize, sqlx::FromRow)]
struct DomainClaimRow {
    id: Uuid,
    domain: String,
    status: String,
    verification_token: String,
    created_at: DateTime<Utc>,
    verified_at: Option<DateTime<Utc>>,
}

async fn create_claim(
    State(state): State<AppState>,
    Json(body): Json<CreateClaimBody>,
) -> Result<(StatusCode, Json<DomainClaimRow>), StatusCode> {
    let domain = body.domain.trim().to_lowercase();
    if domain.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }
    let token = Uuid::new_v4().to_string();
    let row: DomainClaimRow = sqlx::query_as(
        r#"
        INSERT INTO console_domain_claims (domain, status, verification_token)
        VALUES ($1, 'pending', $2)
        ON CONFLICT (domain) DO UPDATE SET
            verification_token = EXCLUDED.verification_token,
            status = 'pending'
        RETURNING id, domain, status, verification_token, created_at, verified_at
        "#,
    )
    .bind(&domain)
    .bind(&token)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| {
        tracing::error!("db: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    Ok((StatusCode::CREATED, Json(row)))
}

async fn list_claims(
    State(state): State<AppState>,
) -> Result<Json<Vec<DomainClaimRow>>, StatusCode> {
    let rows: Vec<DomainClaimRow> = sqlx::query_as(
        r#"
        SELECT id, domain, status, verification_token, created_at, verified_at
        FROM console_domain_claims
        ORDER BY created_at DESC
        LIMIT 200
        "#,
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|e| {
        tracing::error!("db: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    Ok(Json(rows))
}
