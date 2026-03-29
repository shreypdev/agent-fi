//! In-memory cache for fetched `robots.txt` bodies with positive / negative TTLs.

use crate::parse::{parse_robots_txt, ParsedRobots};
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Cached robots evaluation for one origin (`https://example.com` origin = `https://example.com`).
#[derive(Debug, Clone)]
pub enum CachedRobots {
    /// Parsed rules from a 200 response.
    Rules {
        rules: ParsedRobots,
        fetched_at: Instant,
    },
    /// No robots file (404, etc.): treat as allow-all until TTL expires.
    Missing { fetched_at: Instant },
}

/// Thread-safe in-memory cache (one process). For multi-replica deployments each instance
/// maintains its own cache; TTLs limit staleness.
#[derive(Debug)]
pub struct RobotsCache {
    ttl_ok: Duration,
    ttl_negative: Duration,
    inner: std::sync::RwLock<HashMap<String, CachedRobots>>,
}

impl RobotsCache {
    pub fn new(ttl_ok: Duration, ttl_negative: Duration) -> Self {
        Self {
            ttl_ok,
            ttl_negative,
            inner: std::sync::RwLock::new(HashMap::new()),
        }
    }

    /// Returns `None` if entry absent or expired.
    pub fn get(&self, origin_key: &str) -> Option<CachedRobots> {
        let map = self.inner.read().ok()?;
        let e = map.get(origin_key)?;
        let ttl = match e {
            CachedRobots::Rules { fetched_at, .. } => {
                if fetched_at.elapsed() > self.ttl_ok {
                    return None;
                }
                self.ttl_ok
            }
            CachedRobots::Missing { fetched_at } => {
                if fetched_at.elapsed() > self.ttl_negative {
                    return None;
                }
                self.ttl_negative
            }
        };
        let _ = ttl;
        Some(e.clone())
    }

    pub fn insert_rules(&self, origin_key: String, body: &str) {
        let rules = parse_robots_txt(body);
        if let Ok(mut map) = self.inner.write() {
            map.insert(
                origin_key,
                CachedRobots::Rules {
                    rules,
                    fetched_at: Instant::now(),
                },
            );
        }
    }

    pub fn insert_missing(&self, origin_key: String) {
        if let Ok(mut map) = self.inner.write() {
            map.insert(
                origin_key,
                CachedRobots::Missing {
                    fetched_at: Instant::now(),
                },
            );
        }
    }

    pub fn clear(&self) {
        if let Ok(mut map) = self.inner.write() {
            map.clear();
        }
    }
}

impl CachedRobots {
    pub fn is_allowed(&self, user_agent: &str, path: &str) -> bool {
        match self {
            CachedRobots::Rules { rules, .. } => rules.is_allowed(user_agent, path),
            CachedRobots::Missing { .. } => true,
        }
    }

    pub fn crawl_delay_secs(&self, user_agent: &str) -> Option<f64> {
        match self {
            CachedRobots::Rules { rules, .. } => rules.crawl_delay_secs(user_agent),
            CachedRobots::Missing { .. } => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn second_get_before_ttl_returns_cached() {
        let c = RobotsCache::new(Duration::from_secs(3600), Duration::from_secs(3600));
        c.insert_rules("https://ex.com".into(), "User-agent: *\nDisallow: /admin\n");
        let g = c.get("https://ex.com").expect("cached");
        assert!(g.is_allowed("Bot", "/public"));
        assert!(!g.is_allowed("Bot", "/admin"));
    }
}
