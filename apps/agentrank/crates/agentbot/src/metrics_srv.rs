//! Prometheus scrape endpoint for long-running AgentBot processes.

use anyhow::Context;
use axum::routing::get;
use axum::Router;
use metrics_exporter_prometheus::PrometheusBuilder;
use std::net::SocketAddr;
use std::sync::OnceLock;

static HANDLE: OnceLock<Result<metrics_exporter_prometheus::PrometheusHandle, String>> =
    OnceLock::new();

/// Install global Prometheus recorder and return handle for `/metrics` (idempotent).
pub fn init_metrics() -> anyhow::Result<()> {
    let res = HANDLE.get_or_init(|| {
        PrometheusBuilder::new()
            .install_recorder()
            .map_err(|e| e.to_string())
    });
    match res {
        Ok(_) => Ok(()),
        Err(e) => Err(anyhow::anyhow!("prometheus recorder: {e}")),
    }
}

pub fn handle() -> Option<&'static metrics_exporter_prometheus::PrometheusHandle> {
    HANDLE.get().and_then(|r| r.as_ref().ok())
}

/// Bind and serve until error (run in `tokio::spawn`).
pub async fn serve_metrics(addr: SocketAddr) -> anyhow::Result<()> {
    let h = HANDLE
        .get()
        .context("init_metrics must run before serve_metrics")?
        .as_ref()
        .map_err(|e| anyhow::anyhow!("metrics recorder failed: {e}"))?
        .clone();
    let app = Router::new().route(
        "/metrics",
        get(move || {
            let h = h.clone();
            async move { h.render() }
        }),
    );
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .with_context(|| format!("bind metrics {addr}"))?;
    tracing::info!(%addr, "agentbot metrics listening");
    axum::serve(listener, app).await?;
    Ok(())
}
