//! Pluggable **registry** / seed sources that emit Agent Card URLs for the frontier.

use async_trait::async_trait;
use reqwest::Client;
use serde::Deserialize;
use thiserror::Error;

/// A URL to enqueue (card document or discoverable entry).
#[derive(Debug, Clone)]
pub struct DiscoveredUrl {
    pub url: String,
    pub priority: f64,
}

#[derive(Debug, Error)]
pub enum RegistryError {
    #[error("http: {0}")]
    Http(#[from] reqwest::Error),
    #[error("json: {0}")]
    Json(#[from] serde_json::Error),
    #[error("invalid feed: {0}")]
    InvalidFeed(String),
}

#[async_trait]
pub trait RegistrySource: Send + Sync {
    fn source_name(&self) -> &'static str;
    async fn discover(&self, client: &Client) -> Result<Vec<DiscoveredUrl>, RegistryError>;
}

/// Small built-in list for demos and smoke tests (extend in code or replace with HTTP feeds).
pub struct BuiltinDemoSeed;

#[async_trait]
impl RegistrySource for BuiltinDemoSeed {
    fn source_name(&self) -> &'static str {
        "builtin_demo_seed"
    }

    async fn discover(&self, _client: &Client) -> Result<Vec<DiscoveredUrl>, RegistryError> {
        Ok(vec![DiscoveredUrl {
            url: "https://pronox-public-agent.up.railway.app/.well-known/agent-card.json".into(),
            priority: 2.0,
        }])
    }
}

/// `GET` JSON body shaped as `{ "urls": [ "https://...", ... ] }` or top-level array.
pub struct HttpJsonUrlFeed {
    pub feed_url: String,
    pub default_priority: f64,
}

#[derive(Deserialize)]
struct UrlsWrapper {
    urls: Vec<String>,
}

#[async_trait]
impl RegistrySource for HttpJsonUrlFeed {
    fn source_name(&self) -> &'static str {
        "http_json_urls"
    }

    async fn discover(&self, client: &Client) -> Result<Vec<DiscoveredUrl>, RegistryError> {
        let resp = client.get(&self.feed_url).send().await?;
        let status = resp.status();
        if !status.is_success() {
            return Err(RegistryError::InvalidFeed(format!(
                "status {status} for {}",
                self.feed_url
            )));
        }
        let text = resp.text().await?;
        let urls: Vec<String> = if let Ok(w) = serde_json::from_str::<UrlsWrapper>(&text) {
            w.urls
        } else if let Ok(arr) = serde_json::from_str::<Vec<String>>(&text) {
            arr
        } else {
            return Err(RegistryError::InvalidFeed(
                "expected {\"urls\":[...]} or [\"...\"]".into(),
            ));
        };
        let p = self.default_priority;
        Ok(urls
            .into_iter()
            .map(|url| DiscoveredUrl { url, priority: p })
            .collect())
    }
}

/// Load same JSON formats as [`HttpJsonUrlFeed`] from a local path (CI fixtures, air-gapped).
pub struct StaticJsonFile {
    pub path: std::path::PathBuf,
    pub default_priority: f64,
}

#[async_trait]
impl RegistrySource for StaticJsonFile {
    fn source_name(&self) -> &'static str {
        "static_json_file"
    }

    async fn discover(&self, _client: &Client) -> Result<Vec<DiscoveredUrl>, RegistryError> {
        let text = tokio::fs::read_to_string(&self.path)
            .await
            .map_err(|e| RegistryError::InvalidFeed(e.to_string()))?;
        let urls: Vec<String> = if let Ok(w) = serde_json::from_str::<UrlsWrapper>(&text) {
            w.urls
        } else if let Ok(arr) = serde_json::from_str::<Vec<String>>(&text) {
            arr
        } else {
            return Err(RegistryError::InvalidFeed(
                "expected {\"urls\":[...]} or [\"...\"]".into(),
            ));
        };
        let p = self.default_priority;
        Ok(urls
            .into_iter()
            .map(|url| DiscoveredUrl { url, priority: p })
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn static_json_file_parses_top_level_array() {
        use std::time::{SystemTime, UNIX_EPOCH};
        let n = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let p = std::env::temp_dir().join(format!("agentrank_registry_static_{n}.json"));
        tokio::fs::write(
            &p,
            br#"["https://a.example/card.json","https://b.example/card.json"]"#,
        )
        .await
        .unwrap();
        let client = Client::builder().build().unwrap();
        let src = StaticJsonFile {
            path: p.clone(),
            default_priority: 1.5,
        };
        let out = src.discover(&client).await.unwrap();
        assert_eq!(out.len(), 2);
        assert_eq!(out[0].priority, 1.5);
        let _ = tokio::fs::remove_file(&p).await;
    }

    #[tokio::test]
    async fn http_json_feed_parses_wrapper() {
        let srv = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/feed"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "urls": ["https://a.example/x.json", "https://b.example/y.json"]
            })))
            .mount(&srv)
            .await;
        let client = Client::builder().build().unwrap();
        let src = HttpJsonUrlFeed {
            feed_url: format!("{}/feed", srv.uri()),
            default_priority: 3.0,
        };
        let out = src.discover(&client).await.unwrap();
        assert_eq!(out.len(), 2);
        assert_eq!(out[0].priority, 3.0);
    }

    #[tokio::test]
    async fn http_json_non_success_is_invalid_feed() {
        let srv = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/feed"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&srv)
            .await;
        let client = Client::builder().build().unwrap();
        let src = HttpJsonUrlFeed {
            feed_url: format!("{}/feed", srv.uri()),
            default_priority: 1.0,
        };
        let err = src.discover(&client).await.unwrap_err();
        let s = err.to_string();
        assert!(s.contains("404") || s.contains("status"), "{s}");
    }

    #[tokio::test]
    async fn http_json_500_is_invalid_feed() {
        let srv = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/feed"))
            .respond_with(ResponseTemplate::new(500))
            .mount(&srv)
            .await;
        let client = Client::builder().build().unwrap();
        let src = HttpJsonUrlFeed {
            feed_url: format!("{}/feed", srv.uri()),
            default_priority: 1.0,
        };
        assert!(src.discover(&client).await.is_err());
    }

    #[tokio::test]
    async fn http_json_malformed_body_errors() {
        let srv = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/feed"))
            .respond_with(ResponseTemplate::new(200).set_body_string("not json"))
            .mount(&srv)
            .await;
        let client = Client::builder().build().unwrap();
        let src = HttpJsonUrlFeed {
            feed_url: format!("{}/feed", srv.uri()),
            default_priority: 1.0,
        };
        let err = src.discover(&client).await.unwrap_err();
        assert!(
            matches!(err, RegistryError::InvalidFeed(_)),
            "expected InvalidFeed, got {err:?}"
        );
    }

    #[tokio::test]
    async fn http_json_empty_urls_array_ok() {
        let srv = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/feed"))
            .respond_with(ResponseTemplate::new(200).set_body_string(r#"{"urls":[]}"#))
            .mount(&srv)
            .await;
        let client = Client::builder().build().unwrap();
        let src = HttpJsonUrlFeed {
            feed_url: format!("{}/feed", srv.uri()),
            default_priority: 1.0,
        };
        assert!(src.discover(&client).await.unwrap().is_empty());
    }

    #[tokio::test]
    async fn http_json_duplicate_urls_preserved_in_vec() {
        let srv = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/feed"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "urls": ["https://dup.example/c", "https://dup.example/c"]
            })))
            .mount(&srv)
            .await;
        let client = Client::builder().build().unwrap();
        let src = HttpJsonUrlFeed {
            feed_url: format!("{}/feed", srv.uri()),
            default_priority: 1.0,
        };
        let out = src.discover(&client).await.unwrap();
        assert_eq!(out.len(), 2);
    }

    #[tokio::test]
    async fn builtin_demo_seed_non_empty() {
        let client = Client::builder().build().unwrap();
        let out = BuiltinDemoSeed.discover(&client).await.unwrap();
        assert_eq!(out.len(), 1);
        assert!(out[0].url.starts_with("https://"));
    }
}
