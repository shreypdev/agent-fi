//! Qdrant collection management and vector upsert/search (gRPC).

use anyhow::Context;
use qdrant_client::qdrant::{
    CreateCollectionBuilder, DeletePointsBuilder, Distance, PointStruct, PointsIdsList,
    SearchPointsBuilder, UpsertPointsBuilder, VectorParamsBuilder,
};
use qdrant_client::{Payload, Qdrant};
use uuid::Uuid;

pub const COLLECTION_AGENTS: &str = "agents";

/// Connect to Qdrant from `QDRANT_URL` (e.g. `http://localhost:6334` for gRPC).
pub async fn connect() -> anyhow::Result<Qdrant> {
    let url = std::env::var("QDRANT_URL").context("QDRANT_URL")?;
    Qdrant::from_url(url.as_str())
        .build()
        .context("qdrant connect")
}

/// Ensure `agents` collection exists with cosine + `vector_dim`.
pub async fn ensure_agents_collection(client: &Qdrant, vector_dim: u64) -> anyhow::Result<()> {
    let exists = client.collection_exists(COLLECTION_AGENTS).await?;
    if exists {
        return Ok(());
    }
    client
        .create_collection(
            CreateCollectionBuilder::new(COLLECTION_AGENTS)
                .vectors_config(VectorParamsBuilder::new(vector_dim, Distance::Cosine)),
        )
        .await
        .context("create_collection agents")?;
    Ok(())
}

/// Upsert one agent vector; point id is the agent UUID.
pub async fn upsert_agent_vector(
    client: &Qdrant,
    agent_id: Uuid,
    vector: Vec<f32>,
) -> anyhow::Result<()> {
    let payload: Payload = serde_json::json!({
        "agent_id": agent_id.to_string(),
    })
    .try_into()
    .unwrap_or_else(|_| Payload::default());

    let point = PointStruct::new(agent_id, vector, payload);
    client
        .upsert_points(UpsertPointsBuilder::new(COLLECTION_AGENTS, vec![point]).wait(true))
        .await
        .context("qdrant upsert")?;
    Ok(())
}

/// Delete point for agent.
pub async fn delete_agent_vector(client: &Qdrant, agent_id: Uuid) -> anyhow::Result<()> {
    client
        .delete_points(
            DeletePointsBuilder::new(COLLECTION_AGENTS)
                .points(PointsIdsList {
                    ids: vec![agent_id.into()],
                })
                .wait(true),
        )
        .await
        .context("qdrant delete")?;
    Ok(())
}

/// kNN search; returns (agent_id, score) ordered by descending similarity.
pub async fn search_knn(
    client: &Qdrant,
    query_vector: Vec<f32>,
    limit: u64,
) -> anyhow::Result<Vec<(Uuid, f32)>> {
    let res = client
        .search_points(SearchPointsBuilder::new(
            COLLECTION_AGENTS,
            query_vector,
            limit,
        ))
        .await
        .context("qdrant search")?;

    let mut out = Vec::new();
    for p in res.result {
        let score = p.score;
        let Some(pid) = p.id else {
            continue;
        };
        let uid = match pid.point_id_options {
            Some(qdrant_client::qdrant::point_id::PointIdOptions::Uuid(u)) => {
                Uuid::parse_str(&u).ok()
            }
            Some(qdrant_client::qdrant::point_id::PointIdOptions::Num(n)) => {
                // Legacy numeric ids not used
                let _ = n;
                None
            }
            None => None,
        };
        if let Some(uid) = uid {
            out.push((uid, score));
        }
    }
    Ok(out)
}

/// Health: list collections (cheap).
pub async fn health_check(client: &Qdrant) -> anyhow::Result<()> {
    client
        .list_collections()
        .await
        .context("qdrant list_collections")?;
    Ok(())
}
