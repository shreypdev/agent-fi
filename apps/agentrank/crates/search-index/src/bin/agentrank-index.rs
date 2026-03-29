//! Index CLI: rebuild from Postgres or upsert one agent.

use agentrank_data_plane::database_url;
use agentrank_search_index::store::{probe_index_readable, rebuild_index, upsert_agent};
use anyhow::Context;
use clap::{Parser, Subcommand};
use sqlx::postgres::PgPoolOptions;
use std::path::PathBuf;
use tracing_subscriber::EnvFilter;
use uuid::Uuid;

#[derive(Parser)]
#[command(
    name = "agentrank-index",
    about = "AgentRank Tantivy index maintenance"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Exit 0 if the index at `output` is readable (version + Tantivy open); no DB required.
    Probe {
        #[arg(long, env = "SEARCH_INDEX_PATH")]
        output: PathBuf,
    },
    /// Drop (if any) and rebuild index from all agents.
    Rebuild {
        #[arg(long, env = "SEARCH_INDEX_PATH")]
        output: PathBuf,
    },
    /// Re-index a single agent by UUID.
    Upsert {
        #[arg(long, env = "SEARCH_INDEX_PATH")]
        index: PathBuf,
        #[arg(long)]
        agent_id: Uuid,
    },
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
        Command::Probe { output } => {
            probe_index_readable(&output).map_err(|e| anyhow::anyhow!("{e}"))?;
            tracing::info!(path = %output.display(), "probe ok");
        }
        Command::Rebuild { output } => {
            let db = database_url().context("DATABASE_URL")?;
            let pool = PgPoolOptions::new()
                .max_connections(5)
                .connect(&db)
                .await
                .context("connect postgres")?;
            rebuild_index(&pool, &output)
                .await
                .map_err(|e| anyhow::anyhow!("{e}"))?;
            tracing::info!(path = %output.display(), "rebuild complete");
        }
        Command::Upsert { index, agent_id } => {
            let db = database_url().context("DATABASE_URL")?;
            let pool = PgPoolOptions::new()
                .max_connections(5)
                .connect(&db)
                .await
                .context("connect postgres")?;
            upsert_agent(&pool, &index, agent_id)
                .await
                .map_err(|e| anyhow::anyhow!("{e}"))?;
            tracing::info!(%agent_id, path = %index.display(), "upsert complete");
        }
    }
    Ok(())
}
