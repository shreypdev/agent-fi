//! Full-stack smoke: Postgres + Redis + Tantivy + Axum (requires `DATABASE_URL`, `REDIS_URL`).

use agentrank_search_index::schema::VERSION_FILENAME;
use agentrank_search_index::store::rebuild_index;
use agentrank_searchd::{build_app, init_metrics};
use axum::body::Body;
use axum::extract::connect_info::ConnectInfo;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use std::net::SocketAddr;
use serde_json::json;
use sqlx::migrate::Migrator;
use sqlx::PgPool;
use std::path::Path;
use tempfile::tempdir;
use tower::ServiceExt;
use uuid::Uuid;

#[tokio::test]
async fn search_empty_query_400_and_health_ok() {
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
async fn search_finds_inserted_agent() {
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
async fn ready_fails_when_index_marker_removed() {
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
