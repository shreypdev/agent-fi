//! Domain probing: HEAD then GET with byte cap (Open-web crawler §B).

use agentrank_crawl_policy::{
    merged_robots_allow, refresh_agent_robots_for_url, refresh_robots_for_url, RobotsCache,
};
use agentrank_frontier::{FrontierMeta, UrlFrontier, DEFAULT_FRONTIER_KEY};
use metrics::{counter, histogram};
use redis::aio::MultiplexedConnection;
use redis::AsyncCommands;
use reqwest::header::CONTENT_TYPE;
use reqwest::Client;
use std::time::Instant;
use tokio::net::lookup_host;
use url::Url;

use crate::IngestPolicy;
use crate::AGENTBOT_USER_AGENT;

const PROBE_PATHS: &[&str] = &[
    "/.well-known/agent.json",
    "/.well-known/agent-sitemap.xml",
    "/agent.json",
    "/api/agent-card",
];

const SUB_PREFIXES: &[&str] = &["agents", "agent", "api", "a2a", "mcp"];

pub fn probe_byte_cap() -> usize {
    std::env::var("AGENTBOT_PROBE_MAX_BODY_BYTES")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(32_768)
}

fn probe_dedup_key(host: &str, path: &str) -> String {
    format!("probe:seen:{host}:{path}")
}

/// HEAD (5s) then GET with byte cap when inconclusive.
pub async fn probe_well_known_urls(
    client: &Client,
    robots_cache: &RobotsCache,
    policy: IngestPolicy,
    host: &str,
) -> anyhow::Result<Vec<String>> {
    let host = host.trim().trim_end_matches('.');
    let mut hosts = vec![host.to_string()];
    for p in SUB_PREFIXES {
        let h = format!("{p}.{host}");
        let addr = format!("{h}:0");
        if lookup_host(&addr).await.is_ok() {
            hosts.push(h);
        }
    }

    let mut found = Vec::new();
    for h in hosts {
        let base = format!("https://{h}");
        let Ok(origin) = Url::parse(&base) else {
            continue;
        };
        let site = refresh_robots_for_url(
            client,
            robots_cache,
            &origin,
            policy.allow_http_localhost,
            policy.allow_loopback_https,
        )
        .await?;
        let agent = refresh_agent_robots_for_url(
            client,
            robots_cache,
            &origin,
            policy.allow_http_localhost,
            policy.allow_loopback_https,
        )
        .await?;

        for path in PROBE_PATHS {
            let url_s = format!("https://{h}{path}");
            let Ok(u) = Url::parse(&url_s) else {
                continue;
            };
            let pth = u.path();
            let pth = if pth.is_empty() { "/" } else { pth };
            if !merged_robots_allow(&site, &agent, AGENTBOT_USER_AGENT, pth) {
                counter!("agentrank_domain_probe_total", "result" => "miss").increment(1);
                continue;
            }

            let t0 = Instant::now();
            let head = client
                .head(&url_s)
                .timeout(std::time::Duration::from_secs(5))
                .send()
                .await;

            let mut resolved = false;
            if let Ok(resp) = &head {
                if resp.status().is_success() {
                    let ct = resp
                        .headers()
                        .get(CONTENT_TYPE)
                        .and_then(|v| v.to_str().ok())
                        .unwrap_or("");
                    if ct.contains("json") {
                        found.push(url_s.clone());
                        resolved = true;
                        counter!("agentrank_domain_probe_total", "result" => "found").increment(1);
                    }
                }
            }

            if !resolved {
                let cap = probe_byte_cap();
                let get = client
                    .get(&url_s)
                    .timeout(std::time::Duration::from_secs(15))
                    .send()
                    .await;
                if let Ok(resp) = get {
                    let len = resp.content_length().unwrap_or(0);
                    counter!("agentrank_probe_bytes_total").increment(len);
                    let b = resp.bytes().await.unwrap_or_default();
                    let slice = if b.len() > cap { &b[..cap] } else { &b[..] };
                    if slice.len() >= 2 && (slice[0] == b'{' || slice[0] == b'[') {
                        found.push(url_s);
                        counter!("agentrank_domain_probe_total", "result" => "found").increment(1);
                    } else {
                        counter!("agentrank_domain_probe_total", "result" => "miss").increment(1);
                    }
                } else {
                    counter!("agentrank_domain_probe_total", "result" => "error").increment(1);
                }
            }
            histogram!("agentrank_domain_probe_latency_seconds").record(t0.elapsed().as_secs_f64());
        }
    }
    Ok(found)
}

/// Enqueue probe results with Redis dedup (30d TTL).
pub async fn enqueue_probe_urls(
    redis: &mut MultiplexedConnection,
    urls: &[String],
) -> anyhow::Result<u32> {
    let f = UrlFrontier::new(DEFAULT_FRONTIER_KEY);
    let mut n = 0u32;
    for u in urls {
        let Ok(parsed) = Url::parse(u) else {
            continue;
        };
        let host = parsed.host_str().unwrap_or("").to_ascii_lowercase();
        let path = parsed.path();
        let key = probe_dedup_key(&host, path);
        // On Redis errors, do not skip enqueue (fail open for discovery).
        let exists: usize = redis.exists(&key).await.unwrap_or(0);
        if exists > 0 {
            continue;
        }
        let _: () = redis.set_ex(&key, "1", 2_592_000).await.unwrap_or(());
        let meta = FrontierMeta::new("domain_probe");
        f.enqueue_with_meta(redis, u, 1.1, &meta).await?;
        n += 1;
    }
    Ok(n)
}

pub fn seed_domains_from_env() -> Vec<String> {
    std::env::var("AGENTBOT_PROBE_SEED_DOMAINS")
        .unwrap_or_default()
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(String::from)
        .collect()
}

pub async fn run_probe_tick(
    client: &Client,
    redis: &mut MultiplexedConnection,
    robots_cache: &RobotsCache,
    policy: IngestPolicy,
) -> anyhow::Result<u32> {
    let mut total = 0u32;
    for host in seed_domains_from_env() {
        let urls = probe_well_known_urls(client, robots_cache, policy, &host).await?;
        total += enqueue_probe_urls(redis, &urls).await?;
    }
    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn probe_dedup_key_is_stable() {
        let k = probe_dedup_key("example.com", "/.well-known/agent.json");
        assert!(k.starts_with("probe:seen:"));
        assert!(k.contains("example.com"));
    }

    #[test]
    fn probe_byte_cap_default() {
        std::env::remove_var("AGENTBOT_PROBE_MAX_BODY_BYTES");
        assert_eq!(probe_byte_cap(), 32_768);
    }
}
