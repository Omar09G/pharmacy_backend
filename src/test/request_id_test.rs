//! Unit tests for the X-Request-ID correlation middleware.
//!
//! Lives in `src/test/` per project convention: no `#[cfg(test)]` blocks
//! inside production source files.

use axum::body::Body;
use axum::extract::Extension;
use axum::http::{Request, StatusCode};
use axum::middleware::from_fn;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Json, Router};
use serde_json::json;
use tower::ServiceExt;
use uuid::Uuid;

use crate::config::config_middleware::request_id::{
    MAX_INCOMING_ID_LEN, REQUEST_ID_HEADER, RequestId, request_id_middleware,
};

async fn handler(Extension(request_id): Extension<RequestId>) -> Response {
    Json(json!({ "request_id": request_id.0 })).into_response()
}

fn app() -> Router {
    Router::new()
        .route("/ping", get(handler))
        .layer(from_fn(request_id_middleware))
}

#[tokio::test]
async fn generates_uuid_when_header_absent() {
    let res = app()
        .oneshot(Request::builder().uri("/ping").body(Body::empty()).unwrap())
        .await
        .unwrap();

    let sent = res
        .headers()
        .get(REQUEST_ID_HEADER)
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    let parsed = Uuid::parse_str(&sent).expect("generated id must be a valid UUID");
    assert_eq!(parsed.get_version_num(), 4);

    let bytes = axum::body::to_bytes(res.into_body(), 1024).await.unwrap();
    let body: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(body["request_id"], json!(sent), "handler saw the same id");
}

#[tokio::test]
async fn reuses_valid_incoming_header() {
    let res = app()
        .oneshot(
            Request::builder()
                .uri("/ping")
                .header(REQUEST_ID_HEADER, "client-trace_42.abc")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(
        res.headers().get(REQUEST_ID_HEADER).unwrap(),
        "client-trace_42.abc"
    );
}

#[tokio::test]
async fn rejects_unsafe_or_oversized_ids() {
    // All of these are legal HeaderValues but must be replaced: they would
    // pollute log lines or exceed the size cap.
    for bad in [
        "with space",
        "tab\tvalue",
        &"x".repeat(MAX_INCOMING_ID_LEN + 1),
    ] {
        let res = app()
            .oneshot(
                Request::builder()
                    .uri("/ping")
                    .header(REQUEST_ID_HEADER, bad)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_ne!(res.status(), StatusCode::INTERNAL_SERVER_ERROR);
        let sent = res
            .headers()
            .get(REQUEST_ID_HEADER)
            .unwrap()
            .to_str()
            .unwrap();
        assert_ne!(sent, bad.trim(), "unsafe id must be replaced");
        assert!(
            Uuid::parse_str(sent).is_ok(),
            "replacement must be a generated UUID"
        );
    }
}
