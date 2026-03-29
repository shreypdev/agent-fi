//! Periodic registry discover (Week 6) — builtin + optional HTTP JSON feeds from env.

use crate::http_client;
use agentrank_frontier::{UrlFrontier, DEFAULT_FRONTIER_KEY};
use agentrank_registry_connectors::{
    BuiltinDemoSeed, DiscoveredUrl, HttpJsonUrlFeed, RegistrySource,
};
use metrics::counter;
use redis::aio::MultiplexedConnection;
use reqwest::Client;

fn env_http_feeds() -> Vec<String> {
    std::env::var("AGENTBOT_DISCOVER_HTTP_JSONS")
        .unwrap_or_default()
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect()
}

async fn enqueue_discovered(
    conn: &mut MultiplexedConnection,
    source: &'static str,
    items: Vec<DiscoveredUrl>,
) -> anyhow::Result<u32> {
    let f = UrlFrontier::new(DEFAULT_FRONTIER_KEY);
    let mut n = 0u32;
    for d in items {
        f.enqueue(conn, &d.url, d.priority).await?;
        n += 1;
        counter!("registry_discovered_urls_total", "source" => source).increment(1);
    }
    counter!("registry_connector_runs_total", "source" => source, "status" => "ok").increment(1);
    Ok(n)
}

async fn tick_once(http: &Client, conn: &mut MultiplexedConnection) -> anyhow::Result<()> {
    let src = BuiltinDemoSeed;
    let items = src.discover(http).await?;
    let n = enqueue_discovered(conn, src.source_name(), items).await?;
    tracing::info!(n, source = src.source_name(), "scheduled discover enqueued");

    for url in env_http_feeds() {
        let src = HttpJsonUrlFeed {
            feed_url: url.clone(),
            default_priority: 1.0,
        };
        match src.discover(http).await {
            Ok(items) => {
                let n = enqueue_discovered(conn, src.source_name(), items).await?;
                tracing::info!(n, %url, "scheduled http_json discover enqueued");
            }
            Err(e) => tracing::warn!(%url, "scheduled http_json discover: {e}"),
        }
    }

    for (env_key, source_name, pri) in [
        ("PULSEMCP_FEED_URL", "pulsemcp_http", 0.9_f64),
        ("MCPSO_FEED_URL", "mcpso_http", 0.85_f64),
    ] {
        if let Ok(url) = std::env::var(env_key) {
            let url = url.trim().to_string();
            if url.is_empty() {
                continue;
            }
            let src = HttpJsonUrlFeed {
                feed_url: url.clone(),
                default_priority: pri,
            };
            match src.discover(http).await {
                Ok(items) => {
                    let n = enqueue_discovered(conn, source_name, items).await?;
                    tracing::info!(n, %url, source = source_name, "scheduled registry feed enqueued");
                }
                Err(e) => tracing::warn!(%url, source = source_name, "discover: {e}"),
            }
        }
    }
    Ok(())
}

/// Spawn background task: every `interval_secs`, run builtin + configured HTTP feeds.
pub fn spawn(redis_url: String, interval_secs: u64) {
    if interval_secs == 0 {
        return;
    }
    tokio::spawn(async move {
        let http = match http_client() {
            Ok(c) => c,
            Err(e) => {
                tracing::error!("discover_scheduler: http client: {e}");
                return;
            }
        };
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(interval_secs));
        interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
        loop {
            interval.tick().await;
            let client = match redis::Client::open(redis_url.as_str()) {
                Ok(c) => c,
                Err(e) => {
                    tracing::warn!("discover_scheduler redis: {e}");
                    continue;
                }
            };
            let mut conn = match client.get_multiplexed_async_connection().await {
                Ok(c) => c,
                Err(e) => {
                    tracing::warn!("discover_scheduler redis conn: {e}");
                    continue;
                }
            };
            if let Err(e) = tick_once(&http, &mut conn).await {
                tracing::warn!("discover_scheduler tick: {e}");
            }
        }
    });
}
