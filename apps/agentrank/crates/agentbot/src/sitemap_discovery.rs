//! Fetch `/.well-known/agent-sitemap.xml` for provider domains; enqueue card URLs.

use agentrank_frontier::{FrontierMeta, UrlFrontier, DEFAULT_FRONTIER_KEY};
use agentrank_sitemap::{
    gunzip_if_needed, parse_sitemap_document, record_cards_discovered, record_sitemap_metrics,
    SitemapParseResult,
};
use metrics::counter;
use redis::aio::MultiplexedConnection;
use redis::AsyncCommands;
use reqwest::Client;
use sqlx::PgPool;
use url::Url;

/// Redis marker: `sitemap:{domain}` — set when a post-ingest sitemap fetch was done (24h TTL).
pub fn sitemap_fetched_key(domain: &str) -> String {
    format!("sitemap:{}", domain.trim().to_ascii_lowercase())
}

/// Scheduled tick: all distinct provider domains → fetch sitemap → enqueue (high priority).
pub async fn scheduled_sitemap_tick(
    pool: &PgPool,
    http: &Client,
    redis: &mut MultiplexedConnection,
) -> anyhow::Result<u32> {
    let domains: Vec<String> = sqlx::query_scalar(
        r#"SELECT DISTINCT primary_domain FROM providers WHERE primary_domain <> ''"#,
    )
    .fetch_all(pool)
    .await?;
    let mut total = 0u32;
    for domain in domains {
        let n = fetch_enqueue_sitemap_for_domain(http, redis, &domain, "scheduled", 2.2).await?;
        total += n;
    }
    Ok(total)
}

/// After ingest: if `sitemap:{domain}` missing, fetch sitemap and enqueue peers (excluding `skip_card_url`).
pub async fn maybe_post_ingest_sitemap(
    http: &Client,
    redis: &mut MultiplexedConnection,
    domain: &str,
    skip_card_url: &str,
) -> anyhow::Result<()> {
    let key = sitemap_fetched_key(domain);
    let exists: usize = redis.exists(&key).await.unwrap_or(0);
    if exists > 0 {
        return Ok(());
    }
    let n = fetch_enqueue_sitemap_for_domain_skip(
        http,
        redis,
        domain,
        "post_ingest",
        2.0,
        skip_card_url,
    )
    .await?;
    let _: () = redis.set_ex(&key, "1", 86_400).await?;
    let _ = n;
    Ok(())
}

async fn fetch_enqueue_sitemap_for_domain(
    http: &Client,
    redis: &mut MultiplexedConnection,
    domain: &str,
    phase: &'static str,
    priority: f64,
) -> anyhow::Result<u32> {
    fetch_enqueue_sitemap_for_domain_skip(http, redis, domain, phase, priority, "").await
}

async fn fetch_enqueue_sitemap_for_domain_skip(
    http: &Client,
    redis: &mut MultiplexedConnection,
    domain: &str,
    phase: &'static str,
    priority: f64,
    skip: &str,
) -> anyhow::Result<u32> {
    let base = format!("https://{}", domain.trim().trim_end_matches('/'));
    let sm_url = Url::parse(&format!(
        "{}/.well-known/agent-sitemap.xml",
        base.trim_end_matches('/')
    ))?;
    let resp = http.get(sm_url.clone()).send().await;
    let resp = match resp {
        Ok(r) => r,
        Err(e) => {
            tracing::debug!(%domain, "sitemap fetch: {e}");
            record_sitemap_metrics("error");
            return Ok(0);
        }
    };
    if !resp.status().is_success() {
        record_sitemap_metrics("http_error");
        return Ok(0);
    }
    let bytes = resp.bytes().await?;
    let plain = match gunzip_if_needed(&bytes) {
        Ok(b) => b,
        Err(_) => {
            record_sitemap_metrics("gzip_error");
            return Ok(0);
        }
    };
    let parsed = match parse_sitemap_document(&plain) {
        Ok(p) => p,
        Err(_) => {
            record_sitemap_metrics("parse_error");
            return Ok(0);
        }
    };
    record_sitemap_metrics("ok");

    let mut urls = Vec::new();
    match parsed {
        SitemapParseResult::Index(children) => {
            for child in children {
                if let Ok(r) = http.get(&child).send().await {
                    if r.status().is_success() {
                        if let Ok(b) = r.bytes().await {
                            let p = gunzip_if_needed(&b).unwrap_or(b.to_vec());
                            if let Ok(SitemapParseResult::Urlset(entries)) =
                                parse_sitemap_document(&p)
                            {
                                for e in entries {
                                    urls.push(e.card_url);
                                }
                            }
                        }
                    }
                }
            }
        }
        SitemapParseResult::Urlset(entries) => {
            for e in entries {
                urls.push(e.card_url);
            }
        }
    }

    let frontier = UrlFrontier::new(DEFAULT_FRONTIER_KEY);
    let mut n = 0u32;
    for u in urls {
        if !skip.is_empty() && u == skip {
            continue;
        }
        let meta = FrontierMeta::new(if phase == "scheduled" {
            "sitemap_scheduled"
        } else {
            "sitemap_post_ingest"
        });
        frontier
            .enqueue_with_meta(redis, &u, priority, &meta)
            .await?;
        n += 1;
        counter!("registry_discovered_urls_total", "source" => "sitemap").increment(1);
    }
    record_cards_discovered(u64::from(n), phase);
    Ok(n)
}
