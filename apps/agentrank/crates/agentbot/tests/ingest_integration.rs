//! Requires `DATABASE_URL` (Postgres). Uses an in-process HTTP server as the card origin.

use axum::http::StatusCode;
use axum::response::Redirect;
use axum::{routing::get, Json, Router};
use serde_json::json;
use sqlx::postgres::PgPoolOptions;
use sqlx::types::Json as SqlxJson;
use sqlx::PgPool;
use std::time::Duration;
use tokio::net::TcpListener;

fn test_ingest_policy() -> agentrank_agentbot::IngestPolicy {
    agentrank_agentbot::IngestPolicy {
        allow_http_localhost: true,
        allow_loopback_https: true,
    }
}

async fn pool_with_migrations() -> PgPool {
    let db = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set for integration test");
    let pool = PgPoolOptions::new()
        .max_connections(64)
        .connect(&db)
        .await
        .expect("connect postgres");
    sqlx::migrate!("../../migrations")
        .run(&pool)
        .await
        .expect("migrations");
    pool
}

#[tokio::test]
async fn mock_agent_json_persists_agent_row() {
    let pool = pool_with_migrations().await;

    let card = json!({
        "name": "TestAgent",
        "description": "Integration test agent",
        "version": "0.5.0",
        "url": "https://agents.test.example/exec"
    });
    let card_for_server = card.clone();

    let app = Router::new().route(
        "/.well-known/agent.json",
        get(move || async move { Json(card_for_server) }),
    );

    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("bind test listener");
    let addr = listener.local_addr().expect("local_addr");
    tokio::spawn(async move {
        axum::serve(listener, app).await.expect("axum serve");
    });

    tokio::time::sleep(Duration::from_millis(50)).await;

    let url = format!("http://{addr}/.well-known/agent.json");
    let client = agentrank_agentbot::http_client().expect("http client");
    let out = agentrank_agentbot::ingest_card_url_with_policy(
        &pool,
        &client,
        &url,
        1_048_576,
        test_ingest_policy(),
    )
    .await
    .expect("ingest");

    let row: (String, String, String, SqlxJson<serde_json::Value>) = sqlx::query_as(
        "SELECT name, endpoint_url, protocol_version, card_json FROM agents WHERE id = $1",
    )
    .bind(out.agent_id)
    .fetch_one(&pool)
    .await
    .expect("select agent");

    let (name, endpoint_url, protocol_version, SqlxJson(card_json)) = row;
    assert_eq!(name, "TestAgent");
    assert_eq!(endpoint_url, "https://agents.test.example/exec");
    assert_eq!(protocol_version, "0.5.0");
    assert_eq!(card_json["name"], "TestAgent");

    assert_eq!(
        out.external_id,
        agentrank_card::parse_agent_card_bytes(&serde_json::to_vec(&card).unwrap(), &url, &url)
            .unwrap()
            .external_id
    );
}

#[tokio::test]
async fn http_404_records_crawl_history_and_fails() {
    let pool = pool_with_migrations().await;
    let app = Router::new().route("/missing.json", get(|| async { StatusCode::NOT_FOUND }));
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });
    tokio::time::sleep(Duration::from_millis(50)).await;

    let url = format!("http://{addr}/missing.json");
    let client = agentrank_agentbot::http_client().unwrap();
    let err = agentrank_agentbot::ingest_card_url_with_policy(
        &pool,
        &client,
        &url,
        1024,
        test_ingest_policy(),
    )
    .await
    .unwrap_err();
    assert!(matches!(
        err,
        agentrank_agentbot::IngestError::HttpStatus(s) if s == reqwest::StatusCode::NOT_FOUND
    ));

    let row: (Option<String>,) = sqlx::query_as(
        "SELECT error_code FROM crawl_history WHERE url = $1 ORDER BY id DESC LIMIT 1",
    )
    .bind(&url)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(row.0.as_deref(), Some("http_error"));
}

#[tokio::test]
async fn invalid_json_200_records_parse_error() {
    let pool = pool_with_migrations().await;
    let app = Router::new().route(
        "/bad.json",
        get(|| async { (StatusCode::OK, r#"{"name":"x"}"#.to_string() + "NOT_JSON") }),
    );
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });
    tokio::time::sleep(Duration::from_millis(50)).await;

    let url = format!("http://{addr}/bad.json");
    let client = agentrank_agentbot::http_client().unwrap();
    let err = agentrank_agentbot::ingest_card_url_with_policy(
        &pool,
        &client,
        &url,
        1024,
        test_ingest_policy(),
    )
    .await
    .unwrap_err();
    assert!(matches!(err, agentrank_agentbot::IngestError::CardParse(_)));

    let row: (Option<String>,) = sqlx::query_as(
        "SELECT error_code FROM crawl_history WHERE url = $1 ORDER BY id DESC LIMIT 1",
    )
    .bind(&url)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(row.0.as_deref(), Some("parse_error"));
}

#[tokio::test]
async fn oversized_body_records_body_too_large() {
    let pool = pool_with_migrations().await;
    let big = vec![b'z'; 800];
    let app = Router::new().route("/huge.json", get(|| async { (StatusCode::OK, big) }));
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });
    tokio::time::sleep(Duration::from_millis(50)).await;

    let url = format!("http://{addr}/huge.json");
    let client = agentrank_agentbot::http_client().unwrap();
    let err = agentrank_agentbot::ingest_card_url_with_policy(
        &pool,
        &client,
        &url,
        400,
        test_ingest_policy(),
    )
    .await
    .unwrap_err();
    assert!(matches!(
        err,
        agentrank_agentbot::IngestError::BodyTooLarge(800, 400)
    ));

    let row: (Option<String>,) = sqlx::query_as(
        "SELECT error_code FROM crawl_history WHERE url = $1 ORDER BY id DESC LIMIT 1",
    )
    .bind(&url)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(row.0.as_deref(), Some("body_too_large"));
}

#[tokio::test]
async fn reingest_same_card_updates_agent_row() {
    let pool = pool_with_migrations().await;
    let card_v1 = json!({
        "name": "V1",
        "description": "first",
        "version": "1.0.0",
        "url": "https://reingest.example/agent"
    });
    let card_v2 = json!({
        "name": "V2",
        "description": "second",
        "version": "2.0.0",
        "url": "https://reingest.example/agent"
    });

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let app = Router::new().route(
        "/card.json",
        get(move || async move { Json(card_v1.clone()) }),
    );
    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });
    tokio::time::sleep(Duration::from_millis(50)).await;

    let url = format!("http://{addr}/card.json");
    let client = agentrank_agentbot::http_client().unwrap();
    let out1 = agentrank_agentbot::ingest_card_url_with_policy(
        &pool,
        &client,
        &url,
        1024,
        test_ingest_policy(),
    )
    .await
    .unwrap();

    let listener2 = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr2 = listener2.local_addr().unwrap();
    let app2 = Router::new().route(
        "/card.json",
        get(move || async move { Json(card_v2.clone()) }),
    );
    tokio::spawn(async move {
        axum::serve(listener2, app2).await.unwrap();
    });
    tokio::time::sleep(Duration::from_millis(50)).await;

    let url2 = format!("http://{addr2}/card.json");
    let out2 = agentrank_agentbot::ingest_card_url_with_policy(
        &pool,
        &client,
        &url2,
        1024,
        test_ingest_policy(),
    )
    .await
    .unwrap();

    assert_eq!(
        out1.agent_id, out2.agent_id,
        "same external_id → same agent row"
    );
    let name: String = sqlx::query_scalar("SELECT name FROM agents WHERE id = $1")
        .bind(out2.agent_id)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(name, "V2");

    let count: i64 =
        sqlx::query_scalar("SELECT COUNT(*)::bigint FROM agents WHERE external_id = $1")
            .bind(&out2.external_id)
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(count, 1);
}

#[tokio::test]
async fn redirect_chain_three_hops_reaches_card() {
    let pool = pool_with_migrations().await;
    let card = json!({
        "name": "R",
        "description": "redirect test",
        "version": "1",
        "url": "https://redirect.example/agent"
    });
    let card = card.clone();
    let app = Router::new()
        .route("/t3", get(move || async move { Json(card.clone()) }))
        .route("/t2", get(|| async { Redirect::temporary("/t3") }))
        .route("/t1", get(|| async { Redirect::temporary("/t2") }))
        .route("/t0", get(|| async { Redirect::temporary("/t1") }));
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });
    tokio::time::sleep(Duration::from_millis(50)).await;

    let url = format!("http://{addr}/t0");
    let client = agentrank_agentbot::http_client().unwrap();
    let out = agentrank_agentbot::ingest_card_url_with_policy(
        &pool,
        &client,
        &url,
        4096,
        test_ingest_policy(),
    )
    .await
    .unwrap();
    let canon: String = sqlx::query_scalar("SELECT canonical_url FROM agents WHERE id = $1")
        .bind(out.agent_id)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert!(
        canon.contains("/t3"),
        "final URL after redirects should point at card resource: {canon}"
    );
}

#[tokio::test]
async fn six_redirects_exceeds_client_policy() {
    let pool = pool_with_migrations().await;
    let card = json!({
        "name": "X",
        "description": "too many hops",
        "version": "1",
        "url": "https://x.example/a"
    });
    let card = card.clone();
    let app = Router::new()
        .route("/h6", get(move || async move { Json(card.clone()) }))
        .route("/h5", get(|| async { Redirect::temporary("/h6") }))
        .route("/h4", get(|| async { Redirect::temporary("/h5") }))
        .route("/h3", get(|| async { Redirect::temporary("/h4") }))
        .route("/h2", get(|| async { Redirect::temporary("/h3") }))
        .route("/h1", get(|| async { Redirect::temporary("/h2") }))
        .route("/h0", get(|| async { Redirect::temporary("/h1") }));
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });
    tokio::time::sleep(Duration::from_millis(50)).await;

    let url = format!("http://{addr}/h0");
    let client = agentrank_agentbot::http_client().unwrap();
    let err = agentrank_agentbot::ingest_card_url_with_policy(
        &pool,
        &client,
        &url,
        4096,
        test_ingest_policy(),
    )
    .await
    .unwrap_err();
    match &err {
        agentrank_agentbot::IngestError::Http(e) => {
            assert!(
                e.is_redirect() || e.is_connect() || e.to_string().contains("redirect"),
                "unexpected http error: {e}"
            );
        }
        _ => panic!("expected redirect-related HTTP error, got {err:?}"),
    }
}

#[tokio::test]
async fn connection_refused_surfaces_http_error() {
    let pool = pool_with_migrations().await;
    let client = agentrank_agentbot::http_client().unwrap();
    let err = agentrank_agentbot::ingest_card_url_with_policy(
        &pool,
        &client,
        "http://127.0.0.1:20987/.well-known/agent.json",
        1024,
        test_ingest_policy(),
    )
    .await
    .unwrap_err();
    let agentrank_agentbot::IngestError::Http(e) = err else {
        panic!("expected Http error, got {err:?}");
    };
    assert!(
        e.is_connect(),
        "connection refused should surface as connect error: {e}"
    );
}

#[tokio::test]
async fn https_url_against_plain_http_listener_fails() {
    let pool = pool_with_migrations().await;
    let app = Router::new().route("/plain", get(|| async { "not-tls" }));
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });
    tokio::time::sleep(Duration::from_millis(50)).await;

    let url = format!("https://{addr}/plain");
    let client = agentrank_agentbot::http_client().unwrap();
    let err = agentrank_agentbot::ingest_card_url_with_policy(
        &pool,
        &client,
        &url,
        1024,
        test_ingest_policy(),
    )
    .await
    .unwrap_err();
    assert!(
        matches!(err, agentrank_agentbot::IngestError::Http(_)),
        "TLS handshake to HTTP-only port should fail: {err:?}"
    );
}

#[tokio::test]
async fn slow_response_hits_reqwest_timeout() {
    let pool = pool_with_migrations().await;
    let card = json!({
        "name": "Slow",
        "description": "late",
        "version": "1",
        "url": "https://slow.example/a"
    });
    let card = card.clone();
    let app = Router::new().route(
        "/slow.json",
        get(move || async move {
            tokio::time::sleep(Duration::from_millis(800)).await;
            Json(card.clone())
        }),
    );
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });
    tokio::time::sleep(Duration::from_millis(50)).await;

    let url = format!("http://{addr}/slow.json");
    let client = agentrank_agentbot::http_client_with_timeout(Duration::from_millis(200)).unwrap();
    let err = agentrank_agentbot::ingest_card_url_with_policy(
        &pool,
        &client,
        &url,
        4096,
        test_ingest_policy(),
    )
    .await
    .unwrap_err();
    match err {
        agentrank_agentbot::IngestError::Http(e) => {
            assert!(e.is_timeout(), "expected timeout, got {e}");
        }
        o => panic!("expected Http error, got {o:?}"),
    }
}

#[tokio::test]
async fn unresolvable_host_fails_without_panicking() {
    let pool = pool_with_migrations().await;
    let client = agentrank_agentbot::http_client_with_timeout(Duration::from_secs(12)).unwrap();
    let err = agentrank_agentbot::ingest_card_url_with_policy(
        &pool,
        &client,
        "https://agentrank-nx-7f2a.invalid/.well-known/agent.json",
        1024,
        test_ingest_policy(),
    )
    .await
    .unwrap_err();
    assert!(
        matches!(err, agentrank_agentbot::IngestError::Http(_)),
        "NXDOMAIN / DNS failure should map to Http error: {err:?}"
    );
}

#[tokio::test]
async fn concurrent_ingest_same_external_id_single_agent_row() {
    let pool = pool_with_migrations().await;
    let card = json!({
        "name": "Concurrent",
        "description": "race ingest",
        "version": "1",
        "url": "https://concurrent-stress.example/agent"
    });
    let card = card.clone();
    let app = Router::new().route("/card.json", get(move || async move { Json(card.clone()) }));
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });
    tokio::time::sleep(Duration::from_millis(50)).await;

    let url = format!("http://{addr}/card.json");
    let client = agentrank_agentbot::http_client().unwrap();
    const N: usize = 32;
    let mut handles = Vec::new();
    for _ in 0..N {
        let pool = pool.clone();
        let client = client.clone();
        let url = url.clone();
        handles.push(tokio::spawn(async move {
            agentrank_agentbot::ingest_card_url_with_policy(
                &pool,
                &client,
                &url,
                8192,
                test_ingest_policy(),
            )
            .await
        }));
    }
    let mut expected_ext: Option<String> = None;
    let mut expected_id = None;
    for h in handles {
        let ok = h.await.unwrap().expect("concurrent ingest");
        match (&expected_ext, expected_id) {
            (None, None) => {
                expected_ext = Some(ok.external_id.clone());
                expected_id = Some(ok.agent_id);
            }
            (Some(ext), Some(id)) => {
                assert_eq!(ext, &ok.external_id);
                assert_eq!(id, ok.agent_id);
            }
            _ => unreachable!(),
        }
    }

    let ext = expected_ext.expect("external_id");
    let count: i64 =
        sqlx::query_scalar("SELECT COUNT(*)::bigint FROM agents WHERE external_id = $1")
            .bind(&ext)
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(count, 1, "ON CONFLICT must collapse to one agent row");

    let crawls: i64 = sqlx::query_scalar(
        "SELECT COUNT(*)::bigint FROM crawl_history WHERE url = $1 AND agent_id IS NOT NULL",
    )
    .bind(&url)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(
        crawls, N as i64,
        "each successful ingest should append crawl_history"
    );
}
