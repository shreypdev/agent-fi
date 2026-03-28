//! Normalized cards from successful parses satisfy the repo's core JSON Schema (draft-07).

use agentrank_card::parse_agent_card_bytes;
use jsonschema::Validator;
use serde_json::Value;
use std::fs;
use std::path::PathBuf;
use std::sync::OnceLock;

const SRC: &str = "https://x/.well-known/agent.json";
const FINAL: &str = "https://x/.well-known/agent.json";

fn core_validator() -> &'static Validator {
    static V: OnceLock<Validator> = OnceLock::new();
    V.get_or_init(|| {
        let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let path = manifest_dir.join("../../schemas/agent_card_core.schema.json");
        let raw = fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("read schema {}: {e}", path.display()));
        let schema: Value = serde_json::from_str(&raw).expect("schema JSON");
        jsonschema::validator_for(&schema).expect("compile schema")
    })
}

fn assert_instance_valid(instance: &Value) {
    if let Err(e) = core_validator().validate(instance) {
        panic!("schema validation failed: {e}");
    }
}

#[test]
fn opus_style_fixture_passes_schema_after_parse() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let path = manifest_dir.join("tests/fixtures/opus_style_card.json");
    let bytes = fs::read(&path).unwrap();
    let parsed = parse_agent_card_bytes(
        &bytes,
        "https://acmecorp.com/.well-known/agent.json",
        "https://acmecorp.com/.well-known/agent.json",
    )
    .unwrap();
    assert_instance_valid(&parsed.normalized_card);
}

#[test]
fn minimal_parse_output_validates_schema() {
    let j = agentrank_card::minimal_valid_card_json("A", "B", "1.0.0", "https://x.example/y");
    let parsed = parse_agent_card_bytes(j.as_bytes(), SRC, FINAL).unwrap();
    assert_instance_valid(&parsed.normalized_card);
}
