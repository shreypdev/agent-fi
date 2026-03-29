//! Paginated registry API connectors (Week 8).

use crate::{DiscoveredUrl, RegistryError, RegistrySource};
use async_trait::async_trait;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::Client;
use serde::Deserialize;
use serde_json::Value;

fn json_path_array<'a>(v: &'a Value, path: &str) -> Result<&'a Vec<Value>, RegistryError> {
    let mut cur = v;
    if path.is_empty() || path == "." {
        return cur
            .as_array()
            .ok_or_else(|| RegistryError::InvalidFeed("expected array at root".into()));
    }
    for part in path.split('.').filter(|p| !p.is_empty()) {
        cur = cur
            .get(part)
            .ok_or_else(|| RegistryError::InvalidFeed(format!("missing path {part}")))?;
    }
    cur.as_array()
        .ok_or_else(|| RegistryError::InvalidFeed("path not array".into()))
}

/// PulseMCP Sub-Registry style API (`/api/v0.1/servers` or similar).
pub struct PulseMcpRegistry {
    pub api_base: String,
    pub tenant_id: String,
    pub api_key: String,
    pub default_priority: f64,
}

#[derive(Deserialize)]
struct PulsePage {
    #[serde(default)]
    servers: Vec<PulseServer>,
    #[serde(default)]
    next_cursor: Option<String>,
}

#[derive(Deserialize)]
struct PulseServer {
    #[serde(default)]
    card_url: Option<String>,
    #[serde(default)]
    url: Option<String>,
}

#[async_trait]
impl RegistrySource for PulseMcpRegistry {
    fn source_name(&self) -> &'static str {
        "pulsemcp_registry"
    }

    async fn discover(&self, client: &Client) -> Result<Vec<DiscoveredUrl>, RegistryError> {
        let (v, _) = self.discover_paginated(client, None).await?;
        Ok(v)
    }

    async fn discover_paginated(
        &self,
        client: &Client,
        cursor: Option<String>,
    ) -> Result<(Vec<DiscoveredUrl>, Option<String>), RegistryError> {
        let mut url = format!("{}/api/v0.1/servers", self.api_base.trim_end_matches('/'));
        if let Some(c) = cursor {
            let sep = if url.contains('?') { '&' } else { '?' };
            url = format!("{url}{sep}cursor={}", urlencoding::encode(&c));
        }
        let mut headers = HeaderMap::new();
        headers.insert(
            HeaderName::from_static("x-tenant-id"),
            HeaderValue::from_str(&self.tenant_id)
                .map_err(|e| RegistryError::InvalidFeed(e.to_string()))?,
        );
        headers.insert(
            HeaderName::from_static("x-api-key"),
            HeaderValue::from_str(&self.api_key)
                .map_err(|e| RegistryError::InvalidFeed(e.to_string()))?,
        );
        let resp = client.get(&url).headers(headers).send().await?;
        if !resp.status().is_success() {
            return Err(RegistryError::InvalidFeed(format!(
                "pulse {}",
                resp.status()
            )));
        }
        let page: PulsePage = resp.json().await?;
        let mut out = Vec::new();
        for s in page.servers {
            let u = s.card_url.or(s.url);
            if let Some(url) = u {
                if !url.is_empty() {
                    out.push(DiscoveredUrl::new(
                        url,
                        self.default_priority,
                        self.source_name(),
                    ));
                }
            }
        }
        Ok((out, page.next_cursor))
    }
}

/// AgentVerse / Fetch.ai style list.
pub struct AgentVerseRegistry {
    pub api_base: String,
    pub api_key: String,
    pub default_priority: f64,
}

#[derive(Deserialize)]
struct AvPage {
    #[serde(default)]
    agents: Vec<AvAgent>,
    #[serde(default)]
    next: Option<String>,
}

#[derive(Deserialize)]
struct AvAgent {
    #[serde(default)]
    card_url: Option<String>,
}

#[async_trait]
impl RegistrySource for AgentVerseRegistry {
    fn source_name(&self) -> &'static str {
        "agentverse_registry"
    }

    async fn discover(&self, client: &Client) -> Result<Vec<DiscoveredUrl>, RegistryError> {
        let (v, _) = self.discover_paginated(client, None).await?;
        Ok(v)
    }

    async fn discover_paginated(
        &self,
        client: &Client,
        cursor: Option<String>,
    ) -> Result<(Vec<DiscoveredUrl>, Option<String>), RegistryError> {
        let mut url = format!("{}/api/v1/agents", self.api_base.trim_end_matches('/'));
        if let Some(c) = cursor {
            let sep = if url.contains('?') { '&' } else { '?' };
            url = format!("{url}{sep}page_token={}", urlencoding::encode(&c));
        }
        let resp = client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;
        if !resp.status().is_success() {
            return Err(RegistryError::InvalidFeed(format!(
                "agentverse {}",
                resp.status()
            )));
        }
        let page: AvPage = resp.json().await?;
        let mut out = Vec::new();
        for a in page.agents {
            if let Some(u) = a.card_url {
                if !u.is_empty() {
                    out.push(DiscoveredUrl::new(
                        u,
                        self.default_priority,
                        self.source_name(),
                    ));
                }
            }
        }
        Ok((out, page.next))
    }
}

/// mcp.so HTML/API fixture-oriented connector (expects JSON list in tests).
pub struct McpSoRegistry {
    pub api_url: String,
    pub default_priority: f64,
}

#[derive(Deserialize)]
struct McpSoEntry {
    #[serde(default)]
    card_url: Option<String>,
}

#[async_trait]
impl RegistrySource for McpSoRegistry {
    fn source_name(&self) -> &'static str {
        "mcpso_registry"
    }

    async fn discover(&self, client: &Client) -> Result<Vec<DiscoveredUrl>, RegistryError> {
        let (v, _) = self.discover_paginated(client, None).await?;
        Ok(v)
    }

    async fn discover_paginated(
        &self,
        client: &Client,
        cursor: Option<String>,
    ) -> Result<(Vec<DiscoveredUrl>, Option<String>), RegistryError> {
        if cursor.is_some() {
            return Ok((vec![], None));
        }
        let resp = client.get(&self.api_url).send().await?;
        if !resp.status().is_success() {
            return Err(RegistryError::InvalidFeed(format!(
                "mcpso {}",
                resp.status()
            )));
        }
        let v: Value = resp.json().await?;
        let arr = v
            .get("entries")
            .and_then(|x| x.as_array())
            .or_else(|| v.as_array())
            .ok_or_else(|| RegistryError::InvalidFeed("expected entries array".into()))?;
        let mut out = Vec::new();
        for x in arr {
            let u = x
                .get("card_url")
                .and_then(|c| c.as_str())
                .map(String::from)
                .or_else(|| {
                    serde_json::from_value::<McpSoEntry>(x.clone())
                        .ok()
                        .and_then(|e| e.card_url)
                });
            if let Some(url) = u {
                if !url.is_empty() {
                    out.push(DiscoveredUrl::new(
                        url,
                        self.default_priority,
                        self.source_name(),
                    ));
                }
            }
        }
        Ok((out, None))
    }
}

/// Config-driven JSON registry (Smithery / Glama-style).
pub struct GenericJsonRegistry {
    pub api_url: String,
    pub auth_header: Option<(String, String)>,
    pub page_param: String,
    /// Dot path to array of objects, e.g. `data.items`
    pub entries_path: String,
    pub card_url_field: String,
    pub default_priority: f64,
}

#[async_trait]
impl RegistrySource for GenericJsonRegistry {
    fn source_name(&self) -> &'static str {
        "generic_json_registry"
    }

    async fn discover(&self, client: &Client) -> Result<Vec<DiscoveredUrl>, RegistryError> {
        let (v, _) = self.discover_paginated(client, None).await?;
        Ok(v)
    }

    async fn discover_paginated(
        &self,
        client: &Client,
        cursor: Option<String>,
    ) -> Result<(Vec<DiscoveredUrl>, Option<String>), RegistryError> {
        let mut url = self.api_url.clone();
        if let Some(c) = cursor {
            let sep = if url.contains('?') { '&' } else { '?' };
            url = format!("{url}{sep}{}={}", self.page_param, urlencoding::encode(&c));
        }
        let mut req = client.get(&url);
        if let Some((k, v)) = &self.auth_header {
            req = req.header(k.as_str(), v.as_str());
        }
        let resp = req.send().await?;
        if !resp.status().is_success() {
            return Err(RegistryError::InvalidFeed(format!(
                "generic {}",
                resp.status()
            )));
        }
        let root: Value = resp.json().await?;
        let arr = json_path_array(&root, &self.entries_path)?;
        let field = if self.card_url_field.trim().is_empty() {
            "card_url"
        } else {
            self.card_url_field.as_str()
        };
        let mut out = Vec::new();
        for x in arr {
            let u = x
                .get(field)
                .and_then(|c| c.as_str())
                .map(String::from)
                .or_else(|| x.get("url").and_then(|c| c.as_str()).map(String::from));
            if let Some(url) = u {
                if !url.is_empty() {
                    out.push(DiscoveredUrl::new(
                        url,
                        self.default_priority,
                        self.source_name(),
                    ));
                }
            }
        }
        let next = root
            .get("next_cursor")
            .and_then(|c| c.as_str())
            .map(String::from)
            .or_else(|| root.get("next").and_then(|c| c.as_str()).map(String::from));
        Ok((out, next))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn pulsemcp_one_page_with_cursor() {
        let srv = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/api/v0.1/servers"))
            .and(header("x-tenant-id", "t1"))
            .and(header("x-api-key", "k1"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "servers": [
                    {"card_url": "https://a.example/c1.json"},
                    {"url": "https://b.example/c2.json"}
                ],
                "next_cursor": null
            })))
            .mount(&srv)
            .await;

        let client = Client::builder().build().unwrap();
        let src = PulseMcpRegistry {
            api_base: srv.uri().to_string(),
            tenant_id: "t1".into(),
            api_key: "k1".into(),
            default_priority: 1.0,
        };
        let (p1, n1) = src.discover_paginated(&client, None).await.unwrap();
        assert_eq!(p1.len(), 2);
        assert!(n1.is_none());
    }

    #[test]
    fn json_path_nested() {
        let v = serde_json::json!({"data": {"items": [{"card_url": "https://x.example/c"}]}});
        let arr = json_path_array(&v, "data.items").unwrap();
        assert_eq!(arr.len(), 1);
    }
}
