//! GitHub Code Search → candidate `raw.githubusercontent.com` card URLs (best-effort).

use anyhow::Context;
use reqwest::Client;
use reqwest::StatusCode;
use serde_json::Value;
use url::Url;

fn default_github_api_base() -> String {
    std::env::var("GITHUB_API_BASE_URL").unwrap_or_else(|_| "https://api.github.com".to_string())
}

/// `api_base` is typically `https://api.github.com` or a wiremock root (e.g. `http://127.0.0.1:PORT`).
pub async fn discover_github_card_urls_at(
    client: &Client,
    api_base: &str,
    token: &str,
    query: &str,
    max: u32,
) -> anyhow::Result<Vec<String>> {
    let root = api_base.trim_end_matches('/');
    let mut u = Url::parse(&format!("{root}/search/code"))?;
    u.query_pairs_mut()
        .append_pair("q", query)
        .append_pair("per_page", &max.min(100).to_string());

    let resp = client
        .get(u.as_str())
        .header("Authorization", format!("Bearer {}", token.trim()))
        .header("Accept", "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .header("User-Agent", "AgentRank-AgentBot/1.0")
        .send()
        .await?;

    if let Some(rl) = resp.headers().get("x-ratelimit-remaining") {
        tracing::info!(?rl, "github X-RateLimit-Remaining");
    }
    if let Some(reset) = resp.headers().get("x-ratelimit-reset") {
        tracing::debug!(?reset, "github X-RateLimit-Reset");
    }

    let status = resp.status();
    let body_text = resp.text().await?;
    if status == StatusCode::FORBIDDEN || status == StatusCode::TOO_MANY_REQUESTS {
        anyhow::bail!("github api rate-limited or forbidden ({status}): {body_text}");
    }
    if !status.is_success() {
        anyhow::bail!("github api {status}: {body_text}");
    }

    let v: Value = serde_json::from_str(&body_text)?;
    let items = v["items"].as_array().cloned().unwrap_or_default();
    let mut out = Vec::new();
    for it in items {
        let Some(repo) = it["repository"]["full_name"].as_str() else {
            continue;
        };
        for b in ["main", "master"] {
            out.push(format!(
                "https://raw.githubusercontent.com/{repo}/{b}/.well-known/agent.json"
            ));
        }
    }
    Ok(out)
}

pub async fn discover_github_card_urls(
    client: &Client,
    query: &str,
    max: u32,
) -> anyhow::Result<Vec<String>> {
    let token = std::env::var("GITHUB_TOKEN").context("GITHUB_TOKEN for GitHub API")?;
    let base = default_github_api_base();
    discover_github_card_urls_at(client, &base, &token, query, max).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{header_exists, method, path, query_param};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn parses_search_items_to_raw_urls() {
        let srv = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/search/code"))
            .and(query_param("q", "filename:agent.json"))
            .and(query_param("per_page", "10"))
            .and(header_exists("Authorization"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "items": [
                    { "repository": { "full_name": "acme/demo" } }
                ]
            })))
            .mount(&srv)
            .await;

        let client = Client::builder().build().unwrap();
        let urls =
            discover_github_card_urls_at(&client, &srv.uri(), "tok", "filename:agent.json", 10)
                .await
                .unwrap();
        assert_eq!(urls.len(), 2);
        assert!(urls.iter().any(|u| u.contains("acme/demo/main")));
        assert!(urls.iter().any(|u| u.contains("acme/demo/master")));
    }

    #[tokio::test]
    async fn maps_403_to_rate_limit_style_error() {
        let srv = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/search/code"))
            .respond_with(ResponseTemplate::new(403).set_body_string("API rate limit exceeded"))
            .mount(&srv)
            .await;

        let client = Client::builder().build().unwrap();
        let err = discover_github_card_urls_at(&client, &srv.uri(), "tok", "q", 5)
            .await
            .unwrap_err();
        let s = format!("{err:#}");
        assert!(s.contains("rate-limited") || s.contains("forbidden"), "{s}");
    }
}
