//! Table-driven cases from `url_policy_cases.json` (default HTTPS policy, no localhost exceptions).

use agentrank_crawl_policy::{validate_outbound_url, UrlPolicyError};
use serde::Deserialize;
use url::Url;

#[derive(Deserialize)]
struct Case {
    url: String,
    expect: String,
}

#[test]
fn table_default_policy() {
    let data = include_str!("url_policy_cases.json");
    let cases: Vec<Case> = serde_json::from_str(data).expect("json");
    for c in cases {
        let u = Url::parse(&c.url).expect("parse url");
        let got = validate_outbound_url(&u, false, false);
        match c.expect.as_str() {
            "ok" => assert!(got.is_ok(), "case {}: expected ok, got {got:?}", c.url),
            "scheme" => assert!(
                matches!(got, Err(UrlPolicyError::SchemeNotAllowed)),
                "case {}: expected scheme, got {got:?}",
                c.url
            ),
            "blocked_ip" => assert!(
                matches!(got, Err(UrlPolicyError::BlockedIp(_))),
                "case {}: expected blocked_ip, got {got:?}",
                c.url
            ),
            "userinfo" => assert!(
                matches!(got, Err(UrlPolicyError::UserinfoNotAllowed)),
                "case {}: expected userinfo, got {got:?}",
                c.url
            ),
            "blocked_host" => assert!(
                matches!(got, Err(UrlPolicyError::BlockedHost(_))),
                "case {}: expected blocked_host, got {got:?}",
                c.url
            ),
            other => panic!("unknown expect={other} for {}", c.url),
        }
    }
}
