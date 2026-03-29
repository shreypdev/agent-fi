//! Post-ingest Tantivy + Qdrant upsert and lag metric.

use agentrank_embed::{embed_document, EMBEDDING_DIM};
use agentrank_search_index::skills_blob_from_card_json;
use agentrank_search_index::store::upsert_agent;
use agentrank_vector::upsert_agent_vector;
use chrono::Utc;
use metrics::histogram;
use qdrant_client::Qdrant;
use sqlx::PgPool;
use std::path::Path;
use std::sync::Arc;
use uuid::Uuid;

/// Full index update after DB commit: embedding → Qdrant + Tantivy; records lag histogram.
pub async fn index_agent_after_ingest(
    pool: &PgPool,
    agent_id: Uuid,
    search_index_path: Option<&Path>,
    qdrant: Option<&Arc<Qdrant>>,
) {
    if search_index_path.is_none() && qdrant.is_none() {
        return;
    }

    let row = match sqlx::query_as::<_, AgentEmbedRow>(
        r#"
        SELECT a.name, a.description, a.card_json, a.updated_at
        FROM agents a
        WHERE a.id = $1
        "#,
    )
    .bind(agent_id)
    .fetch_optional(pool)
    .await
    {
        Ok(Some(r)) => r,
        Ok(None) => {
            tracing::warn!(%agent_id, "index_pipeline: agent row missing");
            return;
        }
        Err(e) => {
            tracing::warn!(%agent_id, "index_pipeline: db: {e}");
            let _ = record_index_job_failure(pool, agent_id, &e.to_string()).await;
            metrics::counter!("agentrank_index_upsert_failures_total").increment(1);
            return;
        }
    };

    let skills = skills_blob_from_card_json(&row.card_json.0);
    let vector = embed_document(&row.name, &row.description, &skills);

    let lag_secs = (Utc::now() - row.updated_at).num_seconds().max(0) as f64;
    histogram!("agentrank_index_lag_seconds").record(lag_secs);

    if let Some(q) = qdrant {
        if let Err(e) = upsert_agent_vector(q.as_ref(), agent_id, vector.clone()).await {
            tracing::warn!(%agent_id, "qdrant upsert: {e}");
            let _ = record_index_job_failure(pool, agent_id, &format!("qdrant: {e}")).await;
            metrics::counter!("agentrank_index_upsert_failures_total").increment(1);
        }
    }

    if let Some(path) = search_index_path {
        if let Err(e) = upsert_agent(pool, path, agent_id).await {
            tracing::warn!(%agent_id, "tantivy upsert: {e}");
            let _ = record_index_job_failure(pool, agent_id, &format!("tantivy: {e}")).await;
            metrics::counter!("agentrank_index_upsert_failures_total").increment(1);
        }
    }
}

#[derive(Debug, sqlx::FromRow)]
struct AgentEmbedRow {
    name: String,
    description: String,
    card_json: sqlx::types::Json<serde_json::Value>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

async fn record_index_job_failure(
    pool: &PgPool,
    agent_id: Uuid,
    err: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO index_jobs (agent_id, attempts, next_run_at, last_error, updated_at)
        VALUES ($1, 1, NOW() + INTERVAL '2 minutes', $2, NOW())
        ON CONFLICT (agent_id) DO UPDATE SET
            attempts = index_jobs.attempts + 1,
            next_run_at = NOW() + INTERVAL '2 minutes',
            last_error = EXCLUDED.last_error,
            updated_at = NOW()
        "#,
    )
    .bind(agent_id)
    .bind(err)
    .execute(pool)
    .await?;
    Ok(())
}

/// Qdrant / embedding vector size.
pub fn embedding_dim() -> u64 {
    EMBEDDING_DIM as u64
}
