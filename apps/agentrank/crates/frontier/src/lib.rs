//! Priority URL frontier backed by a Redis sorted set (`ZSET`).

use redis::aio::MultiplexedConnection;
use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Default Redis key for the v0 frontier (single queue).
pub const DEFAULT_FRONTIER_KEY: &str = "agentrank:frontier:v0";

/// Provenance for a discovery enqueue (Week 8 §D).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FrontierMeta {
    /// Stable source label: `sitemap_scheduled`, `domain_probe`, `pulsemcp_registry`, …
    pub discovery_source: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f64>,
}

impl FrontierMeta {
    pub fn new(discovery_source: impl Into<String>) -> Self {
        Self {
            discovery_source: discovery_source.into(),
            confidence: None,
        }
    }

    pub fn with_confidence(mut self, c: f64) -> Self {
        self.confidence = Some(c);
        self
    }
}

/// Normalize URL for ZSET member and meta hash field (trim; single representation).
#[must_use]
pub fn normalize_frontier_url(url: &str) -> String {
    url.trim().to_string()
}

/// Result of [`UrlFrontier::enqueue`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnqueueResult {
    /// URL was new; inserted with the given score.
    Inserted,
    /// URL already existed; score was updated to the new value.
    ScoreUpdated,
}

/// Redis-backed priority frontier: higher score dequeues first.
pub struct UrlFrontier {
    key: String,
}

impl UrlFrontier {
    pub fn new(key: impl Into<String>) -> Self {
        Self { key: key.into() }
    }

    /// Redis hash key holding JSON [`FrontierMeta`] per frontier URL.
    pub fn meta_key(&self) -> String {
        format!("{}:meta", self.key)
    }

    /// Enqueue or update priority. Same URL is a single member (dedup). No provenance metadata.
    pub async fn enqueue(
        &self,
        conn: &mut MultiplexedConnection,
        url: &str,
        priority_score: f64,
    ) -> Result<EnqueueResult, FrontierError> {
        let u = normalize_frontier_url(url);
        let added: i32 = conn.zadd(&self.key, &u, priority_score).await?;
        Ok(if added > 0 {
            EnqueueResult::Inserted
        } else {
            EnqueueResult::ScoreUpdated
        })
    }

    /// Enqueue or update priority and attach [`FrontierMeta`] (HSET).
    pub async fn enqueue_with_meta(
        &self,
        conn: &mut MultiplexedConnection,
        url: &str,
        priority_score: f64,
        meta: &FrontierMeta,
    ) -> Result<EnqueueResult, FrontierError> {
        let u = normalize_frontier_url(url);
        let added: i32 = conn.zadd(&self.key, &u, priority_score).await?;
        let meta_json = serde_json::to_string(meta)?;
        let mk = self.meta_key();
        let _: () = conn.hset(&mk, &u, meta_json).await?;
        Ok(if added > 0 {
            EnqueueResult::Inserted
        } else {
            EnqueueResult::ScoreUpdated
        })
    }

    /// Read metadata for a URL (if any) without dequeuing.
    pub async fn get_meta(
        &self,
        conn: &mut MultiplexedConnection,
        url: &str,
    ) -> Result<Option<FrontierMeta>, FrontierError> {
        let u = normalize_frontier_url(url);
        let mk = self.meta_key();
        let raw: Option<String> = conn.hget(&mk, &u).await?;
        let Some(s) = raw else {
            return Ok(None);
        };
        Ok(serde_json::from_str(&s).ok())
    }

    /// Remove and return the URL with the highest score, if any. Provenance metadata is removed.
    pub async fn dequeue_highest(
        &self,
        conn: &mut MultiplexedConnection,
    ) -> Result<Option<(String, f64)>, FrontierError> {
        let popped: Option<Vec<(String, f64)>> = conn.zpopmax(&self.key, 1).await?;
        let Some(mut v) = popped else {
            return Ok(None);
        };
        let Some((url, score)) = v.pop() else {
            return Ok(None);
        };
        let mk = self.meta_key();
        let _: i32 = conn.hdel(&mk, &url).await?;
        Ok(Some((url, score)))
    }

    /// Like [`Self::dequeue_highest`] but returns attached [`FrontierMeta`] if present.
    pub async fn dequeue_highest_with_meta(
        &self,
        conn: &mut MultiplexedConnection,
    ) -> Result<Option<(String, f64, Option<FrontierMeta>)>, FrontierError> {
        let popped: Option<Vec<(String, f64)>> = conn.zpopmax(&self.key, 1).await?;
        let Some(mut v) = popped else {
            return Ok(None);
        };
        let Some((url, score)) = v.pop() else {
            return Ok(None);
        };
        let mk = self.meta_key();
        let raw: Option<String> = conn.hget(&mk, &url).await?;
        let meta = raw.and_then(|s| serde_json::from_str(&s).ok());
        let _: i32 = conn.hdel(&mk, &url).await?;
        Ok(Some((url, score, meta)))
    }

    /// Current number of URLs in the frontier.
    pub async fn len(&self, conn: &mut MultiplexedConnection) -> Result<u64, FrontierError> {
        let n: u64 = conn.zcard(&self.key).await?;
        Ok(n)
    }

    /// Remove all members (test / admin).
    pub async fn clear(&self, conn: &mut MultiplexedConnection) -> Result<(), FrontierError> {
        let _: () = conn.del(&self.key).await?;
        let _: () = conn.del(self.meta_key()).await?;
        Ok(())
    }
}

/// Errors from frontier operations.
#[derive(Debug, Error)]
pub enum FrontierError {
    #[error("redis: {0}")]
    Redis(#[from] redis::RedisError),
    #[error("json: {0}")]
    Json(#[from] serde_json::Error),
}

#[cfg(test)]
mod tests {
    use super::*;
    use redis::Client;

    fn redis_url() -> Option<String> {
        std::env::var("REDIS_URL").ok()
    }

    #[tokio::test]
    async fn enqueue_dedup_updates_score() {
        let Some(url) = redis_url() else {
            return;
        };
        let client = Client::open(url.as_str()).unwrap();
        let mut conn = client.get_multiplexed_async_connection().await.unwrap();
        let f = UrlFrontier::new(format!("{DEFAULT_FRONTIER_KEY}:test_dedup"));
        f.clear(&mut conn).await.unwrap();

        assert_eq!(
            f.enqueue(&mut conn, "https://a.com/x", 1.0).await.unwrap(),
            EnqueueResult::Inserted
        );
        assert_eq!(
            f.enqueue(&mut conn, "https://a.com/x", 5.0).await.unwrap(),
            EnqueueResult::ScoreUpdated
        );
        assert_eq!(f.len(&mut conn).await.unwrap(), 1);
        let (u, s) = f.dequeue_highest(&mut conn).await.unwrap().unwrap();
        assert_eq!(u, "https://a.com/x");
        assert!((s - 5.0).abs() < 1e-9);

        f.clear(&mut conn).await.unwrap();
    }

    #[tokio::test]
    async fn enqueue_with_meta_roundtrip_on_dequeue() {
        let Some(url) = redis_url() else {
            return;
        };
        let client = Client::open(url.as_str()).unwrap();
        let mut conn = client.get_multiplexed_async_connection().await.unwrap();
        let f = UrlFrontier::new(format!("{DEFAULT_FRONTIER_KEY}:test_meta_rt"));
        f.clear(&mut conn).await.unwrap();

        let m = FrontierMeta::new("sitemap_scheduled").with_confidence(0.9);
        f.enqueue_with_meta(&mut conn, "https://a.com/c", 2.0, &m)
            .await
            .unwrap();

        let got = f
            .get_meta(&mut conn, "https://a.com/c")
            .await
            .unwrap()
            .unwrap();
        assert_eq!(got.discovery_source, "sitemap_scheduled");
        assert!((got.confidence.unwrap() - 0.9).abs() < 1e-9);

        let (u, s, meta) = f
            .dequeue_highest_with_meta(&mut conn)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(u, "https://a.com/c");
        assert!((s - 2.0).abs() < 1e-9);
        assert_eq!(meta.as_ref().unwrap().discovery_source, "sitemap_scheduled");
        assert!(f
            .get_meta(&mut conn, "https://a.com/c")
            .await
            .unwrap()
            .is_none());

        f.clear(&mut conn).await.unwrap();
    }

    #[tokio::test]
    async fn dequeue_order_highest_first() {
        let Some(url) = redis_url() else {
            return;
        };
        let client = Client::open(url.as_str()).unwrap();
        let mut conn = client.get_multiplexed_async_connection().await.unwrap();
        let f = UrlFrontier::new(format!("{DEFAULT_FRONTIER_KEY}:test_order"));
        f.clear(&mut conn).await.unwrap();

        f.enqueue(&mut conn, "https://low.com", 1.0).await.unwrap();
        f.enqueue(&mut conn, "https://mid.com", 5.0).await.unwrap();
        f.enqueue(&mut conn, "https://high.com", 10.0)
            .await
            .unwrap();

        assert_eq!(
            f.dequeue_highest(&mut conn).await.unwrap().unwrap().0,
            "https://high.com"
        );
        assert_eq!(
            f.dequeue_highest(&mut conn).await.unwrap().unwrap().0,
            "https://mid.com"
        );
        assert_eq!(
            f.dequeue_highest(&mut conn).await.unwrap().unwrap().0,
            "https://low.com"
        );
        assert!(f.dequeue_highest(&mut conn).await.unwrap().is_none());

        f.clear(&mut conn).await.unwrap();
    }

    #[tokio::test]
    async fn stress_ten_k_unique_urls_descending_priority() {
        let Some(url) = redis_url() else {
            return;
        };
        let client = Client::open(url.as_str()).unwrap();
        let mut conn = client.get_multiplexed_async_connection().await.unwrap();
        let f = UrlFrontier::new(format!("{DEFAULT_FRONTIER_KEY}:test_10k"));
        f.clear(&mut conn).await.unwrap();

        const N: usize = 10_000;
        for i in 0..N {
            let url = format!("https://example.com/card/{i}");
            let score = i as f64;
            assert_eq!(
                f.enqueue(&mut conn, &url, score).await.unwrap(),
                EnqueueResult::Inserted
            );
        }
        assert_eq!(f.len(&mut conn).await.unwrap(), N as u64);

        let mut last_score = f64::INFINITY;
        let mut seen = std::collections::HashSet::new();
        for _ in 0..N {
            let Some((u, s)) = f.dequeue_highest(&mut conn).await.unwrap() else {
                panic!("expected URL");
            };
            assert!(
                s <= last_score + 1e-6,
                "priority must not increase: got {s} after {last_score}"
            );
            last_score = s;
            assert!(seen.insert(u.clone()), "duplicate URL dequeued: {u}");
        }
        assert!(f.dequeue_highest(&mut conn).await.unwrap().is_none());
        assert_eq!(f.len(&mut conn).await.unwrap(), 0);

        f.clear(&mut conn).await.unwrap();
    }

    /// Many workers dequeue concurrently; each URL must appear exactly once (Redis `ZPOPMAX` is atomic).
    #[tokio::test]
    async fn concurrent_dequeue_no_duplicates_no_loss() {
        let Some(url) = redis_url() else {
            return;
        };
        let uniq = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let key = format!("{DEFAULT_FRONTIER_KEY}:concurrent_{uniq}");
        let client = Client::open(url.as_str()).unwrap();
        let mut conn = client.get_multiplexed_async_connection().await.unwrap();
        let f = UrlFrontier::new(key.clone());
        f.clear(&mut conn).await.unwrap();

        const N: usize = 400;
        const WORKERS: usize = 12;
        for i in 0..N {
            let u = format!("https://concurrent.test/card/{i}");
            assert_eq!(
                f.enqueue(&mut conn, &u, i as f64).await.unwrap(),
                EnqueueResult::Inserted
            );
        }
        assert_eq!(f.len(&mut conn).await.unwrap(), N as u64);

        let mut handles = Vec::new();
        for _ in 0..WORKERS {
            let url = url.clone();
            let key = key.clone();
            handles.push(tokio::spawn(async move {
                let client = Client::open(url.as_str()).unwrap();
                let mut conn = client.get_multiplexed_async_connection().await.unwrap();
                let f = UrlFrontier::new(key);
                let mut popped = Vec::new();
                while let Some((u, _)) = f.dequeue_highest(&mut conn).await.unwrap() {
                    popped.push(u);
                }
                popped
            }));
        }

        let mut all = Vec::new();
        for h in handles {
            all.extend(h.await.unwrap());
        }

        assert_eq!(
            all.len(),
            N,
            "every enqueued URL must be dequeued exactly once; got {} pops",
            all.len()
        );
        let set: std::collections::HashSet<_> = all.iter().collect();
        assert_eq!(
            set.len(),
            N,
            "no duplicate dequeues under concurrent workers"
        );

        let mut conn = client.get_multiplexed_async_connection().await.unwrap();
        let f = UrlFrontier::new(key);
        assert_eq!(f.len(&mut conn).await.unwrap(), 0);
        f.clear(&mut conn).await.unwrap();
    }

    /// Same priority score: all members still dequeued uniquely (order among ties is Redis-defined).
    #[tokio::test]
    async fn dequeue_same_score_all_unique() {
        let Some(url) = redis_url() else {
            return;
        };
        let client = Client::open(url.as_str()).unwrap();
        let mut conn = client.get_multiplexed_async_connection().await.unwrap();
        let f = UrlFrontier::new(format!("{DEFAULT_FRONTIER_KEY}:test_ties"));
        f.clear(&mut conn).await.unwrap();

        for i in 0..50 {
            let u = format!("https://tie.test/{i}");
            f.enqueue(&mut conn, &u, 1.0).await.unwrap();
        }
        let mut seen = std::collections::HashSet::new();
        for _ in 0..50 {
            let (u, s) = f.dequeue_highest(&mut conn).await.unwrap().unwrap();
            assert!((s - 1.0).abs() < 1e-9);
            assert!(seen.insert(u));
        }
        assert!(f.dequeue_highest(&mut conn).await.unwrap().is_none());
        f.clear(&mut conn).await.unwrap();
    }

    /// Nothing should listen on `127.0.0.1:1` in CI or a normal dev machine.
    #[tokio::test]
    async fn redis_connection_refused_on_dead_port() {
        let client = Client::open("redis://127.0.0.1:1").expect("URL parses");
        client
            .get_multiplexed_async_connection()
            .await
            .expect_err("expected connection refused to dead redis port");
    }

    #[test]
    fn redis_invalid_url_open_fails() {
        assert!(Client::open("not-a-redis-url").is_err());
    }
}
