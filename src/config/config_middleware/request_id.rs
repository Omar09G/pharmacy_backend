use std::time::Instant;

use axum::body::Body;
use axum::http::{HeaderValue, Request};
use axum::middleware::Next;
use axum::response::Response;
use log::info;
use uuid::Uuid;

/// Header used to correlate each API call with its server-side logs.
/// Clients may send their own value; otherwise a UUID v4 is generated.
pub const REQUEST_ID_HEADER: &str = "x-request-id";

/// Defensive cap for client-supplied ids: rejects oversized values before
/// they reach logs or Redis-backed layers.
pub(crate) const MAX_INCOMING_ID_LEN: usize = 128;

fn is_safe_id_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.')
}

/// Extractor-friendly wrapper so handlers/middlewares can pull the current
/// request id from extensions: `Extension(request_id): Extension<RequestId>`.
#[derive(Clone, Debug)]
pub struct RequestId(pub String);

/// Assigns a request id to every call, echoes it back in the response and
/// emits one trace line per request: method, path, status, duration and id.
///
/// Runs as the outermost layer so even early failures (timeouts, oversized
/// bodies, CORS denials) are traced with the same id.
pub async fn request_id_middleware(mut req: Request<Body>, next: Next) -> Response {
    let started = Instant::now();
    let method = req.method().clone();
    let path = req.uri().path().to_string();

    // Reuse a well-formed client id; never trust it blindly: length-capped
    // and restricted to safe characters to prevent log injection.
    let incoming = req
        .headers()
        .get(REQUEST_ID_HEADER)
        .and_then(|v| v.to_str().ok())
        .map(str::trim)
        .filter(|v| !v.is_empty() && v.len() <= MAX_INCOMING_ID_LEN)
        .filter(|v| v.chars().all(is_safe_id_char));

    let id = incoming
        .map(str::to_string)
        .unwrap_or_else(|| Uuid::new_v4().to_string());

    req.extensions_mut().insert(RequestId(id.clone()));

    let mut response = next.run(req).await;

    if let Ok(value) = HeaderValue::from_str(&id) {
        response.headers_mut().insert(REQUEST_ID_HEADER, value);
    }

    info!(
        "{} {} -> {} {}ms [{}]",
        method,
        path,
        response.status().as_u16(),
        started.elapsed().as_millis(),
        id
    );

    response
}
