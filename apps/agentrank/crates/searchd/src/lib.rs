//! Axum search API: `POST /v1/search`, `GET /v1/agents/:id`, `GET /health`, `GET /ready`, `GET /metrics`.

mod error;
mod handlers;
mod metrics;
mod models;
mod rate_limit;

pub use error::{ApiError, ErrorBody};
pub use handlers::AgentDetailResponse;
pub use models::{SearchRequest, SearchResponse, SearchResultItem, MAX_LIMIT};

use agentrank_search_index::schema::AgentSchema;
use agentrank_search_index::store::open_index;
use axum::http::Method;
use axum::middleware::from_fn;
use axum::routing::{get, post};
use axum::Router;
use redis::aio::MultiplexedConnection;
use sqlx::PgPool;
use std::path::PathBuf;
use tantivy::{Index, IndexReader};
use tower_http::cors::{AllowOrigin, Any, CorsLayer};
use tower_http::trace::TraceLayer;

/// Shared application state.
#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub redis: MultiplexedConnection,
    pub redis_url: String,
    pub reader: IndexReader,
    pub index: Index,
    pub agent_schema: AgentSchema,
    pub rate_limit_per_minute: u64,
    pub index_path: PathBuf,
    /// When true, rate limiting uses `X-Forwarded-For` / `X-Real-IP` (trusted reverse proxy).
    pub trust_proxy_headers: bool,
}

/// When `CORS_REQUIRE_ORIGINS` is set, `CORS_ORIGINS` must list at least one valid origin
/// (no wildcard `Any` in production).
fn build_cors_layer() -> anyhow::Result<CorsLayer> {
    let require = env_flag_truthy("CORS_REQUIRE_ORIGINS");
    let origins = std::env::var("CORS_ORIGINS").unwrap_or_default();
    let t = origins.trim();
    if require && t.is_empty() {
        anyhow::bail!(
            "CORS_REQUIRE_ORIGINS is set but CORS_ORIGINS is empty; set explicit origins for production"
        );
    }
    let base = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers(Any);
    if t.is_empty() {
        return Ok(base.allow_origin(Any));
    }
    let headers: Vec<axum::http::HeaderValue> = t
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .filter_map(|s| s.parse().ok())
        .collect();
    if headers.is_empty() {
        if require {
            anyhow::bail!("CORS_ORIGINS contained no valid origin header values");
        }
        return Ok(base.allow_origin(Any));
    }
    Ok(base.allow_origin(AllowOrigin::list(headers)))
}

/// Build router (for tests and production).
pub fn app_router(state: AppState, cors: CorsLayer) -> Router {
    Router::new()
        .route("/health", get(handlers::health))
        .route("/ready", get(handlers::ready))
        .route("/metrics", get(metrics::prometheus))
        .route("/v1/search", post(handlers::search))
        .route("/v1/agents/:id", get(handlers::get_agent))
        .layer(from_fn(metrics::http_metrics))
        .layer(TraceLayer::new_for_http())
        .layer(cors)
        .with_state(state)
}

/// Install Prometheus metrics (call once before `build_app` / `serve`).
pub fn init_metrics() -> anyhow::Result<()> {
    metrics::init()
}

/// Open index from `SEARCH_INDEX_PATH`, connect Redis, return router.
pub async fn build_app(
    pool: PgPool,
    redis_url: &str,
    index_path: &std::path::Path,
) -> anyhow::Result<Router> {
    let client = redis::Client::open(redis_url)?;
    let redis = client.get_multiplexed_async_connection().await?;
    let rate_limit_per_minute: u64 = std::env::var("SEARCH_RATE_LIMIT_PER_MINUTE")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(120);

    let trust_proxy_headers = resolve_trust_proxy_headers();

    let (index, agent_schema) = open_index(index_path)?;
    let reader = index.reader()?;

    let state = AppState {
        pool,
        redis,
        redis_url: redis_url.to_string(),
        reader,
        index,
        agent_schema,
        rate_limit_per_minute,
        index_path: index_path.to_path_buf(),
        trust_proxy_headers,
    };
    let cors = build_cors_layer()?;
    Ok(app_router(state, cors))
}

fn env_flag_truthy(name: &str) -> bool {
    match std::env::var(name) {
        Ok(s) => {
            let t = s.trim().to_ascii_lowercase();
            matches!(t.as_str(), "1" | "true" | "yes")
        }
        Err(_) => false,
    }
}

/// Rate-limit key from `X-Forwarded-For` / `X-Real-IP` only when safe.
/// - `TRUST_PROXY_HEADERS=1|true|yes` → trust headers.
/// - `TRUST_PROXY_HEADERS=0|false|no` → use TCP peer only.
/// - If unset: trust on Railway (`RAILWAY_ENVIRONMENT` / `RAILWAY_PROJECT_ID`), else distrust (local dev).
fn resolve_trust_proxy_headers() -> bool {
    match std::env::var("TRUST_PROXY_HEADERS") {
        Ok(s) => {
            let t = s.trim().to_ascii_lowercase();
            match t.as_str() {
                "0" | "false" | "no" => false,
                "1" | "true" | "yes" => true,
                _ => railway_hosting_detected(),
            }
        }
        Err(_) => railway_hosting_detected(),
    }
}

fn railway_hosting_detected() -> bool {
    std::env::var("RAILWAY_ENVIRONMENT").is_ok() || std::env::var("RAILWAY_PROJECT_ID").is_ok()
}

/// Serve with graceful shutdown (Ctrl+C). Enables [`axum::extract::connect_info::ConnectInfo`]
/// for rate limiting when `TRUST_PROXY_HEADERS` is unset/false.
pub async fn serve(router: Router, port: u16) -> anyhow::Result<()> {
    use std::net::SocketAddr;

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::info!(%addr, "searchd listening");
    axum::serve(
        listener,
        router.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown_signal())
    .await?;
    Ok(())
}

async fn shutdown_signal() {
    #[cfg(unix)]
    {
        use tokio::signal::unix::{signal, SignalKind};
        let mut sigterm = match signal(SignalKind::terminate()) {
            Ok(s) => s,
            Err(e) => {
                tracing::warn!("sigterm unavailable: {e}; falling back to ctrl-c only");
                let _ = tokio::signal::ctrl_c().await;
                return;
            }
        };
        tokio::select! {
            _ = tokio::signal::ctrl_c() => {},
            _ = sigterm.recv() => {},
        }
    }
    #[cfg(not(unix))]
    {
        let _ = tokio::signal::ctrl_c().await;
    }
    tracing::info!("shutdown signal received");
}
