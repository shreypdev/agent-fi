//! End-to-end hybrid search: Tantivy BM25 + Qdrant kNN + RRF fusion (same path as production).
//!
//! **Requires:** `DATABASE_URL`, `REDIS_URL`, and **`QDRANT_URL`** (gRPC, e.g. `http://127.0.0.1:6334`).
//! Embeddings use the same deterministic **`AGENTRANK_EMBEDDER`** (default hash) as ingest/searchd.
//!
//! CI runs this with a Qdrant service. Locally: `docker compose up qdrant` then
//! `QDRANT_URL=http://127.0.0.1:6334 cargo test -p agentrank-searchd --test hybrid_qdrant_e2e`.

use agentrank_embed::{embed_document, EMBEDDING_DIM};
use agentrank_search_index::skills_blob_from_card_json;
use agentrank_search_index::store::rebuild_index;
use agentrank_searchd::{build_app, init_metrics};
use agentrank_vector::{connect, ensure_agents_collection, upsert_agent_vector};
use axum::body::Body;
use axum::extract::connect_info::ConnectInfo;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use serde_json::json;
use serial_test::serial;
use sqlx::migrate::Migrator;
use sqlx::PgPool;
use std::net::SocketAddr;
use std::path::Path;
use tempfile::tempdir;
use tower::ServiceExt;
use uuid::Uuid;

fn skip_no_qdrant() -> bool {
    if std::env::var("QDRANT_URL")
        .unwrap_or_default()
        .trim()
        .is_empty()
    {
        eprintln!("skip: QDRANT_URL unset (hybrid E2E needs a Qdrant gRPC endpoint)");
        return true;
    }
    false
}

#[tokio::test]
#[serial]
async fn hybrid_search_fuses_bm25_and_qdrant_rrf() {
    if skip_no_qdrant() {
        return;
    }
    let Ok(db_url) = std::env::var("DATABASE_URL") else {
        eprintln!("skip: DATABASE_URL");
        return;
    };
    let Ok(redis_url) = std::env::var("REDIS_URL") else {
        eprintln!("skip: REDIS_URL");
        return;
    };

    let pool = PgPool::connect(&db_url).await.expect("pg");
    let mpath = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../migrations");
    Migrator::new(mpath)
        .await
        .expect("migrator")
        .run(&pool)
        .await
        .expect("migrate");

    let id = Uuid::new_v4();
    let keyword = "hybrid_e2e_unique_kw_rrf";
    let card = json!({
        "name": format!("Agent {keyword}"),
        "description": format!("desc {keyword} beta gamma"),
        "version": "1",
        "url": "https://hybrid-e2e.example/agent",
        "skills": [{"name": "skill1", "tags": [keyword]}]
    });
    let name = format!("Agent {keyword}");
    let desc = format!("desc {keyword} beta gamma");
    sqlx::query(
        r#"
        INSERT INTO agents (
            id, external_id, source_url, canonical_url, card_json, content_hash,
            name, description, endpoint_url, protocol_version
        )
        VALUES ($1, $2, 'https://src', 'https://canon', $3, 'h1', $4, $5,
                'https://hybrid-e2e.example/agent', '1')
        "#,
    )
    .bind(id)
    .bind(format!("ext-{id}"))
    .bind(sqlx::types::Json(card.clone()))
    .bind(&name)
    .bind(&desc)
    .execute(&pool)
    .await
    .expect("insert");

    let dir = tempdir().unwrap();
    let index_path = dir.path();
    rebuild_index(&pool, index_path).await.expect("rebuild");

    let qclient = connect().await.expect("connect qdrant");
    ensure_agents_collection(&qclient, EMBEDDING_DIM as u64)
        .await
        .expect("ensure collection");

    let skills = skills_blob_from_card_json(&card);
    let vec = embed_document(&name, &desc, &skills);
    upsert_agent_vector(&qclient, id, vec)
        .await
        .expect("qdrant upsert");

    init_metrics().expect("metrics");
    let app = build_app(pool, &redis_url, index_path)
        .await
        .expect("build_app hybrid");

    let body = json!({ "query": keyword, "limit": 10 }).to_string();
    let mut req = Request::builder()
        .method("POST")
        .uri("/v1/search")
        .header("content-type", "application/json")
        .body(Body::from(body))
        .unwrap();
    req.extensions_mut()
        .insert(ConnectInfo(SocketAddr::from(([127, 0, 0, 1], 14001))));
    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let bytes = res.into_body().collect().await.unwrap().to_bytes();
    let v: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    let results = v["results"].as_array().expect("results");
    assert!(
        results
            .iter()
            .any(|r| r["agent_id"].as_str() == Some(&id.to_string())),
        "expected agent {id} in hybrid results: {results:?}"
    );

    let mcp = json!({
        "jsonrpc": "2.0",
        "id": 99,
        "method": "tools/call",
        "params": {
            "name": "search_agents",
            "arguments": { "query": keyword, "limit": 5 }
        }
    })
    .to_string();
    let res = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/mcp")
                .header("content-type", "application/json")
                .body(Body::from(mcp))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let bytes = res.into_body().collect().await.unwrap().to_bytes();
    let mcp_v: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    let text = mcp_v["result"]["content"][0]["text"]
        .as_str()
        .expect("mcp text");
    assert!(
        text.contains(&id.to_string()),
        "MCP tool should return hybrid search payload with agent: {text}"
    );
}

#[tokio::test]
#[serial]
async fn ready_succeeds_with_qdrant_configured() {
    if skip_no_qdrant() {
        return;
    }
    let Ok(db_url) = std::env::var("DATABASE_URL") else {
        return;
    };
    let Ok(redis_url) = std::env::var("REDIS_URL") else {
        return;
    };

    let pool = PgPool::connect(&db_url).await.expect("pg");
    let mpath = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../migrations");
    Migrator::new(mpath)
        .await
        .expect("migrator")
        .run(&pool)
        .await
        .expect("migrate");

    let dir = tempdir().unwrap();
    let index_path = dir.path();
    rebuild_index(&pool, index_path).await.expect("rebuild");

    init_metrics().expect("metrics");
    let app = build_app(pool, &redis_url, index_path)
        .await
        .expect("build");

    let res = app
        .oneshot(
            Request::builder()
                .uri("/ready")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);
}
