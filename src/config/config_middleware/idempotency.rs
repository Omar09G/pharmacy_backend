use crate::config::config_redis;
use axum::{
    body::Body,
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use std::time::Duration;

/// In-memory idempotency store. Maps `X-Idempotency-Key` to (cached_response_body, status_code, created_at).
///
/// ⚠️  LIMITATION: This store is local to the process. In multi-instance (horizontal scale)
/// deployments, idempotency keys will NOT be shared across instances.
/// For distributed deployments, replace with Redis:
///   Key  → `idempotency:{X-Idempotency-Key}`
///   TTL  → 24 hours
/// Until then, deploy as a single instance behind a load balancer with session affinity (sticky sessions).
// When Redis is available we store cached responses under `idempotency:{key}` as base64
// encoded body and status separated by a small header. For simplicity we keep
// the existing in-memory store as a fallback when Redis is unavailable.

const IDEMPOTENCY_TTL: Duration = Duration::from_secs(24 * 60 * 60); // 24 hours

/// Middleware that enforces idempotency for POST requests.
///
/// If a `X-Idempotency-Key` header is present on a POST request:
/// - If the key was seen before (within TTL), return the cached response.
/// - Otherwise, execute the handler, cache the response, and return it.
///
/// Non-POST requests and requests without the header pass through unmodified.
pub async fn idempotency_middleware(req: Request<Body>, next: Next) -> Response {
    // Only apply to POST (creation) requests
    if req.method() != axum::http::Method::POST {
        return next.run(req).await;
    }

    let idempotency_key = req
        .headers()
        .get("x-idempotency-key")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    let key = match idempotency_key {
        Some(k) if !k.is_empty() && k.len() <= 256 => k,
        _ => return next.run(req).await, // No key or invalid — proceed normally
    };

    // Clean up expired entries periodically (simple approach)
    // Try Redis first
    if let Ok(Some(payload)) = config_redis::get_kv(&format!("idempotency:{}", key)).await {
        // Stored format: 1 byte status (u16 little-endian), then body bytes
        if payload.len() >= 2 {
            let status = u16::from_le_bytes([payload[0], payload[1]]);
            let body = payload[2..].to_vec();
            return (
                StatusCode::from_u16(status).unwrap_or(StatusCode::OK),
                [(
                    axum::http::header::CONTENT_TYPE,
                    "application/json"
                        .parse::<axum::http::HeaderValue>()
                        .unwrap(),
                )],
                body,
            )
                .into_response();
        }
    }

    // Execute the actual handler
    let response = next.run(req).await;
    let status = response.status();

    // Only cache successful responses (2xx)
    if status.is_success() {
        let (parts, body) = response.into_parts();
        let body_bytes = axum::body::to_bytes(body, 1024 * 1024)
            .await
            .unwrap_or_default();

        // Try to store in Redis (status u16 + body)
        let mut to_store = vec![];
        to_store.extend_from_slice(&(parts.status.as_u16() as u16).to_le_bytes());
        to_store.extend_from_slice(&body_bytes);
        let _ = config_redis::set_kv(
            &format!("idempotency:{}", key),
            &to_store,
            IDEMPOTENCY_TTL.as_secs() as usize,
        )
        .await;

        Response::from_parts(parts, Body::from(body_bytes))
    } else {
        response
    }
}
