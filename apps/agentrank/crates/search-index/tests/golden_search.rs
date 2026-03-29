//! Golden queries: fixed UUID docs, assert top hit (tokenizer upgrades may require golden refresh).

use agentrank_search_index::schema::AgentSchema;
use agentrank_search_index::search::search_agents;
use agentrank_search_index::store::index_reader;
use serde::Deserialize;
use std::path::Path;
use std::sync::OnceLock;
use tantivy::Index;
use uuid::Uuid;

#[derive(Deserialize)]
struct GoldenFile {
    cases: Vec<GoldenCase>,
}

#[derive(Deserialize)]
struct GoldenCase {
    query: String,
    expect_top: Uuid,
}

fn golden_path() -> &'static Path {
    static P: OnceLock<std::path::PathBuf> = OnceLock::new();
    P.get_or_init(|| Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests/search_golden.json"))
}

fn build_golden_index(dir: &std::path::Path) {
    let ag = AgentSchema::new();
    let index = Index::create_in_dir(dir, ag.schema.clone()).unwrap();
    let mut w = index.writer(20_000_000).unwrap();

    let docs = [
        (
            Uuid::parse_str("aaaaaaaa-aaaa-5aaa-8aaa-aaaaaaaaaaaa").unwrap(),
            "Acme Financial Agent",
            "We analyze markets",
            "portfolio risk acme finance",
        ),
        (
            Uuid::parse_str("bbbbbbbb-bbbb-5bbb-8bbb-bbbbbbbbbbbb").unwrap(),
            "Chef Helper",
            "Recipes and meals",
            "cooking pasta italian kitchen",
        ),
        (
            Uuid::parse_str("cccccccc-cccc-5ccc-8ccc-cccccccccccc").unwrap(),
            "K8s Bot",
            "Infrastructure helper",
            "kubernetes deployment helm cluster",
        ),
    ];

    for (id, name, desc, skills) in docs {
        let mut d = tantivy::TantivyDocument::default();
        d.add_text(ag.agent_id, id.to_string());
        d.add_text(ag.name, name.to_string());
        d.add_text(ag.description, desc.to_string());
        d.add_text(ag.skills_blob, skills.to_string());
        w.add_document(d).unwrap();
    }
    w.commit().unwrap();
    w.wait_merging_threads().unwrap();
    std::fs::write(
        dir.join(agentrank_search_index::schema::VERSION_FILENAME),
        agentrank_search_index::schema::INDEX_VERSION,
    )
    .unwrap();
}

#[test]
fn golden_queries_top_hit() {
    let raw = std::fs::read_to_string(golden_path()).expect("read golden");
    let golden: GoldenFile = serde_json::from_str(&raw).expect("parse golden");

    let dir = tempfile::tempdir().unwrap();
    build_golden_index(dir.path());

    let (reader, index, ag) = index_reader(dir.path()).unwrap();

    for case in golden.cases {
        let hits = search_agents(&reader, &index, &ag, &case.query, 5).unwrap();
        assert!(!hits.is_empty(), "no hits for query {:?}", case.query);
        assert_eq!(
            hits[0].agent_id, case.expect_top,
            "query {:?}: expected top {:?}, got {:?}",
            case.query, case.expect_top, hits[0].agent_id
        );
    }
}
