//! Enqueue related card URLs discovered from normalized `card_json`.

use agentrank_crawl_policy::validate_outbound_url;
use agentrank_frontier::{FrontierMeta, UrlFrontier};
use redis::aio::MultiplexedConnection;
use serde_json::Value;
use sqlx::types::Json;
use sqlx::PgPool;
use url::Url;
use uuid::Uuid;

use crate::IngestPolicy;

/// Extract `https://` URLs from JSON and enqueue valid outbound candidates.
pub async fn enqueue_card_links(
    pool: &PgPool,
    redis: &mut MultiplexedConnection,
    frontier: &UrlFrontier,
    agent_id: Uuid,
    policy: IngestPolicy,
) -> anyhow::Result<()> {
    let card_json: Option<Json<Value>> =
        sqlx::query_scalar(r#"SELECT card_json FROM agents WHERE id = $1"#)
            .bind(agent_id)
            .fetch_optional(pool)
            .await?;
    let Some(Json(card_json)) = card_json else {
        return Ok(());
    };

    let mut urls = Vec::new();
    collect_https_urls(&card_json, &mut urls);
    urls.sort();
    urls.dedup();

    for u in urls {
        let Ok(parsed) = Url::parse(&u) else {
            continue;
        };
        if validate_outbound_url(
            &parsed,
            policy.allow_http_localhost,
            policy.allow_loopback_https,
        )
        .is_err()
        {
            continue;
        }
        let m = FrontierMeta::new("card_expand");
        let _ = frontier.enqueue_with_meta(redis, &u, 0.5, &m).await;
        metrics::counter!("agentrank_card_expand_enqueue_total").increment(1);
    }
    Ok(())
}

/// Collect `https://` URLs from arbitrary JSON (for tests and frontier expansion).
pub(crate) fn collect_https_urls(v: &Value, out: &mut Vec<String>) {
    match v {
        Value::String(s) => {
            let t = s.trim();
            if t.starts_with("https://") && Url::parse(t).is_ok() {
                out.push(t.to_string());
            }
        }
        Value::Array(a) => {
            for x in a {
                collect_https_urls(x, out);
            }
        }
        Value::Object(o) => {
            for (_, x) in o {
                collect_https_urls(x, out);
            }
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::collect_https_urls;
    use serde_json::json;

    #[test]
    fn collects_nested_https_strings() {
        let v = json!({
            "url": "https://a.example/card",
            "nested": { "x": "https://b.example/x" },
            "skills": [{ "examples": ["https://c.example/e"] }]
        });
        let mut out = Vec::new();
        collect_https_urls(&v, &mut out);
        out.sort();
        out.dedup();
        assert_eq!(
            out,
            vec![
                "https://a.example/card",
                "https://b.example/x",
                "https://c.example/e"
            ]
        );
    }

    #[test]
    fn ignores_http_and_non_urls() {
        let v = json!({
            "a": "http://insecure.example",
            "b": "not a url",
            "c": "file:///etc/passwd",
            "d": "https://ok.example/y"
        });
        let mut out = Vec::new();
        collect_https_urls(&v, &mut out);
        assert_eq!(out, vec!["https://ok.example/y"]);
    }

    #[test]
    fn empty_object_no_panic() {
        let mut out = Vec::new();
        collect_https_urls(&json!({}), &mut out);
        assert!(out.is_empty());
    }
}
