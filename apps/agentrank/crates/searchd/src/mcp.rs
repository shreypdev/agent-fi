//! Minimal MCP JSON-RPC over HTTP (`POST /mcp`) for `initialize`, `tools/list`, `tools/call`.

use crate::error::{ApiError, ErrorBody};
use crate::handlers::{get_agent_inner, search_execute};
use crate::models::SearchResponse;
use crate::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde_json::{json, Value};
use uuid::Uuid;

pub async fn mcp_get() -> (StatusCode, Json<Value>) {
    (
        StatusCode::METHOD_NOT_ALLOWED,
        Json(json!({
            "jsonrpc": "2.0",
            "error": { "code": -32000, "message": "Method not allowed. Use POST." },
            "id": null
        })),
    )
}

/// `POST /mcp` — JSON-RPC 2.0 subset for MCP clients (Cursor, etc.).
pub async fn mcp_post(
    State(state): State<AppState>,
    Json(body): Json<Value>,
) -> Result<Json<Value>, ApiError> {
    let id = body.get("id").cloned().unwrap_or(Value::Null);
    let method = body.get("method").and_then(|m| m.as_str()).unwrap_or("");

    match method {
        "initialize" => Ok(Json(json!({
            "jsonrpc": "2.0",
            "id": id,
            "result": {
                "protocolVersion": "2024-11-05",
                "capabilities": { "tools": {} },
                "serverInfo": { "name": "agentrank-search", "version": env!("CARGO_PKG_VERSION") }
            }
        }))),
        "notifications/initialized" | "initialized" => Ok(Json(json!({
            "jsonrpc": "2.0",
            "id": id,
            "result": null
        }))),
        "tools/list" => Ok(Json(json!({
            "jsonrpc": "2.0",
            "id": id,
            "result": {
                "tools": [
                    {
                        "name": "search_agents",
                        "description": "Search the AgentRank index for agents matching a query.",
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                "query": { "type": "string" },
                                "limit": { "type": "integer", "minimum": 1, "maximum": 50 }
                            },
                            "required": ["query"]
                        }
                    },
                    {
                        "name": "get_agent_details",
                        "description": "Fetch full agent card and metadata by id.",
                        "inputSchema": {
                            "type": "object",
                            "properties": {
                                "agent_id": { "type": "string", "format": "uuid" }
                            },
                            "required": ["agent_id"]
                        }
                    }
                ]
            }
        }))),
        "tools/call" => {
            let params = body.get("params").cloned().unwrap_or(json!({}));
            let name = params.get("name").and_then(|n| n.as_str()).unwrap_or("");
            let args = params.get("arguments").cloned().unwrap_or(json!({}));
            match name {
                "search_agents" => {
                    let q = args
                        .get("query")
                        .and_then(|q| q.as_str())
                        .unwrap_or("")
                        .trim();
                    if q.is_empty() {
                        return Err(ApiError {
                            status: StatusCode::BAD_REQUEST,
                            body: ErrorBody {
                                error: "invalid_args",
                                message: "query required".into(),
                            },
                            retry_after_secs: None,
                        });
                    }
                    let limit = args
                        .get("limit")
                        .and_then(|v| v.as_u64())
                        .unwrap_or(10)
                        .clamp(1, 50) as u32;
                    let (results, query_time_ms) = search_execute(&state, q, limit, 0).await?;
                    let total = results.len() as u64;
                    let resp = SearchResponse {
                        results,
                        total_hits_estimate: Some(total),
                        query_time_ms,
                    };
                    let text = serde_json::to_string(&resp).unwrap_or_else(|_| "{}".into());
                    Ok(Json(json!({
                        "jsonrpc": "2.0",
                        "id": id,
                        "result": {
                            "content": [{ "type": "text", "text": text }],
                            "isError": false
                        }
                    })))
                }
                "get_agent_details" => {
                    let aid = args
                        .get("agent_id")
                        .and_then(|v| v.as_str())
                        .and_then(|s| Uuid::parse_str(s).ok())
                        .ok_or_else(|| ApiError {
                            status: StatusCode::BAD_REQUEST,
                            body: ErrorBody {
                                error: "invalid_args",
                                message: "agent_id (uuid) required".into(),
                            },
                            retry_after_secs: None,
                        })?;
                    let detail = get_agent_inner(&state, aid).await?;
                    let text = serde_json::to_string(&detail).unwrap_or_else(|_| "{}".into());
                    Ok(Json(json!({
                        "jsonrpc": "2.0",
                        "id": id,
                        "result": {
                            "content": [{ "type": "text", "text": text }],
                            "isError": false
                        }
                    })))
                }
                _ => Ok(Json(json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "error": { "code": -32601, "message": format!("unknown tool: {name}") }
                }))),
            }
        }
        _ => Ok(Json(json!({
            "jsonrpc": "2.0",
            "id": id,
            "error": { "code": -32601, "message": format!("method not found: {method}") }
        }))),
    }
}
