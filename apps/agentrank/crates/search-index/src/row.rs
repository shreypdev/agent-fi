//! Rows loaded from Postgres for indexing.

use serde_json::Value;
use sqlx::types::Json;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct AgentIndexRow {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub endpoint_url: String,
    pub protocol_version: String,
    pub card_json: Json<Value>,
    pub provider_display_name: Option<String>,
    pub trust_tier: Option<String>,
}

pub const AGENTS_FOR_INDEX_SQL: &str = r#"
SELECT
    a.id,
    a.name,
    a.description,
    a.endpoint_url,
    a.protocol_version,
    a.card_json,
    p.display_name AS provider_display_name,
    t.trust_tier AS trust_tier
FROM agents a
LEFT JOIN providers p ON a.provider_id = p.id
LEFT JOIN trust_records t ON t.agent_id = a.id
ORDER BY a.created_at ASC
"#;

pub const AGENT_BY_ID_SQL: &str = r#"
SELECT
    a.id,
    a.name,
    a.description,
    a.endpoint_url,
    a.protocol_version,
    a.card_json,
    p.display_name AS provider_display_name,
    t.trust_tier AS trust_tier
FROM agents a
LEFT JOIN providers p ON a.provider_id = p.id
LEFT JOIN trust_records t ON t.agent_id = a.id
WHERE a.id = $1
"#;
