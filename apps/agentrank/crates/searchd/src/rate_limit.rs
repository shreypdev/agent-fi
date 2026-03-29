//! Fixed-window rate limit per client IP (Redis).

use redis::aio::MultiplexedConnection;
use redis::AsyncCommands;
use std::time::Duration;

const WINDOW: Duration = Duration::from_secs(60);

/// Returns `Ok(true)` if allowed, `Ok(false)` if over limit.
pub async fn check_search_rate_limit(
    conn: &mut MultiplexedConnection,
    ip: &str,
    max_per_window: u64,
) -> Result<bool, redis::RedisError> {
    let key = format!("searchrl:v1:{ip}");
    let n: u64 = conn.incr(&key, 1i64).await?;
    if n == 1 {
        let _: bool = conn.expire(&key, WINDOW.as_secs() as i64).await?;
    }
    Ok(n <= max_per_window)
}

/// Daily cap per IP for community hints (`POST /v1/hints`).
pub async fn check_hints_daily_limit(
    conn: &mut MultiplexedConnection,
    ip: &str,
    max_per_day: u64,
) -> Result<bool, redis::RedisError> {
    let day = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() / 86_400)
        .unwrap_or(0);
    let key = format!("hintsrl:v1:{ip}:{day}");
    let n: u64 = conn.incr(&key, 1i64).await?;
    if n == 1 {
        let _: bool = conn.expire(&key, 172_800).await?;
    }
    Ok(n <= max_per_day)
}

#[cfg(test)]
mod concurrency_tests {
    use super::*;
    use redis::AsyncCommands;

    #[tokio::test]
    async fn concurrent_search_rate_increments_are_atomic() {
        let Ok(redis_url) = std::env::var("REDIS_URL") else {
            return;
        };
        let Ok(client) = redis::Client::open(redis_url.as_str()) else {
            return;
        };
        let Ok(mut conn) = client.get_multiplexed_async_connection().await else {
            return;
        };
        let ip = "127.199.88.77";
        let key = format!("searchrl:v1:{ip}");
        let _: Result<u64, _> = conn.del(&key).await;

        const N: usize = 40;
        const CAP: u64 = 25;
        let mut handles = Vec::with_capacity(N);
        for _ in 0..N {
            let ru = redis_url.clone();
            let ip = ip.to_string();
            handles.push(tokio::spawn(async move {
                let client = redis::Client::open(ru.as_str()).unwrap();
                let mut conn = client.get_multiplexed_async_connection().await.unwrap();
                check_search_rate_limit(&mut conn, &ip, CAP).await.unwrap()
            }));
        }
        let mut allowed = 0u32;
        let mut denied = 0u32;
        for h in handles {
            if h.await.unwrap() {
                allowed += 1;
            } else {
                denied += 1;
            }
        }
        assert_eq!(allowed, CAP as u32);
        assert_eq!(denied, (N as u64 - CAP) as u32);
    }

    #[tokio::test]
    async fn concurrent_hints_daily_increments_are_atomic() {
        let Ok(redis_url) = std::env::var("REDIS_URL") else {
            return;
        };
        let Ok(client) = redis::Client::open(redis_url.as_str()) else {
            return;
        };
        let Ok(mut conn) = client.get_multiplexed_async_connection().await else {
            return;
        };
        let ip = "127.199.88.78";
        let day = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs() / 86_400)
            .unwrap_or(0);
        let key = format!("hintsrl:v1:{ip}:{day}");
        let _: Result<u64, _> = conn.del(&key).await;

        const N: usize = 12;
        const CAP: u64 = 7;
        let mut handles = Vec::with_capacity(N);
        for _ in 0..N {
            let ru = redis_url.clone();
            let ip = ip.to_string();
            handles.push(tokio::spawn(async move {
                let client = redis::Client::open(ru.as_str()).unwrap();
                let mut conn = client.get_multiplexed_async_connection().await.unwrap();
                check_hints_daily_limit(&mut conn, &ip, CAP).await.unwrap()
            }));
        }
        let mut allowed = 0u32;
        let mut denied = 0u32;
        for h in handles {
            if h.await.unwrap() {
                allowed += 1;
            } else {
                denied += 1;
            }
        }
        assert_eq!(allowed, CAP as u32);
        assert_eq!(denied, (N as u64 - CAP) as u32);
    }
}
