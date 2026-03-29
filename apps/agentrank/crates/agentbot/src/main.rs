//! AgentBot CLI: ingest, enqueue, frontier drain / loop, discovery (Week 5).

use agentrank_agentbot::{
    default_max_body_bytes, github_discover, http_client, index_pipeline,
    ingest_card_url_with_policy, run_drain, run_loop, CrawlRunConfig, IngestPolicy,
};
use agentrank_data_plane::{database_url, redis_url};
use agentrank_frontier::{UrlFrontier, DEFAULT_FRONTIER_KEY};
use agentrank_registry_connectors::{
    BuiltinDemoSeed, DiscoveredUrl, HttpJsonUrlFeed, RegistrySource, StaticJsonFile,
};
use clap::{Parser, Subcommand};
use metrics::counter;
use redis::aio::MultiplexedConnection;
use sqlx::postgres::PgPoolOptions;
use std::path::PathBuf;
use tracing_subscriber::EnvFilter;

#[derive(Parser)]
#[command(name = "agentbot", about = "AgentRank AgentBot v0.2")]
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
    /// Pop the highest-priority URL from the frontier and ingest it (robots + URL policy + rate limit).
    RunOnce,
    /// Pop up to N URLs from the frontier and process each (bounded batch / CI).
    Drain {
        #[arg(short, long, default_value_t = 100)]
        max: u32,
    },
    /// Run until SIGINT/SIGTERM, continuously dequeuing (Unix). Set `AGENTBOT_METRICS_BIND` for `/metrics`.
    RunLoop,
    /// Discover card URLs from registry connectors or GitHub, enqueue to frontier.
    Discover {
        #[command(subcommand)]
        sub: DiscoverCmd,
    },
    /// Re-upsert all agents into Tantivy + Qdrant (repair / backfill; uses SEARCH_INDEX_PATH + QDRANT_URL).
    IndexBackfill,
}

#[derive(Subcommand)]
enum DiscoverCmd {
    /// Built-in demo seed URLs (no network).
    Builtin,
    /// Fetch JSON `{ "urls": [...] }` or `[...]` from an HTTP URL.
    HttpJson {
        url: String,
        #[arg(short, long, default_value_t = 0.0)]
        priority: f64,
    },
    /// Read the same JSON formats from a local file.
    File {
        path: PathBuf,
        #[arg(short, long, default_value_t = 0.0)]
        priority: f64,
    },
    /// GitHub code search (`GITHUB_TOKEN` required). Enqueues raw.githubusercontent.com guesses for main/master.
    Github {
        /// Code search query, e.g. `filename:agent.json path:.well-known`
        query: String,
        #[arg(short, long, default_value_t = 10)]
        max: u32,
        #[arg(long, default_value_t = 5.0)]
        priority: f64,
    },
}

async fn enqueue_discovered(
    conn: &mut MultiplexedConnection,
    source: &'static str,
    items: Vec<DiscoveredUrl>,
) -> anyhow::Result<u32> {
    let f = UrlFrontier::new(DEFAULT_FRONTIER_KEY);
    let mut n = 0u32;
    for d in items {
        let r = f.enqueue(conn, &d.url, d.priority).await?;
        n += 1;
        tracing::debug!(?r, url = %d.url, "enqueued from {}", source);
        match r {
            agentrank_frontier::EnqueueResult::Inserted => {
                counter!("frontier_enqueue_total", "result" => "inserted", "source" => source)
                    .increment(1);
            }
            agentrank_frontier::EnqueueResult::ScoreUpdated => {
                counter!("frontier_enqueue_total", "result" => "updated", "source" => source)
                    .increment(1);
            }
        }
    }
    counter!("registry_discovered_urls_total", "source" => source).increment(u64::from(n));
    Ok(n)
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
            let policy = IngestPolicy::from_env();
            let db = database_url().map_err(|e| anyhow::anyhow!("DATABASE_URL: {e}"))?;
            let pool = PgPoolOptions::new().max_connections(5).connect(&db).await?;
            let client = http_client()?;
            let out =
                ingest_card_url_with_policy(&pool, &client, &url, default_max_body_bytes(), policy)
                    .await?;
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
            let http = http_client()?;
            let mut cfg = CrawlRunConfig::from_env();
            cfg.metrics_bind = None;
            let n = run_drain(&pool, &http, &mut conn, &cfg, 1).await?;
            if n == 0 {
                tracing::warn!("frontier empty");
            }
        }
        Command::Drain { max } => {
            let db = database_url().map_err(|e| anyhow::anyhow!("DATABASE_URL: {e}"))?;
            let ru = redis_url().map_err(|e| anyhow::anyhow!("REDIS_URL: {e}"))?;
            let pool = PgPoolOptions::new().max_connections(5).connect(&db).await?;
            let client = redis::Client::open(ru.as_str())?;
            let mut conn = client.get_multiplexed_async_connection().await?;
            let http = http_client()?;
            let cfg = CrawlRunConfig::from_env();
            let n = run_drain(&pool, &http, &mut conn, &cfg, max).await?;
            tracing::info!(n, "drain complete");
        }
        Command::RunLoop => {
            let db = database_url().map_err(|e| anyhow::anyhow!("DATABASE_URL: {e}"))?;
            let ru = redis_url().map_err(|e| anyhow::anyhow!("REDIS_URL: {e}"))?;
            let pool = PgPoolOptions::new().max_connections(5).connect(&db).await?;
            let client = redis::Client::open(ru.as_str())?;
            let conn = client.get_multiplexed_async_connection().await?;
            let http = http_client()?;
            let cfg = CrawlRunConfig::from_env();
            run_loop(pool, http, conn, cfg).await?;
        }
        Command::Discover { sub } => {
            let _ = agentrank_agentbot::metrics_srv::init_metrics();
            let ru = redis_url().map_err(|e| anyhow::anyhow!("REDIS_URL: {e}"))?;
            let client_redis = redis::Client::open(ru.as_str())?;
            let mut conn = client_redis.get_multiplexed_async_connection().await?;
            let http = http_client()?;
            match sub {
                DiscoverCmd::Builtin => {
                    let src = BuiltinDemoSeed;
                    let items = src.discover(&http).await?;
                    let n = enqueue_discovered(&mut conn, src.source_name(), items).await?;
                    tracing::info!(n, source = src.source_name(), "discover enqueued");
                    counter!(
                        "registry_connector_runs_total",
                        "source" => src.source_name(),
                        "status" => "ok"
                    )
                    .increment(1);
                }
                DiscoverCmd::HttpJson { url, priority } => {
                    let src = HttpJsonUrlFeed {
                        feed_url: url.clone(),
                        default_priority: priority,
                    };
                    let items = src.discover(&http).await?;
                    let n = enqueue_discovered(&mut conn, src.source_name(), items).await?;
                    tracing::info!(n, source = src.source_name(), "discover enqueued");
                    counter!(
                        "registry_connector_runs_total",
                        "source" => src.source_name(),
                        "status" => "ok"
                    )
                    .increment(1);
                }
                DiscoverCmd::File { path, priority } => {
                    let src = StaticJsonFile {
                        path,
                        default_priority: priority,
                    };
                    let items = src.discover(&http).await?;
                    let n = enqueue_discovered(&mut conn, src.source_name(), items).await?;
                    tracing::info!(n, source = src.source_name(), "discover enqueued");
                    counter!(
                        "registry_connector_runs_total",
                        "source" => src.source_name(),
                        "status" => "ok"
                    )
                    .increment(1);
                }
                DiscoverCmd::Github {
                    query,
                    max,
                    priority,
                } => {
                    let urls =
                        github_discover::discover_github_card_urls(&http, &query, max).await?;
                    let items: Vec<DiscoveredUrl> = urls
                        .into_iter()
                        .map(|url| DiscoveredUrl { url, priority })
                        .collect();
                    let n = enqueue_discovered(&mut conn, "github_code_search", items).await?;
                    tracing::info!(n, "github discover enqueued");
                    counter!(
                        "registry_connector_runs_total",
                        "source" => "github_code_search",
                        "status" => "ok"
                    )
                    .increment(1);
                }
            }
        }
        Command::IndexBackfill => {
            let db = database_url().map_err(|e| anyhow::anyhow!("DATABASE_URL: {e}"))?;
            let pool = PgPoolOptions::new().max_connections(5).connect(&db).await?;
            let index_path = std::env::var("SEARCH_INDEX_PATH").ok().map(PathBuf::from);
            let qdrant = match std::env::var("QDRANT_URL") {
                Ok(ref u) if !u.trim().is_empty() => {
                    let c = agentrank_vector::connect().await?;
                    agentrank_vector::ensure_agents_collection(&c, index_pipeline::embedding_dim())
                        .await?;
                    Some(std::sync::Arc::new(c))
                }
                _ => None,
            };
            let ids: Vec<uuid::Uuid> =
                sqlx::query_scalar("SELECT id FROM agents ORDER BY created_at ASC")
                    .fetch_all(&pool)
                    .await?;
            for id in &ids {
                index_pipeline::index_agent_after_ingest(
                    &pool,
                    *id,
                    index_path.as_deref(),
                    qdrant.as_ref(),
                )
                .await;
            }
            tracing::info!(n = ids.len(), "index-backfill complete");
        }
    }
    Ok(())
}
