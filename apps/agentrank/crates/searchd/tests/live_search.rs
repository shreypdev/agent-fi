//! Full-stack smoke: Postgres + Redis + Tantivy + Axum (requires `DATABASE_URL`, `REDIS_URL`).
//!
//! Tests that call [`build_app`] are marked `#[serial]` so local `QDRANT_URL` can be cleared
//! without racing other tests, and Redis hint counters stay deterministic.

use agentrank_search_index::schema::VERSION_FILENAME;
use agentrank_search_index::store::rebuild_index;
use agentrank_searchd::{build_app, init_metrics};
use axum::body::Body;
use axum::extract::connect_info::ConnectInfo;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use redis::AsyncCommands;
use serde_json::json;
use serial_test::serial;
use sqlx::migrate::Migrator;
use sqlx::PgPool;
use std::net::SocketAddr;
use std::path::Path;
use tempfile::tempdir;
use tower::ServiceExt;
use uuid::Uuid;

/// Avoid hybrid path trying to reach a Qdrant instance that CI/local may not run.
fn ensure_no_qdrant_for_tests() {
    std::env::remove_var("QDRANT_URL");
}

async fn clear_hints_daily_counter_for_localhost(redis_url: &str) {
    let day = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() / 86_400)
        .unwrap_or(0);
    let key = format!("hintsrl:v1:127.0.0.1:{day}");
    let Ok(client) = redis::Client::open(redis_url) else {
        return;
    };
    let Ok(mut conn) = client.get_multiplexed_async_connection().await else {
        return;
    };
    let _ = conn.del::<_, u64>(&key).await;
}

#[tokio::test]
#[serial]
async fn search_empty_query_400_and_health_ok() {
    let Ok(db_url) = std::env::var("DATABASE_URL") else {
        eprintln!("skip: DATABASE_URL");
        return;
    };
    let Ok(redis_url) = std::env::var("REDIS_URL") else {
        eprintln!("skip: REDIS_URL");
        return;
    };
    ensure_no_qdrant_for_tests();

    let pool = PgPool::connect(&db_url).await.expect("pg");
    let mpath = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../migrations");
    let migrator = Migrator::new(mpath.clone()).await.expect("migrator");
    migrator.run(&pool).await.expect("migrate");

    let dir = tempdir().unwrap();
    let index_path = dir.path();
    rebuild_index(&pool, index_path).await.expect("rebuild");

    init_metrics().expect("init_metrics");

    let app = build_app(pool.clone(), &redis_url, index_path)
        .await
        .expect("build");

    let res = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    let res = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/ready")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    let mut req = Request::builder()
        .method("POST")
        .uri("/v1/search")
        .header("content-type", "application/json")
        .body(Body::from(r#"{"query":"   ","limit":5}"#))
        .unwrap();
    req.extensions_mut()
        .insert(ConnectInfo(SocketAddr::from(([127, 0, 0, 1], 1234))));
    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
#[serial]
async fn search_finds_inserted_agent() {
    let Ok(db_url) = std::env::var("DATABASE_URL") else {
        return;
    };
    let Ok(redis_url) = std::env::var("REDIS_URL") else {
        return;
    };
    ensure_no_qdrant_for_tests();

    let pool = PgPool::connect(&db_url).await.expect("pg");
    let mpath = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../migrations");
    Migrator::new(mpath)
        .await
        .expect("migrator")
        .run(&pool)
        .await
        .expect("migrate");

    let id = Uuid::new_v4();
    let card = json!({
        "name": "ZebraSearchTest",
        "description": "unique zebra keyword for search smoke",
        "version": "1",
        "url": "https://zebra-search-test.example/agent",
        "skills": [{"name": "alpha", "tags": ["zebra"]}]
    });
    sqlx::query(
        r#"
        INSERT INTO agents (
            id, external_id, source_url, canonical_url, card_json, content_hash,
            name, description, endpoint_url, protocol_version
        )
        VALUES ($1, $2, 'https://src', 'https://canon', $3, 'h1', 'ZebraSearchTest',
                'unique zebra keyword for search smoke',
                'https://zebra-search-test.example/agent', '1')
        "#,
    )
    .bind(id)
    .bind(format!("ext-{id}"))
    .bind(sqlx::types::Json(card))
    .execute(&pool)
    .await
    .expect("insert");

    let dir = tempdir().unwrap();
    let index_path = dir.path();
    rebuild_index(&pool, index_path).await.expect("rebuild");

    init_metrics().expect("init_metrics");

    let app = build_app(pool, &redis_url, index_path)
        .await
        .expect("build");

    let body = json!({"query": "zebra alpha", "limit": 10}).to_string();
    let mut req = Request::builder()
        .method("POST")
        .uri("/v1/search")
        .header("content-type", "application/json")
        .body(Body::from(body))
        .unwrap();
    req.extensions_mut()
        .insert(ConnectInfo(SocketAddr::from(([127, 0, 0, 1], 1235))));
    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let bytes = res.into_body().collect().await.unwrap().to_bytes();
    let v: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    let results = v["results"].as_array().expect("results");
    assert!(
        results
            .iter()
            .any(|r| r["agent_id"].as_str() == Some(&id.to_string())),
        "expected agent in results: {results:?}"
    );

    let res = app
        .oneshot(
            Request::builder()
                .uri(format!("/v1/agents/{id}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);
}

#[tokio::test]
#[serial]
async fn ready_fails_when_index_marker_removed() {
    let Ok(db_url) = std::env::var("DATABASE_URL") else {
        eprintln!("skip: DATABASE_URL");
        return;
    };
    let Ok(redis_url) = std::env::var("REDIS_URL") else {
        eprintln!("skip: REDIS_URL");
        return;
    };
    ensure_no_qdrant_for_tests();

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

    init_metrics().expect("init_metrics");

    let app = build_app(pool, &redis_url, index_path)
        .await
        .expect("build");

    std::fs::remove_file(index_path.join(VERSION_FILENAME)).expect("remove version file");

    let res = app
        .oneshot(
            Request::builder()
                .uri("/ready")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::SERVICE_UNAVAILABLE);
}

#[tokio::test]
#[serial]
async fn mcp_json_rpc_and_a2a_structured_smoke() {
    let Ok(db_url) = std::env::var("DATABASE_URL") else {
        return;
    };
    let Ok(redis_url) = std::env::var("REDIS_URL") else {
        return;
    };
    ensure_no_qdrant_for_tests();

    let pool = PgPool::connect(&db_url).await.expect("pg");
    let mpath = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../migrations");
    Migrator::new(mpath)
        .await
        .expect("migrator")
        .run(&pool)
        .await
        .expect("migrate");

    let id = Uuid::new_v4();
    let card = json!({
        "name": "McpA2aTest",
        "description": "mcp zebra keyword",
        "version": "1",
        "url": "https://mcp-a2a.example/agent",
        "skills": [{"name": "alpha", "tags": ["zebra"]}]
    });
    sqlx::query(
        r#"
        INSERT INTO agents (
            id, external_id, source_url, canonical_url, card_json, content_hash,
            name, description, endpoint_url, protocol_version
        )
        VALUES ($1, $2, 'https://src', 'https://canon', $3, 'h1', 'McpA2aTest',
                'mcp zebra keyword',
                'https://mcp-a2a.example/agent', '1')
        "#,
    )
    .bind(id)
    .bind(format!("ext-mcp-{id}"))
    .bind(sqlx::types::Json(card))
    .execute(&pool)
    .await
    .expect("insert");

    let dir = tempdir().unwrap();
    let index_path = dir.path();
    rebuild_index(&pool, index_path).await.expect("rebuild");

    init_metrics().expect("init_metrics");
    let app = build_app(pool, &redis_url, index_path)
        .await
        .expect("build");

    let res = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/.well-known/mcp.json")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let bytes = res.into_body().collect().await.unwrap().to_bytes();
    let manifest: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert!(manifest["transport"]["url"]
        .as_str()
        .unwrap()
        .contains("/mcp"));

    let res = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/.well-known/agent-card.json")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let bytes = res.into_body().collect().await.unwrap().to_bytes();
    let card_j: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert!(!card_j["skills"].as_array().unwrap().is_empty());

    let init_body = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "initialize",
        "params": {}
    })
    .to_string();
    let res = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/mcp")
                .header("content-type", "application/json")
                .body(Body::from(init_body))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let bytes = res.into_body().collect().await.unwrap().to_bytes();
    let v: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert!(v.get("result").is_some());

    let list_body = json!({
        "jsonrpc": "2.0",
        "id": 2,
        "method": "tools/list",
        "params": {}
    })
    .to_string();
    let res = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/mcp")
                .header("content-type", "application/json")
                .body(Body::from(list_body))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let bytes = res.into_body().collect().await.unwrap().to_bytes();
    let v: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    let tools = v["result"]["tools"].as_array().expect("tools");
    assert!(tools.iter().any(|t| t["name"] == "search_agents"));

    let call_search = json!({
        "jsonrpc": "2.0",
        "id": 3,
        "method": "tools/call",
        "params": {
            "name": "search_agents",
            "arguments": { "query": "zebra", "limit": 5 }
        }
    })
    .to_string();
    let res = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/mcp")
                .header("content-type", "application/json")
                .body(Body::from(call_search))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    let call_detail = json!({
        "jsonrpc": "2.0",
        "id": 4,
        "method": "tools/call",
        "params": {
            "name": "get_agent_details",
            "arguments": { "agent_id": id.to_string() }
        }
    })
    .to_string();
    let res = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/mcp")
                .header("content-type", "application/json")
                .body(Body::from(call_detail))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);

    let call_empty_query = json!({
        "jsonrpc": "2.0",
        "id": 41,
        "method": "tools/call",
        "params": {
            "name": "search_agents",
            "arguments": { "query": "   ", "limit": 5 }
        }
    })
    .to_string();
    let res = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/mcp")
                .header("content-type", "application/json")
                .body(Body::from(call_empty_query))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);

    let bad_method = json!({
        "jsonrpc": "2.0",
        "id": 5,
        "method": "foo/unknown",
        "params": {}
    })
    .to_string();
    let res = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/mcp")
                .header("content-type", "application/json")
                .body(Body::from(bad_method))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let bytes = res.into_body().collect().await.unwrap().to_bytes();
    let v: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert!(v.get("error").is_some());

    let res = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/mcp")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::METHOD_NOT_ALLOWED);

    let a2a_nl = json!({ "text": "find agents about cats" }).to_string();
    let res = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/a2a")
                .header("content-type", "application/json")
                .body(Body::from(a2a_nl))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);

    let a2a_no_skill = json!({}).to_string();
    let res = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/a2a")
                .header("content-type", "application/json")
                .body(Body::from(a2a_no_skill))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);

    let a2a_bad = json!({ "skill": "not_a_real_skill" }).to_string();
    let res = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/a2a")
                .header("content-type", "application/json")
                .body(Body::from(a2a_bad))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);

    let a2a_search = json!({
        "skill": "search_agents",
        "query": "zebra",
        "limit": 5
    })
    .to_string();
    let res = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/a2a")
                .header("content-type", "application/json")
                .body(Body::from(a2a_search))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);
    let bytes = res.into_body().collect().await.unwrap().to_bytes();
    let v: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(v["skill"], "search_agents");
    assert!(v["data"]["results"].is_array());

    let a2a_detail = json!({
        "skill": "get_agent_details",
        "agent_id": id
    })
    .to_string();
    let res = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/v1/a2a")
                .header("content-type", "application/json")
                .body(Body::from(a2a_detail))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);
}

#[tokio::test]
#[serial]
async fn post_hints_enqueue_ssrf_reject_and_daily_cap() {
    let Ok(db_url) = std::env::var("DATABASE_URL") else {
        return;
    };
    let Ok(redis_url) = std::env::var("REDIS_URL") else {
        return;
    };
    ensure_no_qdrant_for_tests();

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

    init_metrics().expect("init_metrics");
    let app = build_app(pool, &redis_url, index_path)
        .await
        .expect("build");

    let mut req = Request::builder()
        .method("POST")
        .uri("/v1/hints")
        .header("content-type", "application/json")
        .body(Body::from(
            json!({"url":"http://127.0.0.1:9999/hint","source":"t"}).to_string(),
        ))
        .unwrap();
    req.extensions_mut()
        .insert(ConnectInfo(SocketAddr::from(([127, 0, 0, 1], 13001))));
    let res = app.clone().oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);

    clear_hints_daily_counter_for_localhost(&redis_url).await;

    let hint_url = "https://hint-smoke.example/.well-known/agent.json";
    for i in 0..5 {
        let mut req = Request::builder()
            .method("POST")
            .uri("/v1/hints")
            .header("content-type", "application/json")
            .body(Body::from(
                json!({"url": hint_url, "source": format!("s{i}")}).to_string(),
            ))
            .unwrap();
        req.extensions_mut()
            .insert(ConnectInfo(SocketAddr::from(([127, 0, 0, 1], 13002))));
        let res = app.clone().oneshot(req).await.unwrap();
        assert_eq!(res.status(), StatusCode::OK, "hint iteration {i}");
        let bytes = res.into_body().collect().await.unwrap().to_bytes();
        let v: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(v["queued"], true);
    }

    let mut req = Request::builder()
        .method("POST")
        .uri("/v1/hints")
        .header("content-type", "application/json")
        .body(Body::from(
            json!({"url": "https://another-hint.example/x", "source": "over"}).to_string(),
        ))
        .unwrap();
    req.extensions_mut()
        .insert(ConnectInfo(SocketAddr::from(([127, 0, 0, 1], 13002))));
    let res = app.oneshot(req).await.unwrap();
    assert_eq!(res.status(), StatusCode::TOO_MANY_REQUESTS);
}
