//! Frontier-driven crawl loop: robots.txt, per-host rate limits, ingest, metrics.

use crate::host_rate_limit::check_host_fetch_allowed;
use crate::metrics_srv;
use crate::{
    default_max_body_bytes, ingest_card_url_with_policy, IngestPolicy, AGENTBOT_USER_AGENT,
};
use agentrank_crawl_policy::{refresh_robots_for_url, RobotsCache};
use agentrank_frontier::{UrlFrontier, DEFAULT_FRONTIER_KEY};
use metrics::{counter, gauge, histogram};
use redis::aio::MultiplexedConnection;
use reqwest::Client;
use sqlx::PgPool;
use std::net::SocketAddr;
use std::time::{Duration, Instant};
use url::Url;

/// Runtime knobs for [`run_drain`] / [`run_loop`].
#[derive(Clone, Debug)]
pub struct CrawlRunConfig {
    pub policy: IngestPolicy,
    /// Max successful HTTP fetches per registrable host per second (Redis).
    pub host_max_per_sec: u64,
    pub robots_ttl_ok: Duration,
    pub robots_ttl_negative: Duration,
    pub frontier_key: String,
    /// Optional metrics bind (e.g. `127.0.0.1:9093`). If set, [`run_loop`] serves `/metrics`.
    pub metrics_bind: Option<SocketAddr>,
}

impl CrawlRunConfig {
    pub fn from_env() -> Self {
        let host_max_per_sec = std::env::var("AGENTBOT_HOST_MAX_PER_SEC")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(2);

        let robots_ttl_ok = std::env::var("AGENTBOT_ROBOTS_TTL_OK_SECS")
            .ok()
            .and_then(|s| s.parse().ok())
            .map(Duration::from_secs)
            .unwrap_or(Duration::from_secs(86_400));

        let robots_ttl_neg = std::env::var("AGENTBOT_ROBOTS_TTL_NEGATIVE_SECS")
            .ok()
            .and_then(|s| s.parse().ok())
            .map(Duration::from_secs)
            .unwrap_or(Duration::from_secs(259_200));

        let metrics_bind = std::env::var("AGENTBOT_METRICS_BIND")
            .ok()
            .and_then(|s| s.parse().ok());

        Self {
            policy: IngestPolicy::from_env(),
            host_max_per_sec,
            robots_ttl_ok,
            robots_ttl_negative: robots_ttl_neg,
            frontier_key: DEFAULT_FRONTIER_KEY.to_string(),
            metrics_bind,
        }
    }
}

async fn record_robots_denied(pool: &PgPool, url: &str, detail: &str) {
    let _ = sqlx::query(
        r#"
        INSERT INTO crawl_history (url, agent_id, http_status, error_code, error_detail, response_bytes)
        VALUES ($1, NULL, NULL, 'robots_denied', $2, 0)
        "#,
    )
    .bind(url)
    .bind(detail)
    .execute(pool)
    .await;
}

#[allow(clippy::too_many_arguments)]
async fn process_one_url(
    pool: &PgPool,
    client: &Client,
    redis: &mut MultiplexedConnection,
    frontier: &UrlFrontier,
    robots_cache: &RobotsCache,
    cfg: &CrawlRunConfig,
    url: String,
    score: f64,
) -> anyhow::Result<()> {
    let parsed = match Url::parse(&url) {
        Ok(u) => u,
        Err(e) => {
            tracing::warn!(%url, "skip bad URL: {e}");
            counter!("agentbot_ingest_total", "result" => "bad_url").increment(1);
            return Ok(());
        }
    };

    if let Err(e) = agentrank_crawl_policy::validate_outbound_url(
        &parsed,
        cfg.policy.allow_http_localhost,
        cfg.policy.allow_loopback_https,
    ) {
        tracing::warn!(%url, "url policy: {e}");
        counter!("agentbot_ingest_total", "result" => "url_policy").increment(1);
        return Ok(());
    }

    let robots = match refresh_robots_for_url(
        client,
        robots_cache,
        &parsed,
        cfg.policy.allow_http_localhost,
        cfg.policy.allow_loopback_https,
    )
    .await
    {
        Ok(r) => r,
        Err(e) => {
            tracing::warn!(%url, "robots fetch: {e}");
            counter!("agentbot_robots_fetch_errors_total").increment(1);
            return Ok(());
        }
    };

    let path = parsed.path();
    let path = if path.is_empty() { "/" } else { path };
    if !robots.is_allowed(AGENTBOT_USER_AGENT, path) {
        tracing::info!(%url, "robots.txt disallows path");
        record_robots_denied(pool, &url, "path disallowed by robots.txt").await;
        counter!("agentbot_ingest_total", "result" => "robots_denied").increment(1);
        counter!("agentbot_robots_denied_total").increment(1);
        return Ok(());
    }

    let host_key = parsed
        .host_str()
        .map(|h| h.to_ascii_lowercase())
        .unwrap_or_else(|| "unknown".into());

    if !check_host_fetch_allowed(redis, &host_key, cfg.host_max_per_sec).await? {
        tracing::debug!(%url, "host rate limited, re-enqueue");
        frontier
            .enqueue(redis, &url, score * 0.5)
            .await
            .map_err(|e| anyhow::anyhow!("re-enqueue: {e}"))?;
        counter!("agentbot_ingest_total", "result" => "rate_limited").increment(1);
        tokio::time::sleep(Duration::from_millis(200)).await;
        return Ok(());
    }

    if let Some(delay) = robots.crawl_delay_secs(AGENTBOT_USER_AGENT) {
        let ms = (delay * 1000.0) as u64;
        if ms > 0 && ms < 60_000 {
            tokio::time::sleep(Duration::from_millis(ms.min(10_000))).await;
        }
    }

    let t0 = Instant::now();
    match ingest_card_url_with_policy(pool, client, &url, default_max_body_bytes(), cfg.policy)
        .await
    {
        Ok(out) => {
            tracing::info!(agent_id = %out.agent_id, %url, "ingest ok");
            counter!("agentbot_ingest_total", "result" => "ok").increment(1);
        }
        Err(e) => {
            tracing::warn!(%url, "ingest err: {e}");
            counter!("agentbot_ingest_total", "result" => "error").increment(1);
        }
    }
    histogram!("agentbot_ingest_duration_seconds").record(t0.elapsed().as_secs_f64());

    Ok(())
}

/// Pop up to `max` URLs from the frontier and process them.
pub async fn run_drain(
    pool: &PgPool,
    client: &Client,
    redis: &mut MultiplexedConnection,
    cfg: &CrawlRunConfig,
    max: u32,
) -> anyhow::Result<u32> {
    let _ = metrics_srv::init_metrics();
    let robots_cache = RobotsCache::new(cfg.robots_ttl_ok, cfg.robots_ttl_negative);
    let frontier = UrlFrontier::new(cfg.frontier_key.clone());
    let mut done = 0u32;
    for _ in 0..max {
        let Some((url, score)) = frontier.dequeue_highest(redis).await? else {
            break;
        };
        process_one_url(
            pool,
            client,
            redis,
            &frontier,
            &robots_cache,
            cfg,
            url,
            score,
        )
        .await?;
        done += 1;
        if let Ok(n) = frontier.len(redis).await {
            gauge!("agentrank_frontier_depth").set(n as f64);
        }
    }
    Ok(done)
}

/// Run until SIGINT/SIGTERM (Unix), processing the frontier continuously.
#[cfg(unix)]
pub async fn run_loop(
    pool: PgPool,
    client: Client,
    mut redis: MultiplexedConnection,
    cfg: CrawlRunConfig,
) -> anyhow::Result<()> {
    use tokio::signal::unix::{signal, SignalKind};

    metrics_srv::init_metrics()?;
    if let Some(addr) = cfg.metrics_bind {
        tokio::spawn(async move {
            if let Err(e) = metrics_srv::serve_metrics(addr).await {
                tracing::error!("metrics server: {e}");
            }
        });
    }

    let robots_cache = RobotsCache::new(cfg.robots_ttl_ok, cfg.robots_ttl_negative);
    let frontier = UrlFrontier::new(cfg.frontier_key.clone());
    let cfg = std::sync::Arc::new(cfg);

    let mut sigint = signal(SignalKind::interrupt()).map_err(|e| anyhow::anyhow!("sigint: {e}"))?;
    let mut sigterm =
        signal(SignalKind::terminate()).map_err(|e| anyhow::anyhow!("sigterm: {e}"))?;

    loop {
        tokio::select! {
            _ = sigint.recv() => {
                tracing::info!("shutdown (SIGINT)");
                break;
            }
            _ = sigterm.recv() => {
                tracing::info!("shutdown (SIGTERM)");
                break;
            }
            r = async {
                if let Some((url, score)) = frontier.dequeue_highest(&mut redis).await? {
                    process_one_url(
                        &pool,
                        &client,
                        &mut redis,
                        &frontier,
                        &robots_cache,
                        cfg.as_ref(),
                        url,
                        score,
                    )
                    .await?;
                } else {
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
                if let Ok(n) = frontier.len(&mut redis).await {
                    gauge!("agentrank_frontier_depth").set(n as f64);
                }
                Ok::<(), anyhow::Error>(())
            } => {
                r?;
            }
        }
    }

    Ok(())
}

#[cfg(not(unix))]
pub async fn run_loop(
    _pool: PgPool,
    _client: Client,
    _redis: MultiplexedConnection,
    _cfg: CrawlRunConfig,
) -> anyhow::Result<()> {
    Err(anyhow::anyhow!(
        "agentbot run-loop is only supported on Unix; use `agentbot drain`"
    ))
}
