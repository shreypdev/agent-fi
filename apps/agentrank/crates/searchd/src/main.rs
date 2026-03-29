//! Search HTTP API — bind `PORT` (default 8080), env: `DATABASE_URL`, `REDIS_URL`, `SEARCH_INDEX_PATH`.

use agentrank_data_plane::database_url;
use agentrank_searchd::{build_app, init_metrics, serve};
use anyhow::Context;
use sqlx::postgres::PgPoolOptions;
use std::path::PathBuf;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let db = database_url().context("DATABASE_URL")?;
    let redis_url = std::env::var("REDIS_URL").context("REDIS_URL")?;
    let index_path: PathBuf = std::env::var("SEARCH_INDEX_PATH")
        .context("SEARCH_INDEX_PATH")?
        .into();

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&db)
        .await
        .context("postgres connect")?;

    init_metrics().context("init_metrics")?;

    let router = build_app(pool, &redis_url, &index_path)
        .await
        .context("build_app")?;

    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(8080);

    serve(router, port).await
}
