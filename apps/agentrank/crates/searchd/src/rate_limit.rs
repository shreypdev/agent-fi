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
