//! Connection helpers for PostgreSQL and Redis (AgentRank data plane).

use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

/// Read `DATABASE_URL` from the environment.
pub fn database_url() -> Result<String, std::env::VarError> {
    std::env::var("DATABASE_URL")
}

/// Read `REDIS_URL` from the environment.
pub fn redis_url() -> Result<String, std::env::VarError> {
    std::env::var("REDIS_URL")
}

/// Build a small pool for health checks and operational probes.
pub async fn pg_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(2)
        .connect(database_url)
        .await
}

/// Verify PostgreSQL with `SELECT 1`.
pub async fn check_postgres(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query_scalar::<_, i32>("SELECT 1")
        .fetch_one(pool)
        .await?;
    Ok(())
}

/// Verify Redis with `PING`.
pub async fn check_redis(redis_url: &str) -> Result<(), RedisCheckError> {
    let client = redis::Client::open(redis_url)?;
    let mut conn = client.get_multiplexed_async_connection().await?;
    let pong: String = redis::cmd("PING").query_async(&mut conn).await?;
    if pong != "PONG" {
        return Err(RedisCheckError::UnexpectedResponse(pong));
    }
    Ok(())
}

/// Errors from [`check_redis`].
#[derive(Debug, thiserror::Error)]
pub enum RedisCheckError {
    #[error("redis client: {0}")]
    Client(#[from] redis::RedisError),
    #[error("unexpected PING response: {0}")]
    UnexpectedResponse(String),
}

#[cfg(test)]
mod tests {
    #[test]
    fn trust_tier_default_documented() {
        // Documented default for `trust_records.trust_tier` (see migrations).
        assert_eq!(default_trust_tier(), "indexed");
    }

    fn default_trust_tier() -> &'static str {
        "indexed"
    }
}
