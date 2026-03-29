//! Prometheus scrape endpoint and HTTP request metrics.

use axum::extract::Request;
use axum::http::{header, HeaderMap, StatusCode};
use axum::middleware::Next;
use axum::response::IntoResponse;
use metrics::{counter, histogram};
use metrics_exporter_prometheus::{PrometheusBuilder, PrometheusHandle};
use std::sync::OnceLock;

static PROM: OnceLock<Result<PrometheusHandle, String>> = OnceLock::new();

/// Install the global Prometheus metrics recorder (idempotent).
pub fn init() -> anyhow::Result<()> {
    let res = PROM.get_or_init(|| {
        PrometheusBuilder::new()
            .install_recorder()
            .map_err(|e| e.to_string())
    });
    match res {
        Ok(_) => Ok(()),
        Err(e) => Err(anyhow::anyhow!("prometheus recorder: {e}")),
    }
}

fn path_label(path: &str) -> &'static str {
    match path {
        "/health" => "/health",
        "/ready" => "/ready",
        "/metrics" => "/metrics",
        "/v1/search" => "/v1/search",
        "/v1/hints" => "/v1/hints",
        "/v1/a2a" => "/v1/a2a",
        "/mcp" => "/mcp",
        p if p.starts_with("/.well-known/") => "/.well-known/*",
        p if p.starts_with("/v1/agents/") => "/v1/agents/:id",
        _ => "other",
    }
}

pub async fn http_metrics(req: Request, next: Next) -> impl IntoResponse {
    let method = req.method().clone();
    let path = req.uri().path().to_owned();
    let label = path_label(&path);
    let start = std::time::Instant::now();
    let res = next.run(req).await;
    let status = res.status().as_u16().to_string();
    let secs = start.elapsed().as_secs_f64();
    counter!(
        "http_requests_total",
        "method" => method.to_string(),
        "path" => label,
        "status" => status
    )
    .increment(1);
    histogram!(
        "http_request_duration_seconds",
        "method" => method.to_string(),
        "path" => label
    )
    .record(secs);
    res
}

fn metrics_bearer_authorized(headers: &HeaderMap) -> bool {
    let Ok(token) = std::env::var("METRICS_BEARER_TOKEN") else {
        return true;
    };
    let token = token.trim();
    if token.is_empty() {
        return true;
    }
    headers
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .is_some_and(|b| b == token)
}

pub async fn prometheus(headers: HeaderMap) -> impl IntoResponse {
    if !metrics_bearer_authorized(&headers) {
        return (
            StatusCode::UNAUTHORIZED,
            "metrics: set Authorization: Bearer <METRICS_BEARER_TOKEN>".to_string(),
        );
    }
    match PROM.get() {
        Some(Ok(h)) => (StatusCode::OK, h.render()),
        Some(Err(e)) => (
            StatusCode::SERVICE_UNAVAILABLE,
            format!("metrics unavailable: {e}"),
        ),
        None => (
            StatusCode::SERVICE_UNAVAILABLE,
            "metrics not initialized".to_string(),
        ),
    }
}
