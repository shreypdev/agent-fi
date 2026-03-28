//! Property-based checks: no panics, round-trip invariants for structurally valid cards.

use agentrank_card::parse_agent_card_bytes;
use proptest::prelude::*;
use serde_json::json;

const SRC: &str = "https://fixture.example/.well-known/agent.json";
const FINAL: &str = "https://fixture.example/.well-known/agent.json";

proptest! {
    #![proptest_config(ProptestConfig::with_cases(256))]

    /// Parser must never panic on arbitrary bytes (malicious or random).
    #[test]
    fn parse_never_panics(bytes in prop::collection::vec(any::<u8>(), 0..12_288)) {
        let _ = parse_agent_card_bytes(&bytes, SRC, FINAL);
    }

    /// Any generated minimal valid card parses; identities match after trim where applicable.
    #[test]
    fn minimal_valid_cards_parse(
        name in "[a-z][a-z0-9 _-]{0,38}",
        description in "[a-zA-Z0-9][a-zA-Z0-9 .,;:'\"!?_-]{0,180}",
        version in "1|1\\.0|2\\.0\\.1|0\\.0\\.1-SNAPSHOT",
        slug in "[a-z]{3,12}",
    ) {
        let endpoint = format!("https://{slug}.proptest.example/agent");
        let body = json!({
            "name": name,
            "description": description,
            "version": version,
            "url": endpoint,
        });
        let bytes = serde_json::to_vec(&body).unwrap();
        let parsed = parse_agent_card_bytes(&bytes, SRC, FINAL).unwrap();
        prop_assert_eq!(parsed.name, name.trim());
        prop_assert_eq!(parsed.description, description.trim());
        prop_assert_eq!(parsed.protocol_version, version.trim());
        prop_assert_eq!(parsed.endpoint_url, endpoint.trim());
        prop_assert!(parsed.external_id.starts_with("sha256:"));
        prop_assert_eq!(parsed.content_hash.len(), 64);
    }
}
