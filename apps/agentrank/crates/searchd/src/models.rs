use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct SearchRequest {
    pub query: String,
    #[serde(default = "default_limit")]
    pub limit: u32,
    #[serde(default)]
    pub offset: u32,
}

fn default_limit() -> u32 {
    10
}

pub const MAX_LIMIT: u32 = 50;

#[derive(Debug, Deserialize)]
pub struct HintsRequest {
    pub url: String,
    #[serde(default)]
    pub source: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct HintsResponse {
    pub hint_id: String,
    pub queued: bool,
}

#[derive(Debug, Serialize)]
pub struct SearchResponse {
    pub results: Vec<SearchResultItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_hits_estimate: Option<u64>,
    pub query_time_ms: u64,
}

#[derive(Debug, Serialize)]
pub struct SearchResultItem {
    pub agent_id: Uuid,
    pub name: String,
    pub description: String,
    pub endpoint_url: String,
    pub protocol_version: String,
    pub trust_tier: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snippet: Option<String>,
}
