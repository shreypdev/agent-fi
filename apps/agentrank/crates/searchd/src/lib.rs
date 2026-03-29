//! Axum search API: `POST /v1/search`, `GET /v1/agents/:id`, `GET /health`.

mod error;
mod handlers;
mod models;
mod rate_limit;

pub use error::{ApiError, ErrorBody};
pub use handlers::AgentDetailResponse;
pub use models::{SearchRequest, SearchResponse, SearchResultItem, MAX_LIMIT};

use agentrank_search_index::schema::AgentSchema;
use agentrank_search_index::store::open_index;
use axum::http::Method;
use axum::routing::{get, post};
use axum::Router;
use redis::aio::MultiplexedConnection;
use sqlx::PgPool;
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
}

fn cors_layer() -> CorsLayer {
    let origins = std::env::var("CORS_ORIGINS").unwrap_or_default();
    let t = origins.trim();
    let base = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers(Any);
    if t.is_empty() {
        return base.allow_origin(Any);
    }
    let headers: Vec<axum::http::HeaderValue> = t
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .filter_map(|s| s.parse().ok())
        .collect();
    if headers.is_empty() {
        base.allow_origin(Any)
    } else {
        base.allow_origin(AllowOrigin::list(headers))
    }
}

/// Build router (for tests and production).
pub fn app_router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(handlers::health))
        .route("/v1/search", post(handlers::search))
        .route("/v1/agents/:id", get(handlers::get_agent))
        .layer(TraceLayer::new_for_http())
        .layer(cors_layer())
        .with_state(state)
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
    };
    Ok(app_router(state))
}

/// Serve with graceful shutdown (Ctrl+C).
pub async fn serve(router: Router, port: u16) -> anyhow::Result<()> {
    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::info!(%addr, "searchd listening");
    axum::serve(listener, router.into_make_service())
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
