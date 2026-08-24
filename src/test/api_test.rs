//! Integration tests for critical HTTP endpoints (TEST-2 / ROBUST-3).
//!
//! These exercise the real router (middlewares included) via
//! `tower::ServiceExt::oneshot`, without requiring a live database or Redis:
//! - `DatabaseConnection::default()` yields a `Disconnected` connection, so
//!   the readiness probe must report DOWN (503).
//! - Redis helpers fail fast when the client is uninitialized, so the rate
//!   limiter exercises its in-memory fallback path.

use crate::config::config_database::config_db_context::AppContext;
use crate::controller::api_controller::get_config_router;
use crate::test::redis_state_lock;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use sea_orm::DatabaseConnection;
use tower::ServiceExt;

fn test_router() -> axum::Router {
    let ctx = AppContext {
        conn: DatabaseConnection::default(),
    };
    get_config_router(&ctx).expect("router must build for tests")
}

async fn send(router: axum::Router, req: Request<Body>) -> axum::response::Response {
    router.oneshot(req).await.expect("infallible oneshot")
}

// ── Health endpoints ─────────────────────────────────────────────────────────

#[tokio::test]
async fn liveness_returns_200_up() {
    let _redis_guard = redis_state_lock().lock().await;
    // Fresh rate-limit buckets: earlier tests may have exhausted them and
    // buckets persist across tests within this process.
    crate::config::config_middleware::rate_limit::reset_in_memory_buckets_for_tests().await;
    let res = send(
        test_router(),
        Request::get("/v1/api/health").body(Body::empty()).unwrap(),
    )
    .await;

    assert_eq!(res.status(), StatusCode::OK);
    let body = axum::body::to_bytes(res.into_body(), 64 * 1024)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["status"], "UP");
    assert_eq!(json["service"], "pharmacy-backend");
}

#[tokio::test]
async fn readiness_with_disconnected_db_returns_503_down() {
    let _redis_guard = redis_state_lock().lock().await;
    // Fresh rate-limit buckets: earlier tests may have exhausted them and
    // buckets persist across tests within this process.
    crate::config::config_middleware::rate_limit::reset_in_memory_buckets_for_tests().await;
    let res = send(
        test_router(),
        Request::get("/v1/api/health/ready")
            .body(Body::empty())
            .unwrap(),
    )
    .await;

    assert_eq!(
        res.status(),
        StatusCode::SERVICE_UNAVAILABLE,
        "readiness must signal 503 when the DB is unreachable"
    );
    let body = axum::body::to_bytes(res.into_body(), 64 * 1024)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["status"], "DOWN");
    assert_eq!(json["checks"]["database"], "DOWN");
    assert_eq!(json["checks"]["redis"], "DEGRADED");
}

// ── Security headers ────────────────────────────────────────────────────────

#[tokio::test]
async fn responses_include_security_headers() {
    let _redis_guard = redis_state_lock().lock().await;
    // Fresh rate-limit buckets: earlier tests may have exhausted them and
    // buckets persist across tests within this process.
    crate::config::config_middleware::rate_limit::reset_in_memory_buckets_for_tests().await;
    let res = send(
        test_router(),
        Request::get("/v1/api/health").body(Body::empty()).unwrap(),
    )
    .await;

    assert_eq!(
        res.headers()
            .get("x-frame-options")
            .and_then(|v| v.to_str().ok()),
        Some("DENY")
    );
    assert!(res.headers().contains_key("x-content-type-options"));
}

// ── Auth endpoint validation ────────────────────────────────────────────────

#[tokio::test]
async fn login_with_malformed_json_is_rejected_without_panic() {
    let _redis_guard = redis_state_lock().lock().await;
    // Fresh rate-limit buckets: earlier tests may have exhausted them and
    // buckets persist across tests within this process.
    crate::config::config_middleware::rate_limit::reset_in_memory_buckets_for_tests().await;
    // Invalid JSON must be rejected by the extractor (4xx) before touching
    // the (disconnected) database — no panic, no 500.
    let res = send(
        test_router(),
        Request::post("/v1/api/auth/login")
            .header("content-type", "application/json")
            .body(Body::from("{ not valid json"))
            .unwrap(),
    )
    .await;

    let status = res.status();
    assert!(
        status == StatusCode::BAD_REQUEST || status == StatusCode::UNPROCESSABLE_ENTITY,
        "expected 400/422, got {status}"
    );
}

// ── Rate limiting (in-memory fallback path) ─────────────────────────────────

#[tokio::test]
async fn login_rate_limit_eventually_returns_429() {
    let _redis_guard = redis_state_lock().lock().await;
    // Fresh rate-limit buckets: earlier tests may have exhausted them and
    // buckets persist across tests within this process.
    crate::config::config_middleware::rate_limit::reset_in_memory_buckets_for_tests().await;
    // Login bucket allows 10 requests / 30 min per IP. All tests share the
    // same process-wide bucket ("unknown" IP), and tests may run in parallel,
    // so fire enough requests that the cap MUST be exceeded regardless of order.
    let mut got_429 = false;
    for _ in 0..20 {
        let res = send(
            test_router(),
            Request::post("/v1/api/auth/login")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"username":"rate_test_user","password":"x"}"#,
                ))
                .unwrap(),
        )
        .await;
        if res.status() == StatusCode::TOO_MANY_REQUESTS {
            got_429 = true;
            break;
        }
    }
    assert!(
        got_429,
        "login burst must eventually hit 429 TOO_MANY_REQUESTS"
    );
}
