use axum::{
    body::Body,
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};
use std::time::{Duration, Instant};

/// In-memory idempotency store. Maps `X-Idempotency-Key` to (cached_response_body, status_code, created_at).
///
/// ⚠️  LIMITATION: This store is local to the process. In multi-instance (horizontal scale)
/// deployments, idempotency keys will NOT be shared across instances.
/// For distributed deployments, replace with Redis:
///   Key  → `idempotency:{X-Idempotency-Key}`
///   TTL  → 24 hours
/// Until then, deploy as a single instance behind a load balancer with session affinity (sticky sessions).
static IDEMPOTENCY_STORE: LazyLock<Mutex<HashMap<String, CachedResponse>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

const IDEMPOTENCY_TTL: Duration = Duration::from_secs(24 * 60 * 60); // 24 hours

struct CachedResponse {
    status: StatusCode,
    body: Vec<u8>,
    created_at: Instant,
}

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
    {
        let mut store = IDEMPOTENCY_STORE.lock().unwrap();
        store.retain(|_, v| v.created_at.elapsed() < IDEMPOTENCY_TTL);

        // Check for cached response
        if let Some(cached) = store.get(&key) {
            if cached.created_at.elapsed() < IDEMPOTENCY_TTL {
                return (
                    cached.status,
                    [(
                        axum::http::header::CONTENT_TYPE,
                        "application/json"
                            .parse::<axum::http::HeaderValue>()
                            .unwrap(),
                    )],
                    cached.body.clone(),
                )
                    .into_response();
            }
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

        {
            let mut store = IDEMPOTENCY_STORE.lock().unwrap();
            store.insert(
                key,
                CachedResponse {
                    status: parts.status,
                    body: body_bytes.to_vec(),
                    created_at: Instant::now(),
                },
            );
        }

        Response::from_parts(parts, Body::from(body_bytes))
    } else {
        response
    }
}
