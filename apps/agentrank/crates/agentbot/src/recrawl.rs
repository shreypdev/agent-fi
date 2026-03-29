//! Track content hash and re-enqueue card URL at low priority for refresh.

use agentrank_frontier::UrlFrontier;
use redis::aio::MultiplexedConnection;
use sqlx::PgPool;

/// Record ingest outcome and enqueue `card_url` for a future recrawl (low priority).
pub async fn schedule_recrawl(
    pool: &PgPool,
    redis: &mut MultiplexedConnection,
    frontier: &UrlFrontier,
    card_url: &str,
    content_hash: &str,
) -> anyhow::Result<()> {
    let hours: i32 = 6;

    sqlx::query(
        r#"
        INSERT INTO frontier_url_state (
            url, last_content_hash, last_ingest_at, next_fetch_at,
            change_streak, stable_streak
        )
        VALUES ($1, $2, NOW(), NOW() + ($3 * interval '1 hour'), 1, 0)
        ON CONFLICT (url) DO UPDATE SET
            last_content_hash = EXCLUDED.last_content_hash,
            last_ingest_at = NOW(),
            next_fetch_at = NOW() + ($3 * interval '1 hour'),
            change_streak = CASE
                WHEN frontier_url_state.last_content_hash IS DISTINCT FROM EXCLUDED.last_content_hash
                THEN frontier_url_state.change_streak + 1
                ELSE 0
            END,
            stable_streak = CASE
                WHEN frontier_url_state.last_content_hash IS DISTINCT FROM EXCLUDED.last_content_hash
                THEN 0
                ELSE frontier_url_state.stable_streak + 1
            END
        "#,
    )
    .bind(card_url)
    .bind(content_hash)
    .bind(hours)
    .execute(pool)
    .await?;

    let _ = frontier.enqueue(redis, card_url, 0.15).await;
    Ok(())
}
