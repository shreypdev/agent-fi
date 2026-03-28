//! Parse, validate, and normalize A2A Agent Card JSON for ingest.

mod error;
mod normalize;

pub use error::{ParseError, ParseErrorKind};
pub use normalize::sort_json_value_keys;

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use sha2::{Digest, Sha256};
use url::Url;

/// Successfully parsed and normalized agent card, ready for persistence mapping.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedAgentCard {
    /// Display name (trimmed, non-empty).
    pub name: String,
    /// Description (trimmed, non-empty).
    pub description: String,
    /// Agent A2A endpoint URL from the `url` field (trimmed).
    pub endpoint_url: String,
    /// Card `version` string mapped to `agents.protocol_version`.
    pub protocol_version: String,
    /// Stable id: `sha256:` + hex(SHA-256) of lowercase trimmed agent endpoint URL (`url` field).
    pub external_id: String,
    /// Final URL of the fetched card document (after redirects).
    pub canonical_url: String,
    /// Original request URL (before redirects), for `source_url`.
    pub source_url: String,
    /// Normalized JSON document (sorted object keys, trimmed strings).
    pub normalized_card: Value,
    /// SHA-256 hex of canonical JSON bytes of `normalized_card`.
    pub content_hash: String,
    /// `providers.primary_domain` — host of endpoint or provider org URL.
    pub primary_domain: String,
    /// Optional `providers.display_name` from `provider.organization`.
    pub provider_display_name: Option<String>,
}

/// Raw card JSON shape for deserialization before validation.
#[derive(Debug, Deserialize, Serialize)]
struct RawCard {
    name: Option<String>,
    description: Option<String>,
    version: Option<String>,
    url: Option<String>,
    provider: Option<RawProvider>,
    #[serde(flatten)]
    extra: Map<String, Value>,
}

#[derive(Debug, Deserialize, Serialize)]
struct RawProvider {
    organization: Option<String>,
    url: Option<String>,
    #[serde(flatten)]
    extra: Map<String, Value>,
}

/// Parse and validate Agent Card JSON bytes.
///
/// `source_url` is the URL that was requested; `final_fetch_url` is the final URL after redirects
/// (used as `canonical_url` for the card document).
pub fn parse_agent_card_bytes(
    bytes: &[u8],
    source_url: &str,
    final_fetch_url: &str,
) -> Result<ParsedAgentCard, ParseError> {
    let v: Value = serde_json::from_slice(bytes).map_err(|e| ParseError {
        kind: ParseErrorKind::InvalidJson(e.to_string()),
    })?;
    parse_agent_card_value(v, source_url, final_fetch_url)
}

fn parse_agent_card_value(
    v: Value,
    source_url: &str,
    final_fetch_url: &str,
) -> Result<ParsedAgentCard, ParseError> {
    if !v.is_object() {
        return Err(ParseError {
            kind: ParseErrorKind::NotJsonObject,
        });
    }

    let raw: RawCard = serde_json::from_value(v.clone()).map_err(|e| ParseError {
        kind: ParseErrorKind::InvalidJson(e.to_string()),
    })?;

    let name = require_str(&raw.name, "name")?;
    let description = require_str(&raw.description, "description")?;
    let endpoint_url = require_str(&raw.url, "url")?;
    let protocol_version = require_str(&raw.version, "version")?;

    validate_endpoint_url(&endpoint_url)?;
    validate_source_urls(source_url, final_fetch_url)?;

    let (primary_domain, provider_display_name) = resolve_provider(&raw.provider, &endpoint_url)?;

    let normalized_card = normalize::normalize_card_value(&v)?;
    let content_hash = content_hash_for_value(&normalized_card)?;
    let external_id = external_id_for_endpoint(&endpoint_url);

    Ok(ParsedAgentCard {
        name,
        description,
        endpoint_url,
        protocol_version,
        external_id,
        canonical_url: final_fetch_url.to_string(),
        source_url: source_url.to_string(),
        normalized_card,
        content_hash,
        primary_domain,
        provider_display_name,
    })
}

fn require_str(field: &Option<String>, name: &'static str) -> Result<String, ParseError> {
    let s = field.as_ref().ok_or(ParseError {
        kind: ParseErrorKind::MissingField(name),
    })?;
    let t = s.trim();
    if t.is_empty() {
        return Err(ParseError {
            kind: ParseErrorKind::EmptyField(name),
        });
    }
    Ok(t.to_string())
}

fn validate_endpoint_url(endpoint_url: &str) -> Result<(), ParseError> {
    let u = Url::parse(endpoint_url).map_err(|_| ParseError {
        kind: ParseErrorKind::InvalidEndpointUrl,
    })?;
    match u.scheme() {
        "http" | "https" => {}
        _ => {
            return Err(ParseError {
                kind: ParseErrorKind::InvalidEndpointScheme,
            });
        }
    }
    if u.host_str().is_none() {
        return Err(ParseError {
            kind: ParseErrorKind::InvalidEndpointUrl,
        });
    }
    Ok(())
}

fn validate_source_urls(source_url: &str, final_fetch_url: &str) -> Result<(), ParseError> {
    Url::parse(source_url).map_err(|_| ParseError {
        kind: ParseErrorKind::InvalidSourceUrl,
    })?;
    Url::parse(final_fetch_url).map_err(|_| ParseError {
        kind: ParseErrorKind::InvalidFinalUrl,
    })?;
    Ok(())
}

fn resolve_provider(
    provider: &Option<RawProvider>,
    endpoint_url: &str,
) -> Result<(String, Option<String>), ParseError> {
    let endpoint_host = Url::parse(endpoint_url)
        .ok()
        .and_then(|u| u.host_str().map(str::to_lowercase));

    let Some(host) = endpoint_host else {
        return Err(ParseError {
            kind: ParseErrorKind::InvalidEndpointUrl,
        });
    };

    match provider {
        None => Ok((host, None)),
        Some(p) => {
            let display = p
                .organization
                .as_ref()
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty());

            if let Some(ref u) = p.url {
                let trimmed = u.trim();
                if !trimmed.is_empty() {
                    let pu = Url::parse(trimmed).map_err(|_| ParseError {
                        kind: ParseErrorKind::InvalidProviderUrl,
                    })?;
                    let domain = pu
                        .host_str()
                        .ok_or(ParseError {
                            kind: ParseErrorKind::InvalidProviderUrl,
                        })?
                        .to_lowercase();
                    return Ok((domain, display));
                }
            }
            Ok((host, display))
        }
    }
}

fn normalized_endpoint_key(endpoint_url: &str) -> String {
    endpoint_url.trim().to_lowercase()
}

fn external_id_for_endpoint(endpoint_url: &str) -> String {
    let key = normalized_endpoint_key(endpoint_url);
    let mut h = Sha256::new();
    h.update(key.as_bytes());
    format!("sha256:{:x}", h.finalize())
}

fn content_hash_for_value(v: &Value) -> Result<String, ParseError> {
    let bytes = serde_json::to_vec(v).map_err(|e| ParseError {
        kind: ParseErrorKind::InvalidJson(e.to_string()),
    })?;
    let mut h = Sha256::new();
    h.update(&bytes);
    Ok(format!("{:x}", h.finalize()))
}

/// Build a minimal valid JSON object for tests (no normalization).
pub fn minimal_valid_card_json(name: &str, description: &str, version: &str, url: &str) -> String {
    format!(
        r#"{{"name":{0},"description":{1},"version":{2},"url":{3}}}"#,
        serde_json::to_string(name).unwrap(),
        serde_json::to_string(description).unwrap(),
        serde_json::to_string(version).unwrap(),
        serde_json::to_string(url).unwrap(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const SRC: &str = "https://example.com/.well-known/agent.json";
    const FINAL: &str = "https://example.com/.well-known/agent.json";

    fn parse(json: &str) -> Result<ParsedAgentCard, ParseError> {
        parse_agent_card_bytes(json.as_bytes(), SRC, FINAL)
    }

    #[test]
    fn minimal_valid() {
        let j = minimal_valid_card_json("A", "B", "1.0.0", "https://agent.example/a");
        let p = parse(&j).unwrap();
        assert_eq!(p.name, "A");
        assert_eq!(p.description, "B");
        assert_eq!(p.protocol_version, "1.0.0");
        assert_eq!(p.endpoint_url, "https://agent.example/a");
        assert!(p.external_id.starts_with("sha256:"));
        assert_eq!(p.primary_domain, "agent.example");
        assert!(p.content_hash.len() == 64);
    }

    #[test]
    fn trims_name_description_url_version() {
        let j = r#"{"name":"  x  ","description":"  d  ","version":" 1.2 ","url":" https://a.example/ "}"#;
        let p = parse(j).unwrap();
        assert_eq!(p.name, "x");
        assert_eq!(p.description, "d");
        assert_eq!(p.protocol_version, "1.2");
        assert_eq!(p.endpoint_url, "https://a.example/");
    }

    #[test]
    fn provider_organization_sets_display_name() {
        let j = r#"{"name":"N","description":"D","version":"1","url":"https://x.com/a","provider":{"organization":"Org Co"}}"#;
        let p = parse(j).unwrap();
        assert_eq!(p.provider_display_name.as_deref(), Some("Org Co"));
    }

    #[test]
    fn provider_url_sets_primary_domain() {
        let j = r#"{"name":"N","description":"D","version":"1","url":"https://x.com/a","provider":{"url":"https://corp.example/about"}}"#;
        let p = parse(j).unwrap();
        assert_eq!(p.primary_domain, "corp.example");
    }

    #[test]
    fn provider_empty_url_falls_back_to_endpoint_host() {
        let j = r#"{"name":"N","description":"D","version":"1","url":"https://api.foo.bar/rpc","provider":{"url":"   "}}"#;
        let p = parse(j).unwrap();
        assert_eq!(p.primary_domain, "api.foo.bar");
    }

    #[test]
    fn duplicate_skill_ids_allowed_in_raw_json() {
        let j = r#"{"name":"N","description":"D","version":"1","url":"https://a.com/x","skills":[{"id":"s","name":"a"},{"id":"s","name":"b"}]}"#;
        assert!(parse(j).is_ok());
    }

    #[test]
    fn unknown_top_level_keys_preserved_in_normalized() {
        let j =
            r#"{"name":"N","description":"D","version":"1","url":"https://a.com/x","custom":42}"#;
        let p = parse(j).unwrap();
        assert_eq!(p.normalized_card["custom"], serde_json::json!(42));
    }

    #[test]
    fn not_object_fails() {
        let e = parse_agent_card_bytes(b"[]", SRC, FINAL).unwrap_err();
        assert_eq!(e.kind, ParseErrorKind::NotJsonObject);
    }

    #[test]
    fn invalid_json_fails() {
        let e = parse_agent_card_bytes(b"{", SRC, FINAL).unwrap_err();
        assert!(matches!(e.kind, ParseErrorKind::InvalidJson(_)));
    }

    #[test]
    fn missing_name() {
        let j = r#"{"description":"D","version":"1","url":"https://a.com"}"#;
        assert_eq!(
            parse(j).unwrap_err().kind,
            ParseErrorKind::MissingField("name")
        );
    }

    #[test]
    fn missing_description() {
        let j = r#"{"name":"N","version":"1","url":"https://a.com"}"#;
        assert_eq!(
            parse(j).unwrap_err().kind,
            ParseErrorKind::MissingField("description")
        );
    }

    #[test]
    fn missing_version() {
        let j = r#"{"name":"N","description":"D","url":"https://a.com"}"#;
        assert_eq!(
            parse(j).unwrap_err().kind,
            ParseErrorKind::MissingField("version")
        );
    }

    #[test]
    fn missing_url() {
        let j = r#"{"name":"N","description":"D","version":"1"}"#;
        assert_eq!(
            parse(j).unwrap_err().kind,
            ParseErrorKind::MissingField("url")
        );
    }

    #[test]
    fn empty_name() {
        let j = r#"{"name":"  ","description":"D","version":"1","url":"https://a.com"}"#;
        assert_eq!(
            parse(j).unwrap_err().kind,
            ParseErrorKind::EmptyField("name")
        );
    }

    #[test]
    fn empty_description() {
        let j = r#"{"name":"N","description":" \t ","version":"1","url":"https://a.com"}"#;
        assert_eq!(
            parse(j).unwrap_err().kind,
            ParseErrorKind::EmptyField("description")
        );
    }

    #[test]
    fn empty_version() {
        let j = r#"{"name":"N","description":"D","version":"","url":"https://a.com"}"#;
        assert_eq!(
            parse(j).unwrap_err().kind,
            ParseErrorKind::EmptyField("version")
        );
    }

    #[test]
    fn empty_url() {
        let j = r#"{"name":"N","description":"D","version":"1","url":"  "}"#;
        assert_eq!(
            parse(j).unwrap_err().kind,
            ParseErrorKind::EmptyField("url")
        );
    }

    #[test]
    fn bad_endpoint_scheme_ftp() {
        let j = r#"{"name":"N","description":"D","version":"1","url":"ftp://x.com"}"#;
        assert_eq!(
            parse(j).unwrap_err().kind,
            ParseErrorKind::InvalidEndpointScheme
        );
    }

    #[test]
    fn bad_endpoint_not_url() {
        let j = r#"{"name":"N","description":"D","version":"1","url":"not a url"}"#;
        assert_eq!(
            parse(j).unwrap_err().kind,
            ParseErrorKind::InvalidEndpointUrl
        );
    }

    #[test]
    fn http_endpoint_ok() {
        let j = r#"{"name":"N","description":"D","version":"1","url":"http://localhost:8080/a"}"#;
        assert!(parse(j).is_ok());
    }

    #[test]
    fn https_endpoint_ok() {
        let j = minimal_valid_card_json("N", "D", "1", "https://h.example/path");
        assert!(parse(&j).is_ok());
    }

    #[test]
    fn invalid_source_url() {
        let v: Value =
            serde_json::from_str(&minimal_valid_card_json("N", "D", "1", "https://a.com")).unwrap();
        let e = parse_agent_card_value(v, ":::bad", FINAL).unwrap_err();
        assert_eq!(e.kind, ParseErrorKind::InvalidSourceUrl);
    }

    #[test]
    fn invalid_final_url() {
        let v: Value =
            serde_json::from_str(&minimal_valid_card_json("N", "D", "1", "https://a.com")).unwrap();
        let e = parse_agent_card_value(v, SRC, "bad").unwrap_err();
        assert_eq!(e.kind, ParseErrorKind::InvalidFinalUrl);
    }

    #[test]
    fn provider_invalid_url() {
        let j = r#"{"name":"N","description":"D","version":"1","url":"https://a.com/x","provider":{"url":"noturl"}}"#;
        assert_eq!(
            parse(j).unwrap_err().kind,
            ParseErrorKind::InvalidProviderUrl
        );
    }

    #[test]
    fn canonical_and_source_preserved() {
        let j = minimal_valid_card_json("N", "D", "1", "https://e.com");
        let p = parse_agent_card_bytes(
            j.as_bytes(),
            "https://start/here",
            "https://final/card.json",
        )
        .unwrap();
        assert_eq!(p.source_url, "https://start/here");
        assert_eq!(p.canonical_url, "https://final/card.json");
    }

    #[test]
    fn external_id_stable() {
        let j = minimal_valid_card_json("N", "D", "1", "https://E.COM/PATH");
        let p1 = parse(&j).unwrap();
        let j2 = minimal_valid_card_json("Other", "Other", "2", "https://e.com/path");
        let p2 = parse(&j2).unwrap();
        assert_eq!(p1.external_id, p2.external_id);
    }

    #[test]
    fn content_hash_stable_for_same_semantics() {
        let j1 = r#"{"name":"N","description":"D","version":"1","url":"https://a.com","x":1}"#;
        let j2 = r#"{"x":1,"name":"N","description":"D","version":"1","url":"https://a.com"}"#;
        let p1 = parse(j1).unwrap();
        let p2 = parse(j2).unwrap();
        assert_eq!(p1.content_hash, p2.content_hash);
    }

    #[test]
    fn opus_style_card_parses() {
        let j = include_str!("../tests/fixtures/opus_style_card.json");
        let p = parse(j).unwrap();
        assert_eq!(p.name, "FinancialAnalysisAgent");
        assert!(p.description.contains("portfolio"));
        assert_eq!(
            p.endpoint_url,
            "https://agents.acmecorp.com/financial-analysis"
        );
        assert_eq!(p.protocol_version, "2.4.1");
    }

    #[test]
    fn unicode_trim_name() {
        let j = r#"{"name":"  你好  ","description":"D","version":"1","url":"https://a.com"}"#;
        let p = parse(j).unwrap();
        assert_eq!(p.name, "你好");
    }

    #[test]
    fn nested_strings_trimmed_in_normalize() {
        let j = r#"{"name":"N","description":"D","version":"1","url":"https://a.com","skills":[{"id":" i ","name":" n "}]}"#;
        let p = parse(j).unwrap();
        let skills = p.normalized_card["skills"].as_array().unwrap();
        assert_eq!(skills[0]["id"], "i");
        assert_eq!(skills[0]["name"], "n");
    }

    #[test]
    fn name_wrong_type_fails() {
        let j = r#"{"name":1,"description":"D","version":"1","url":"https://a.com"}"#;
        assert!(matches!(
            parse(j).unwrap_err().kind,
            ParseErrorKind::InvalidJson(_)
        ));
    }

    #[test]
    fn url_wrong_type_fails() {
        let j = r#"{"name":"N","description":"D","version":"1","url":true}"#;
        assert!(matches!(
            parse(j).unwrap_err().kind,
            ParseErrorKind::InvalidJson(_)
        ));
    }

    #[test]
    fn version_number_in_json_invalid() {
        let j = r#"{"name":"N","description":"D","version":1,"url":"https://a.com"}"#;
        assert!(matches!(
            parse(j).unwrap_err().kind,
            ParseErrorKind::InvalidJson(_)
        ));
    }

    #[test]
    fn boolean_description_invalid() {
        let j = r#"{"name":"N","description":false,"version":"1","url":"https://a.com"}"#;
        assert!(matches!(
            parse(j).unwrap_err().kind,
            ParseErrorKind::InvalidJson(_)
        ));
    }

    #[test]
    fn null_name_invalid() {
        let j = r#"{"name":null,"description":"D","version":"1","url":"https://a.com"}"#;
        assert_eq!(
            parse(j).unwrap_err().kind,
            ParseErrorKind::MissingField("name")
        );
    }

    #[test]
    fn array_instead_of_object_invalid() {
        let e = parse_agent_card_bytes(b"[1,2,3]", SRC, FINAL).unwrap_err();
        assert_eq!(e.kind, ParseErrorKind::NotJsonObject);
    }

    #[test]
    fn string_instead_of_object_invalid() {
        let e = parse_agent_card_bytes(br#""hello""#, SRC, FINAL).unwrap_err();
        assert_eq!(e.kind, ParseErrorKind::NotJsonObject);
    }

    #[test]
    fn number_instead_of_object_invalid() {
        let e = parse_agent_card_bytes(b"42", SRC, FINAL).unwrap_err();
        assert_eq!(e.kind, ParseErrorKind::NotJsonObject);
    }

    #[test]
    fn null_root_invalid() {
        let e = parse_agent_card_bytes(b"null", SRC, FINAL).unwrap_err();
        assert_eq!(e.kind, ParseErrorKind::NotJsonObject);
    }

    #[test]
    fn empty_object_missing_fields() {
        let e = parse("{}").unwrap_err();
        assert_eq!(e.kind, ParseErrorKind::MissingField("name"));
    }

    #[test]
    fn skills_array_with_object_entries() {
        let j = r#"{"name":"N","description":"D","version":"1","url":"https://a.com","skills":[{"id":"a","name":"A","description":"d","tags":["t"]}]}"#;
        assert!(parse(j).is_ok());
    }

    #[test]
    fn authentication_block_preserved() {
        let j = r#"{"name":"N","description":"D","version":"1","url":"https://a.com","authentication":{"schemes":[]}}"#;
        let p = parse(j).unwrap();
        assert!(p.normalized_card.get("authentication").is_some());
    }

    #[test]
    fn capabilities_preserved() {
        let j = r#"{"name":"N","description":"D","version":"1","url":"https://a.com","capabilities":{"streaming":true}}"#;
        let p = parse(j).unwrap();
        assert_eq!(
            p.normalized_card["capabilities"]["streaming"],
            Value::Bool(true)
        );
    }

    #[test]
    fn default_input_modes_array() {
        let j = r#"{"name":"N","description":"D","version":"1","url":"https://a.com","defaultInputModes":["text/plain"]}"#;
        let p = parse(j).unwrap();
        assert_eq!(p.normalized_card["defaultInputModes"][0], "text/plain");
    }

    #[test]
    fn endpoint_with_query_ok() {
        let j = minimal_valid_card_json("N", "D", "1", "https://a.com/x?q=1");
        assert!(parse(&j).is_ok());
    }

    #[test]
    fn endpoint_with_port_ok() {
        let j = minimal_valid_card_json("N", "D", "1", "https://a.com:9443/api");
        assert!(parse(&j).is_ok());
    }

    #[test]
    fn provider_org_empty_string_no_display() {
        let j = r#"{"name":"N","description":"D","version":"1","url":"https://x.com/a","provider":{"organization":"  "}}"#;
        let p = parse(j).unwrap();
        assert!(p.provider_display_name.is_none());
    }

    #[test]
    fn sort_json_empty_object() {
        assert_eq!(
            sort_json_value_keys(&Value::Object(Map::new())),
            Value::Object(Map::new())
        );
    }

    #[test]
    fn external_id_differs_for_different_endpoints() {
        let a = parse(&minimal_valid_card_json("N", "D", "1", "https://a.com/1")).unwrap();
        let b = parse(&minimal_valid_card_json("N", "D", "1", "https://a.com/2")).unwrap();
        assert_ne!(a.external_id, b.external_id);
    }

    #[test]
    fn content_hash_changes_when_optional_field_added() {
        let base = r#"{"name":"N","description":"D","version":"1","url":"https://a.com"}"#;
        let extra = r#"{"name":"N","description":"D","version":"1","url":"https://a.com","z":1}"#;
        assert_ne!(
            parse(base).unwrap().content_hash,
            parse(extra).unwrap().content_hash
        );
    }

    #[test]
    fn large_description_ok() {
        let big = "x".repeat(10_000);
        let j = minimal_valid_card_json("N", &big, "1", "https://a.com");
        assert!(parse(&j).is_ok());
    }

    #[test]
    fn json_escape_in_name() {
        let j = r#"{"name":"Line1\nLine2","description":"D","version":"1","url":"https://a.com"}"#;
        let p = parse(j).unwrap();
        assert_eq!(p.name, "Line1\nLine2");
    }

    #[test]
    fn provider_extra_fields_roundtrip_normalize() {
        let j = r#"{"name":"N","description":"D","version":"1","url":"https://a.com","provider":{"organization":"O","url":"https://p.com","contactEmail":"a@b.c"}}"#;
        let p = parse(j).unwrap();
        assert!(p.normalized_card["provider"].get("contactEmail").is_some());
    }

    #[test]
    fn sort_keys_deterministic() {
        let v1 = serde_json::json!({"b": 1, "a": 2});
        let v2 = serde_json::json!({"a": 2, "b": 1});
        assert_eq!(
            serde_json::to_string(&sort_json_value_keys(&v1)).unwrap(),
            serde_json::to_string(&sort_json_value_keys(&v2)).unwrap()
        );
    }

    #[test]
    fn whitespace_only_description_rejected() {
        let j = r#"{"name":"N","description":"\n\t  ","version":"1","url":"https://a.com"}"#;
        assert_eq!(
            parse(j).unwrap_err().kind,
            ParseErrorKind::EmptyField("description")
        );
    }

    #[test]
    fn ipv6_localhost_endpoint_ok() {
        let j = minimal_valid_card_json("N", "D", "1", "http://[::1]:3000/a");
        assert!(parse(&j).is_ok());
    }

    #[test]
    fn deeply_nested_arrays_normalized() {
        let j = r#"{"name":"N","description":"D","version":"1","url":"https://a.com","a":[[["  x  "]]]}"#;
        let p = parse(j).unwrap();
        assert_eq!(p.normalized_card["a"][0][0][0], "x");
    }

    #[test]
    fn empty_skills_array() {
        let j = r#"{"name":"N","description":"D","version":"1","url":"https://a.com","skills":[]}"#;
        assert!(parse(j).is_ok());
    }

    #[test]
    fn skill_missing_id_still_valid_card() {
        let j = r#"{"name":"N","description":"D","version":"1","url":"https://a.com","skills":[{"name":"only"}]}"#;
        assert!(parse(j).is_ok());
    }
}
