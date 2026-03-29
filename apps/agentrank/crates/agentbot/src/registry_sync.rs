//! Paginated registry sync with `registry_sync_state` cursor persistence.

use agentrank_frontier::{FrontierMeta, UrlFrontier, DEFAULT_FRONTIER_KEY};
use agentrank_registry_connectors::{
    AgentVerseRegistry, GenericJsonRegistry, McpSoRegistry, PulseMcpRegistry, RegistrySource,
};
use redis::aio::MultiplexedConnection;
use reqwest::Client;
use sqlx::PgPool;

async fn load_cursor(pool: &PgPool, name: &str) -> anyhow::Result<Option<String>> {
    let row: Option<Option<String>> = sqlx::query_scalar(
        r#"SELECT last_cursor FROM registry_sync_state WHERE registry_name = $1"#,
    )
    .bind(name)
    .fetch_optional(pool)
    .await?;
    Ok(row.flatten())
}

async fn save_cursor(
    pool: &PgPool,
    name: &str,
    cursor: Option<&str>,
    n: u64,
) -> anyhow::Result<()> {
    sqlx::query(
        r#"
        INSERT INTO registry_sync_state (registry_name, last_cursor, last_sync_at, total_synced, errors)
        VALUES ($1, $2, NOW(), $3, 0)
        ON CONFLICT (registry_name) DO UPDATE SET
            last_cursor = EXCLUDED.last_cursor,
            last_sync_at = NOW(),
            total_synced = registry_sync_state.total_synced + EXCLUDED.total_synced
        "#,
    )
    .bind(name)
    .bind(cursor)
    .bind(n as i64)
    .execute(pool)
    .await?;
    Ok(())
}

async fn enqueue(
    redis: &mut MultiplexedConnection,
    items: &[agentrank_registry_connectors::DiscoveredUrl],
) -> anyhow::Result<u32> {
    let f = UrlFrontier::new(DEFAULT_FRONTIER_KEY);
    let mut n = 0u32;
    for d in items {
        let meta = FrontierMeta {
            discovery_source: d.discovery_source.clone(),
            confidence: d.confidence,
        };
        f.enqueue_with_meta(redis, &d.url, d.priority, &meta)
            .await?;
        n += 1;
    }
    Ok(n)
}

async fn run_source<S: RegistrySource + Send + Sync>(
    pool: &PgPool,
    http: &Client,
    redis: &mut MultiplexedConnection,
    src: &S,
) -> anyhow::Result<()> {
    let name = src.source_name();
    let mut cursor = load_cursor(pool, name).await?;
    let mut pages = 0u32;
    loop {
        let (items, next) = src.discover_paginated(http, cursor.clone()).await?;
        let n = items.len() as u64;
        enqueue(redis, &items).await?;
        save_cursor(pool, name, next.as_deref(), n).await?;
        pages += 1;
        if next.is_none() || pages > 500 {
            break;
        }
        cursor = next;
    }
    Ok(())
}

/// Run all configured paginated registries (env-gated).
pub async fn run_paginated_registry_tick(
    pool: &PgPool,
    http: &Client,
    redis: &mut MultiplexedConnection,
) -> anyhow::Result<()> {
    if let (Ok(base), Ok(tenant), Ok(key)) = (
        std::env::var("PULSEMCP_API_BASE"),
        std::env::var("PULSEMCP_TENANT_ID"),
        std::env::var("PULSEMCP_API_KEY"),
    ) {
        let src = PulseMcpRegistry {
            api_base: base,
            tenant_id: tenant,
            api_key: key,
            default_priority: 0.9,
        };
        if let Err(e) = run_source(pool, http, redis, &src).await {
            tracing::warn!(source = "pulsemcp", "registry sync: {e}");
        }
    }

    if let (Ok(base), Ok(key)) = (
        std::env::var("AGENTVERSE_API_BASE"),
        std::env::var("AGENTVERSE_API_KEY"),
    ) {
        let src = AgentVerseRegistry {
            api_base: base,
            api_key: key,
            default_priority: 0.85,
        };
        if let Err(e) = run_source(pool, http, redis, &src).await {
            tracing::warn!(source = "agentverse", "registry sync: {e}");
        }
    }

    if let Ok(url) = std::env::var("MCPSO_REGISTRY_JSON_URL") {
        if !url.is_empty() {
            let src = McpSoRegistry {
                api_url: url,
                default_priority: 0.85,
            };
            if let Err(e) = run_source(pool, http, redis, &src).await {
                tracing::warn!(source = "mcpso", "registry sync: {e}");
            }
        }
    }

    if let (Ok(api_url), Ok(path)) = (
        std::env::var("GENERIC_REGISTRY_API_URL"),
        std::env::var("GENERIC_REGISTRY_ENTRIES_PATH"),
    ) {
        let page_param =
            std::env::var("GENERIC_REGISTRY_PAGE_PARAM").unwrap_or_else(|_| "cursor".into());
        let card_field =
            std::env::var("GENERIC_REGISTRY_CARD_URL_FIELD").unwrap_or_else(|_| "card_url".into());
        let auth = match (
            std::env::var("GENERIC_REGISTRY_AUTH_HEADER"),
            std::env::var("GENERIC_REGISTRY_AUTH_VALUE"),
        ) {
            (Ok(h), Ok(v)) => Some((h, v)),
            _ => None,
        };
        let src = GenericJsonRegistry {
            api_url,
            auth_header: auth,
            page_param,
            entries_path: path,
            card_url_field: card_field,
            default_priority: 0.8,
        };
        if let Err(e) = run_source(pool, http, redis, &src).await {
            tracing::warn!(source = "generic_registry", "registry sync: {e}");
        }
    }

    Ok(())
}
