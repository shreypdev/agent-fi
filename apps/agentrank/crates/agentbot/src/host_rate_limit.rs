//! Per-host fetch rate limiting (Redis fixed 1-second window).

use redis::aio::MultiplexedConnection;
use redis::AsyncCommands;
use std::time::{SystemTime, UNIX_EPOCH};

/// Returns `Ok(true)` if this host may fetch now, `Ok(false)` if over per-second quota (re-enqueue).
pub async fn check_host_fetch_allowed(
    conn: &mut MultiplexedConnection,
    host_key: &str,
    max_per_sec: u64,
) -> Result<bool, redis::RedisError> {
    if max_per_sec == 0 {
        return Ok(true);
    }
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let key = format!("agentbot:hostrl:v1:{host_key}:{now}");
    let n: u64 = conn.incr(&key, 1i64).await?;
    if n == 1 {
        let _: bool = conn.expire(&key, 3i64).await?;
    }
    Ok(n <= max_per_sec)
}

#[cfg(test)]
mod tests {
    use super::*;
    use redis::Client;

    fn redis_url() -> Option<String> {
        std::env::var("REDIS_URL").ok()
    }

    #[tokio::test]
    async fn max_per_sec_zero_always_allows() {
        let Some(url) = redis_url() else {
            return;
        };
        let client = Client::open(url.as_str()).unwrap();
        let mut conn = client.get_multiplexed_async_connection().await.unwrap();
        let host = format!(
            "rl_zero:{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        );
        for _ in 0..5 {
            assert!(check_host_fetch_allowed(&mut conn, &host, 0).await.unwrap());
        }
    }

    #[tokio::test]
    async fn same_host_second_bucket_allows_two_then_blocks_third() {
        let Some(url) = redis_url() else {
            return;
        };
        let client = Client::open(url.as_str()).unwrap();
        let mut conn = client.get_multiplexed_async_connection().await.unwrap();
        let host = format!(
            "rl_two:{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        );
        assert!(check_host_fetch_allowed(&mut conn, &host, 2).await.unwrap());
        assert!(check_host_fetch_allowed(&mut conn, &host, 2).await.unwrap());
        assert!(!check_host_fetch_allowed(&mut conn, &host, 2).await.unwrap());
    }

    #[tokio::test]
    async fn independent_hosts_do_not_share_quota() {
        let Some(url) = redis_url() else {
            return;
        };
        let client = Client::open(url.as_str()).unwrap();
        let mut conn = client.get_multiplexed_async_connection().await.unwrap();
        let u = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let a = format!("rl_a:{u}");
        let b = format!("rl_b:{u}");
        assert!(check_host_fetch_allowed(&mut conn, &a, 1).await.unwrap());
        assert!(!check_host_fetch_allowed(&mut conn, &a, 1).await.unwrap());
        assert!(check_host_fetch_allowed(&mut conn, &b, 1).await.unwrap());
    }

    /// Two concurrent INCRs for the same host/window should not both observe "under cap" past the limit.
    #[tokio::test]
    async fn concurrent_increments_respect_cap() {
        let Some(url) = redis_url() else {
            return;
        };
        let u = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let host = format!("rl_race:{u}");
        let max: u64 = 1;
        let mut handles = Vec::new();
        for _ in 0..8 {
            let url = url.clone();
            let host = host.clone();
            handles.push(tokio::spawn(async move {
                let client = Client::open(url.as_str()).unwrap();
                let mut conn = client.get_multiplexed_async_connection().await.unwrap();
                check_host_fetch_allowed(&mut conn, &host, max)
                    .await
                    .unwrap()
            }));
        }
        let mut allowed = 0u32;
        for h in handles {
            if h.await.unwrap() {
                allowed += 1;
            }
        }
        assert_eq!(allowed, 1, "only one fetch may pass when max_per_sec=1");
    }
}
