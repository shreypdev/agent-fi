//! Structured A2A-style `message/send` (JSON body, no NL parsing).

use crate::error::{ApiError, ErrorBody};
use crate::handlers::{get_agent_inner, search_execute};
use crate::models::SearchResponse;
use crate::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde::Deserialize;
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct A2aSendRequest {
    /// When set, structured skill dispatch (Week 7 gate).
    pub skill: Option<String>,
    pub query: Option<String>,
    pub limit: Option<u32>,
    pub agent_id: Option<Uuid>,
    /// Free-form path: rejected unless empty (NL deferred).
    pub text: Option<String>,
}

/// `POST /v1/a2a` — minimal structured API (not full JSON-RPC transport).
pub async fn a2a_send(
    State(state): State<AppState>,
    Json(req): Json<A2aSendRequest>,
) -> Result<Json<Value>, ApiError> {
    if let Some(t) = req
        .text
        .as_ref()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
    {
        let _ = t;
        return Err(ApiError {
            status: StatusCode::BAD_REQUEST,
            body: ErrorBody {
                error: "nl_not_supported",
                message:
                    "Natural-language A2A is deferred; use skill + query / agent_id (see docs)."
                        .into(),
            },
            retry_after_secs: None,
        });
    }

    let skill = req.skill.as_deref().unwrap_or("").trim();
    match skill {
        "search_agents" => {
            let q = req.query.as_deref().unwrap_or("").trim();
            if q.is_empty() {
                return Err(ApiError {
                    status: StatusCode::BAD_REQUEST,
                    body: ErrorBody {
                        error: "invalid_request",
                        message: "query required for search_agents".into(),
                    },
                    retry_after_secs: None,
                });
            }
            let limit = req.limit.unwrap_or(10).clamp(1, 50);
            let (results, query_time_ms) = search_execute(&state, q, limit, 0).await?;
            let total = results.len() as u64;
            let sr = SearchResponse {
                results,
                total_hits_estimate: Some(total),
                query_time_ms,
            };
            Ok(Json(serde_json::json!({
                "ok": true,
                "skill": "search_agents",
                "data": serde_json::to_value(&sr).unwrap_or(serde_json::Value::Null)
            })))
        }
        "get_agent_details" => {
            let id = req.agent_id.ok_or_else(|| ApiError {
                status: StatusCode::BAD_REQUEST,
                body: ErrorBody {
                    error: "invalid_request",
                    message: "agent_id required".into(),
                },
                retry_after_secs: None,
            })?;
            let detail = get_agent_inner(&state, id).await?;
            Ok(Json(serde_json::json!({
                "ok": true,
                "skill": "get_agent_details",
                "data": serde_json::to_value(&detail).unwrap_or(serde_json::Value::Null)
            })))
        }
        "" => Err(ApiError {
            status: StatusCode::BAD_REQUEST,
            body: ErrorBody {
                error: "invalid_request",
                message: "skill required".into(),
            },
            retry_after_secs: None,
        }),
        other => Err(ApiError {
            status: StatusCode::BAD_REQUEST,
            body: ErrorBody {
                error: "unknown_skill",
                message: format!("unknown skill: {other}"),
            },
            retry_after_secs: None,
        }),
    }
}
