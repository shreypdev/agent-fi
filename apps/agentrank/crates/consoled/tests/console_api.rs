//! Integration tests: real Postgres from `DATABASE_URL` (set in CI). No env races — API key on [`AppState`].

use agentrank_consoled::{app, AppState};
use axum::body::Body;
use axum::http::{Request, StatusCode};
use sqlx::postgres::PgPoolOptions;
use tower::ServiceExt;
use uuid::Uuid;

async fn pool_with_migrations() -> sqlx::PgPool {
    let db = std::env::var("DATABASE_URL").expect("DATABASE_URL required for consoled tests");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db)
        .await
        .expect("postgres connect");
    sqlx::migrate!("../../migrations")
        .run(&pool)
        .await
        .expect("migrations");
    pool
}

#[tokio::test]
async fn health_ok_without_auth() {
    let pool = pool_with_migrations().await;
    let app = app(AppState::new(pool, "unit-test-secret"));
    let res = app
        .oneshot(Request::get("/health").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);
}

#[tokio::test]
async fn console_routes_401_without_credentials() {
    let pool = pool_with_migrations().await;
    let app = app(AppState::new(pool, "unit-test-secret"));
    let res = app
        .oneshot(
            Request::get("/v1/console/domain-claims")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn console_accepts_bearer_token() {
    let pool = pool_with_migrations().await;
    let app = app(AppState::new(pool, "unit-test-secret"));
    let res = app
        .oneshot(
            Request::get("/v1/console/domain-claims")
                .header("Authorization", "Bearer unit-test-secret")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);
}

#[tokio::test]
async fn console_accepts_x_console_key() {
    let pool = pool_with_migrations().await;
    let app = app(AppState::new(pool, "unit-test-secret"));
    let res = app
        .oneshot(
            Request::get("/v1/console/domain-claims")
                .header("x-console-key", "unit-test-secret")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);
}

#[tokio::test]
async fn empty_configured_key_returns_503() {
    let pool = pool_with_migrations().await;
    let app = app(AppState::new(pool, "   "));
    let res = app
        .oneshot(
            Request::get("/v1/console/domain-claims")
                .header("Authorization", "Bearer x")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::SERVICE_UNAVAILABLE);
}

#[tokio::test]
async fn get_agent_unknown_id_404() {
    let pool = pool_with_migrations().await;
    let app = app(AppState::new(pool, "unit-test-secret"));
    let id = Uuid::nil();
    let res = app
        .oneshot(
            Request::get(format!("/v1/console/agents/{id}"))
                .header("Authorization", "Bearer unit-test-secret")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn crawl_history_limit_clamped_high() {
    let pool = pool_with_migrations().await;
    let app = app(AppState::new(pool, "unit-test-secret"));
    // limit=9999 must clamp to 200 — still 200 OK with empty or partial rows
    let res = app
        .oneshot(
            Request::get("/v1/console/crawl-history?domain=example.com&limit=9999&offset=0")
                .header("Authorization", "Bearer unit-test-secret")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::OK);
}

#[tokio::test]
async fn create_claim_empty_domain_400() {
    let pool = pool_with_migrations().await;
    let app = app(AppState::new(pool, "unit-test-secret"));
    let res = app
        .oneshot(
            Request::post("/v1/console/domain-claims")
                .header("Authorization", "Bearer unit-test-secret")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"domain":"   "}"#))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);
}
