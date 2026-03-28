//! AgentBot CLI: ingest a card URL, enqueue URLs, or process one dequeue.

use agentrank_agentbot::{default_max_body_bytes, http_client, ingest_card_url};
use agentrank_data_plane::{database_url, redis_url};
use agentrank_frontier::{UrlFrontier, DEFAULT_FRONTIER_KEY};
use clap::{Parser, Subcommand};
use sqlx::postgres::PgPoolOptions;
use tracing_subscriber::EnvFilter;

#[derive(Parser)]
#[command(name = "agentbot", about = "AgentRank AgentBot v0.1")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Fetch one URL, parse Agent Card, persist to Postgres.
    Ingest {
        /// Card document URL (e.g. https://example.com/.well-known/agent.json).
        url: String,
    },
    /// Add a URL to the Redis frontier with a priority score (higher = sooner).
    Enqueue {
        url: String,
        #[arg(short, long, default_value_t = 0.0)]
        priority: f64,
    },
    /// Pop the highest-priority URL from the frontier and ingest it.
    RunOnce,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    let cli = Cli::parse();
    match cli.command {
        Command::Ingest { url } => {
            let db = database_url().map_err(|e| anyhow::anyhow!("DATABASE_URL: {e}"))?;
            let pool = PgPoolOptions::new().max_connections(5).connect(&db).await?;
            let client = http_client()?;
            let out = ingest_card_url(&pool, &client, &url, default_max_body_bytes()).await?;
            tracing::info!(
                agent_id = %out.agent_id,
                external_id = %out.external_id,
                crawl_history_id = out.crawl_history_id,
                "ingest ok"
            );
        }
        Command::Enqueue { url, priority } => {
            let ru = redis_url().map_err(|e| anyhow::anyhow!("REDIS_URL: {e}"))?;
            let client = redis::Client::open(ru.as_str())?;
            let mut conn = client.get_multiplexed_async_connection().await?;
            let f = UrlFrontier::new(DEFAULT_FRONTIER_KEY);
            let r = f.enqueue(&mut conn, &url, priority).await?;
            tracing::info!(?r, url = %url, priority, "enqueued");
        }
        Command::RunOnce => {
            let db = database_url().map_err(|e| anyhow::anyhow!("DATABASE_URL: {e}"))?;
            let ru = redis_url().map_err(|e| anyhow::anyhow!("REDIS_URL: {e}"))?;
            let pool = PgPoolOptions::new().max_connections(5).connect(&db).await?;
            let client = redis::Client::open(ru.as_str())?;
            let mut conn = client.get_multiplexed_async_connection().await?;
            let f = UrlFrontier::new(DEFAULT_FRONTIER_KEY);
            let Some((url, score)) = f.dequeue_highest(&mut conn).await? else {
                tracing::warn!("frontier empty");
                return Ok(());
            };
            tracing::info!(%url, score, "dequeued");
            let http = http_client()?;
            let out = ingest_card_url(&pool, &http, &url, default_max_body_bytes()).await?;
            tracing::info!(
                agent_id = %out.agent_id,
                external_id = %out.external_id,
                "ingest ok"
            );
        }
    }
    Ok(())
}
