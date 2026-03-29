//! Tantivy integration: known docs, assert top hit.

use agentrank_search_index::schema::AgentSchema;
use agentrank_search_index::search::search_agents;
use agentrank_search_index::store::index_reader;
use serde_json::json;
use tempfile::tempdir;
use uuid::Uuid;

#[test]
fn three_docs_portfolio_wins() {
    let dir = tempdir().unwrap();
    let path = dir.path();
    let ag = AgentSchema::new();
    let index = tantivy::Index::create_in_dir(path, ag.schema.clone()).unwrap();
    let mut writer = index.writer(20_000_000).unwrap();

    let mk = |id: Uuid, name: &str, desc: &str, skills: &str| {
        let mut d = tantivy::TantivyDocument::default();
        d.add_text(ag.agent_id, id.to_string());
        d.add_text(ag.name, name.to_string());
        d.add_text(ag.description, desc.to_string());
        d.add_text(ag.skills_blob, skills.to_string());
        d
    };

    let a = Uuid::new_v4();
    let b = Uuid::new_v4();
    let c = Uuid::new_v4();
    writer
        .add_document(mk(a, "Other", "nothing", "cooking"))
        .unwrap();
    writer
        .add_document(mk(b, "Finance Bot", "generic", "stocks"))
        .unwrap();
    writer
        .add_document(mk(
            c,
            "Helper",
            "assistant",
            "portfolio analysis risk finance",
        ))
        .unwrap();
    writer.commit().unwrap();
    writer.wait_merging_threads().unwrap();
    std::fs::write(
        path.join(agentrank_search_index::schema::VERSION_FILENAME),
        agentrank_search_index::schema::INDEX_VERSION,
    )
    .unwrap();

    let (reader, index, ag) = index_reader(path).unwrap();
    let hits = search_agents(&reader, &index, &ag, "portfolio finance", 5).unwrap();
    assert!(!hits.is_empty(), "expected hits");
    assert_eq!(hits[0].agent_id, c, "skills-heavy doc should rank first");
}

#[test]
fn extract_roundtrip_via_row() {
    let card = json!({
        "name": "N",
        "description": "D",
        "version": "1",
        "url": "https://ex.com/a",
        "skills": [{"name": "Alpha", "tags": ["t1"], "description": "SkillDesc"}]
    });
    let row = agentrank_search_index::AgentIndexRow {
        id: Uuid::new_v4(),
        name: "Agent".into(),
        description: "Desc".into(),
        endpoint_url: "https://ex.com/a".into(),
        protocol_version: "1".into(),
        card_json: sqlx::types::Json(card),
        provider_display_name: None,
        trust_tier: Some("indexed".into()),
    };
    let ag = AgentSchema::new();
    let doc = ag.doc_from_row(&row);
    let dir = tempdir().unwrap();
    let path = dir.path();
    let index = tantivy::Index::create_in_dir(path, ag.schema.clone()).unwrap();
    let mut w = index.writer(20_000_000).unwrap();
    w.add_document(doc).unwrap();
    w.commit().unwrap();
    w.wait_merging_threads().unwrap();
    std::fs::write(
        path.join(agentrank_search_index::schema::VERSION_FILENAME),
        agentrank_search_index::schema::INDEX_VERSION,
    )
    .unwrap();

    let (reader, index, ag) = index_reader(path).unwrap();
    let hits = search_agents(&reader, &index, &ag, "Alpha SkillDesc", 5).unwrap();
    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].agent_id, row.id);
}
