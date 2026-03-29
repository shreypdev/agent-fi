//! Fetch `/.well-known/agent.json` (or any card URL), validate, and persist to PostgreSQL.

mod crawl_run;
mod error;
pub mod github_discover;
mod host_rate_limit;
pub mod metrics_srv;

pub use crawl_run::{run_drain, run_loop, CrawlRunConfig};
pub use error::IngestError;
pub use host_rate_limit::check_host_fetch_allowed;

use agentrank_card::parse_agent_card_bytes;
use agentrank_crawl_policy::validate_outbound_url;
use reqwest::Client;
use sqlx::types::Json;
use sqlx::PgPool;
use std::time::Duration;
use url::Url;
use uuid::Uuid;

/// URL policy for fetches (SSRF mitigation). Default: HTTPS only, no localhost HTTP.
#[derive(Clone, Copy, Debug, Default)]
pub struct IngestPolicy {
    pub allow_http_localhost: bool,
    /// When true, `https://127.0.0.1` / `https://[::1]` allowed (integration tests only).
    pub allow_loopback_https: bool,
}

impl IngestPolicy {
    /// `AGENTBOT_ALLOW_HTTP_LOCALHOST=1` — `http://127.0.0.1` / `localhost`.
    /// `AGENTBOT_ALLOW_LOOPBACK_HTTPS=1` — `https://127.0.0.1` (tests).
    pub fn from_env() -> Self {
        let allow_http_localhost = std::env::var("AGENTBOT_ALLOW_HTTP_LOCALHOST")
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(false);
        let allow_loopback_https = std::env::var("AGENTBOT_ALLOW_LOOPBACK_HTTPS")
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(false);
        Self {
            allow_http_localhost,
            allow_loopback_https,
        }
    }
}

/// `User-Agent` for outbound fetches (RFC 9309 style product token + URL + contact).
pub const AGENTBOT_USER_AGENT: &str =
    "AgentBot/1.0 (+https://agentrank.dev/bot; agentbot@agentrank.dev)";

const DEFAULT_MAX_BODY: usize = 1_048_576; // 1 MiB

/// Build HTTP client: redirects limited (max 5), 60s timeout, rustls.
pub fn http_client() -> Result<Client, reqwest::Error> {
    http_client_with_timeout(Duration::from_secs(60))
}

/// Same as [`http_client`] but with a custom request timeout (used for tests and tight SLOs).
pub fn http_client_with_timeout(timeout: Duration) -> Result<Client, reqwest::Error> {
    Client::builder()
        .user_agent(AGENTBOT_USER_AGENT)
        .redirect(reqwest::redirect::Policy::limited(5))
        .timeout(timeout)
        .build()
}

/// Result of a successful ingest (card stored).
#[derive(Debug, Clone)]
pub struct IngestSuccess {
    pub agent_id: Uuid,
    pub external_id: String,
    pub crawl_history_id: i64,
}

/// Fetch one URL, parse Agent Card, upsert provider + agent, record crawl + trust.
pub async fn ingest_card_url(
    pool: &PgPool,
    client: &Client,
    fetch_url: &str,
    max_body_bytes: usize,
) -> Result<IngestSuccess, IngestError> {
    ingest_card_url_with_policy(
        pool,
        client,
        fetch_url,
        max_body_bytes,
        IngestPolicy::default(),
    )
    .await
}

/// Same as [`ingest_card_url`] with explicit URL policy (localhost HTTP for tests, etc.).
pub async fn ingest_card_url_with_policy(
    pool: &PgPool,
    client: &Client,
    fetch_url: &str,
    max_body_bytes: usize,
    policy: IngestPolicy,
) -> Result<IngestSuccess, IngestError> {
    let parsed_url = Url::parse(fetch_url)?;
    validate_outbound_url(
        &parsed_url,
        policy.allow_http_localhost,
        policy.allow_loopback_https,
    )?;
    let resp = client.get(parsed_url.clone()).send().await?;
    let status = resp.status();
    let final_url = resp.url().clone();
    validate_outbound_url(
        &final_url,
        policy.allow_http_localhost,
        policy.allow_loopback_https,
    )
    .map_err(|e| IngestError::PostRedirectPolicy(format!("{final_url}: {e}")))?;
    let bytes = resp.bytes().await?;

    if bytes.len() > max_body_bytes {
        let _ = sqlx::query(
            r#"
            INSERT INTO crawl_history (url, agent_id, http_status, error_code, error_detail, response_bytes)
            VALUES ($1, NULL, $2, $3, $4, $5)
            "#,
        )
        .bind(fetch_url)
        .bind(status.as_u16() as i32)
        .bind(Some("body_too_large"))
        .bind(Some(format!("{} bytes exceeds max {}", bytes.len(), max_body_bytes)))
        .bind(bytes.len() as i32)
        .execute(pool)
        .await?;
        return Err(IngestError::BodyTooLarge(bytes.len(), max_body_bytes));
    }

    if !status.is_success() {
        let _ = sqlx::query(
            r#"
            INSERT INTO crawl_history (url, agent_id, http_status, error_code, error_detail, response_bytes)
            VALUES ($1, NULL, $2, $3, $4, $5)
            "#,
        )
        .bind(fetch_url)
        .bind(status.as_u16() as i32)
        .bind(Some("http_error"))
        .bind(Some(status.to_string()))
        .bind(bytes.len() as i32)
        .execute(pool)
        .await?;
        return Err(IngestError::HttpStatus(status));
    }

    let card = match parse_agent_card_bytes(&bytes, fetch_url, final_url.as_str()) {
        Ok(c) => c,
        Err(e) => {
            let _ = sqlx::query(
                r#"
                INSERT INTO crawl_history (url, agent_id, http_status, error_code, error_detail, response_bytes)
                VALUES ($1, NULL, $2, $3, $4, $5)
                "#,
            )
            .bind(fetch_url)
            .bind(status.as_u16() as i32)
            .bind(Some("parse_error"))
            .bind(Some(e.to_string()))
            .bind(bytes.len() as i32)
            .execute(pool)
            .await?;
            return Err(IngestError::CardParse(e));
        }
    };

    let mut tx = pool.begin().await?;

    let provider_id: Uuid = sqlx::query_scalar(
        r#"
        INSERT INTO providers (primary_domain, display_name)
        VALUES ($1, $2)
        ON CONFLICT (primary_domain) DO UPDATE SET
            display_name = COALESCE(EXCLUDED.display_name, providers.display_name),
            updated_at = NOW()
        RETURNING id
        "#,
    )
    .bind(&card.primary_domain)
    .bind(card.provider_display_name.as_deref())
    .fetch_one(&mut *tx)
    .await?;

    let agent_id: Uuid = sqlx::query_scalar(
        r#"
        INSERT INTO agents (
            provider_id, external_id, source_url, canonical_url, card_json, content_hash,
            name, description, endpoint_url, protocol_version
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        ON CONFLICT (external_id) DO UPDATE SET
            provider_id = EXCLUDED.provider_id,
            source_url = EXCLUDED.source_url,
            canonical_url = EXCLUDED.canonical_url,
            card_json = EXCLUDED.card_json,
            content_hash = EXCLUDED.content_hash,
            name = EXCLUDED.name,
            description = EXCLUDED.description,
            endpoint_url = EXCLUDED.endpoint_url,
            protocol_version = EXCLUDED.protocol_version,
            updated_at = NOW()
        RETURNING id
        "#,
    )
    .bind(provider_id)
    .bind(&card.external_id)
    .bind(&card.source_url)
    .bind(&card.canonical_url)
    .bind(Json(card.normalized_card.clone()))
    .bind(&card.content_hash)
    .bind(&card.name)
    .bind(&card.description)
    .bind(&card.endpoint_url)
    .bind(&card.protocol_version)
    .fetch_one(&mut *tx)
    .await?;

    let crawl_history_id: i64 = sqlx::query_scalar(
        r#"
        INSERT INTO crawl_history (url, agent_id, http_status, error_code, error_detail, response_bytes)
        VALUES ($1, $2, $3, NULL, NULL, $4)
        RETURNING id
        "#,
    )
    .bind(fetch_url)
    .bind(agent_id)
    .bind(status.as_u16() as i32)
    .bind(bytes.len() as i32)
    .fetch_one(&mut *tx)
    .await?;

    sqlx::query(
        r#"
        INSERT INTO trust_records (agent_id, trust_tier)
        VALUES ($1, 'indexed')
        ON CONFLICT (agent_id) DO NOTHING
        "#,
    )
    .bind(agent_id)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;

    Ok(IngestSuccess {
        agent_id,
        external_id: card.external_id,
        crawl_history_id,
    })
}

/// Default max response body size (1 MiB).
pub fn default_max_body_bytes() -> usize {
    DEFAULT_MAX_BODY
}
