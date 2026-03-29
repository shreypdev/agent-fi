//! `/.well-known/mcp.json` and Agent Card aliases for A2A discovery.

use axum::Json;
use serde_json::{json, Value};

fn public_base_url() -> String {
    std::env::var("AGENTRANK_PUBLIC_URL")
        .ok()
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.trim_end_matches('/').to_string())
        .unwrap_or_else(|| "http://127.0.0.1:8080".to_string())
}

/// MCP discovery manifest (Streamable HTTP).
pub async fn mcp_manifest() -> Json<Value> {
    let base = public_base_url();
    Json(json!({
        "version": "1.0",
        "protocolVersion": "2025-06-18",
        "serverInfo": {
            "name": "AgentRank Search",
            "version": env!("CARGO_PKG_VERSION"),
            "description": "Search the AgentRank public agent index (BM25 + hybrid vectors).",
            "homepage": base
        },
        "transport": {
            "type": "streamable-http",
            "url": format!("{base}/mcp")
        },
        "capabilities": {
            "tools": true,
            "resources": false,
            "prompts": false
        }
    }))
}

/// A2A Agent Card (same JSON at `agent-card.json` and `agent.json`).
pub async fn agent_card() -> Json<Value> {
    let base = public_base_url();
    Json(json!({
        "name": "AgentRank Search",
        "description": "Programmatic search across indexed public A2A agent cards. Use MCP tools search_agents and get_agent_details, or A2A structured message/send.",
        "url": base,
        "version": "1.0.0",
        "protocolVersion": "0.2.1",
        "capabilities": { "streaming": false, "pushNotifications": false },
        "defaultInputModes": ["application/json"],
        "defaultOutputModes": ["application/json"],
        "skills": [
            {
                "id": "search_agents",
                "name": "Search agents",
                "description": "Keyword / hybrid search over the index",
                "tags": ["search", "agents"],
                "examples": ["query about travel booking"]
            },
            {
                "id": "get_agent_details",
                "name": "Agent details",
                "description": "Fetch one agent by UUID",
                "tags": ["lookup"],
                "examples": []
            }
        ]
    }))
}
