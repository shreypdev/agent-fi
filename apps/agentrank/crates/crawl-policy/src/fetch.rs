//! Fetch `robots.txt` from an origin using a shared [`reqwest::Client`].

use crate::cache::RobotsCache;
use crate::url_policy::{validate_outbound_url, UrlPolicyError};
use reqwest::Client;
use thiserror::Error;
use url::Url;

#[derive(Debug, Error)]
pub enum RobotsFetchError {
    #[error("URL policy: {0}")]
    Policy(#[from] UrlPolicyError),
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("robots cache inconsistency after refresh")]
    CacheMiss,
}

/// Origin key for cache: `"{scheme}://{host}"` without path (lowercase host).
pub fn origin_key_from_url(url: &Url) -> Option<String> {
    let host = url.host_str()?.to_ascii_lowercase();
    Some(format!("{}://{}", url.scheme(), host))
}

/// Build `{origin}/robots.txt` from a card URL's origin.
pub fn robots_url_for_card_url(card_url: &Url) -> Option<Url> {
    let origin = card_url.origin().ascii_serialization();
    if origin == "null" {
        return None;
    }
    Url::parse(&format!("{}/robots.txt", origin.trim_end_matches('/'))).ok()
}

/// Fetch robots.txt for the origin of `target_url`, update cache, return cached view.
pub async fn refresh_robots_for_url(
    client: &Client,
    cache: &RobotsCache,
    target_url: &Url,
    allow_http_localhost: bool,
    allow_loopback_https: bool,
) -> Result<crate::cache::CachedRobots, RobotsFetchError> {
    let Some(origin_key) = origin_key_from_url(target_url) else {
        return Ok(crate::cache::CachedRobots::Missing {
            fetched_at: std::time::Instant::now(),
        });
    };

    if let Some(c) = cache.get(&origin_key) {
        return Ok(c);
    }

    let robots_u = robots_url_for_card_url(target_url)
        .ok_or_else(|| UrlPolicyError::InvalidUrl("cannot derive origin for robots.txt".into()))?;
    validate_outbound_url(&robots_u, allow_http_localhost, allow_loopback_https)?;

    let resp = client.get(robots_u.clone()).send().await?;
    let status = resp.status();
    if status == reqwest::StatusCode::NOT_FOUND || status == reqwest::StatusCode::UNAUTHORIZED {
        cache.insert_missing(origin_key.clone());
    } else if !status.is_success() {
        // Conservative: treat as missing for crawl (don't block on 500)
        cache.insert_missing(origin_key.clone());
    } else {
        let body = resp.text().await.unwrap_or_default();
        cache.insert_rules(origin_key.clone(), &body);
    }

    cache.get(&origin_key).ok_or(RobotsFetchError::CacheMiss)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::RobotsCache;
    use std::time::Duration;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn second_fetch_uses_cache_no_extra_http() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/robots.txt"))
            .respond_with(
                ResponseTemplate::new(200).set_body_string("User-agent: *\nDisallow: /secret\n"),
            )
            .expect(1)
            .mount(&server)
            .await;

        let client = Client::builder()
            .user_agent("AgentBot/test")
            .build()
            .unwrap();
        let cache = RobotsCache::new(Duration::from_secs(3600), Duration::from_secs(3600));
        let card = Url::parse(&format!("{}/.well-known/agent.json", server.uri())).unwrap();

        let r1 = refresh_robots_for_url(&client, &cache, &card, true, false)
            .await
            .unwrap();
        assert!(!r1.is_allowed("AgentBot/1", "/secret"));
        assert!(r1.is_allowed("AgentBot/1", "/public"));

        let r2 = refresh_robots_for_url(&client, &cache, &card, true, false)
            .await
            .unwrap();
        assert!(!r2.is_allowed("AgentBot/1", "/secret"));
    }
}
