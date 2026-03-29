//! Agent Search Console HTTP API v0.1 — authenticated BFF over Postgres.
//!
//! Env: `DATABASE_URL`, `CONSOLE_API_KEY` (required for `/v1/*`), optional `PORT` (default 8091),
//! optional `CONSOLE_CORS_ORIGIN` (single origin for browser UI).

use agentrank_consoled::{app, AppState};
use agentrank_data_plane::database_url;
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let db = database_url().map_err(|e| anyhow::anyhow!("DATABASE_URL: {e}"))?;
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&db)
        .await?;
    sqlx::migrate!("../../migrations").run(&pool).await?;

    let api_key = std::env::var("CONSOLE_API_KEY")
        .map_err(|_| anyhow::anyhow!("CONSOLE_API_KEY must be set"))?;
    let api_key = api_key.trim();
    if api_key.is_empty() {
        anyhow::bail!("CONSOLE_API_KEY must be non-empty");
    }

    let state = AppState::new(pool, api_key.to_string());
    let app = app(state);

    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(8091);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!(%addr, "consoled listening");
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
