//! Periodic registry discover (Week 6) — builtin + optional HTTP JSON feeds from env.

use crate::domain_probe;
use crate::http_client;
use crate::registry_sync;
use crate::sitemap_discovery;
use agentrank_frontier::{FrontierMeta, UrlFrontier, DEFAULT_FRONTIER_KEY};
use agentrank_registry_connectors::{
    BuiltinDemoSeed, DiscoveredUrl, HttpJsonUrlFeed, RegistrySource,
};
use metrics::counter;
use redis::aio::MultiplexedConnection;
use reqwest::Client;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

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
        let meta = FrontierMeta {
            discovery_source: d.discovery_source.clone(),
            confidence: d.confidence,
        };
        f.enqueue_with_meta(conn, &d.url, d.priority, &meta).await?;
        n += 1;
        counter!("registry_discovered_urls_total", "source" => source).increment(1);
    }
    counter!("registry_connector_runs_total", "source" => source, "status" => "ok").increment(1);
    Ok(n)
}

async fn tick_once(
    http: &Client,
    conn: &mut MultiplexedConnection,
    pool: Option<&PgPool>,
) -> anyhow::Result<()> {
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

    if let Some(pool) = pool {
        if std::env::var("AGENTBOT_SITEMAP_SCHEDULED")
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(true)
        {
            match sitemap_discovery::scheduled_sitemap_tick(pool, http, conn).await {
                Ok(n) => tracing::info!(n, "scheduled sitemap tick"),
                Err(e) => tracing::warn!("sitemap scheduled: {e}"),
            }
        }
        if std::env::var("AGENTBOT_REGISTRY_PAGINATION")
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(true)
        {
            if let Err(e) = registry_sync::run_paginated_registry_tick(pool, http, conn).await {
                tracing::warn!("registry pagination: {e}");
            }
        }
    }

    if std::env::var("AGENTBOT_PROBE_SCHEDULED")
        .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
        .unwrap_or(false)
    {
        let robots = agentrank_crawl_policy::RobotsCache::new(
            std::time::Duration::from_secs(86_400),
            std::time::Duration::from_secs(259_200),
        );
        let policy = crate::IngestPolicy::from_env();
        match domain_probe::run_probe_tick(http, conn, &robots, policy).await {
            Ok(n) => tracing::info!(n, "probe tick"),
            Err(e) => tracing::warn!("probe tick: {e}"),
        }
    }

    Ok(())
}

/// Spawn background task: every `interval_secs`, run builtin + configured HTTP feeds.
pub fn spawn(redis_url: String, interval_secs: u64, database_url: Option<String>) {
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

        let pool: Option<PgPool> = match &database_url {
            Some(d) if !d.trim().is_empty() => {
                match PgPoolOptions::new().max_connections(3).connect(d).await {
                    Ok(p) => Some(p),
                    Err(e) => {
                        tracing::warn!("discover_scheduler db: {e}");
                        None
                    }
                }
            }
            _ => None,
        };

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
            if let Err(e) = tick_once(&http, &mut conn, pool.as_ref()).await {
                tracing::warn!("discover_scheduler tick: {e}");
            }
        }
    });
}
