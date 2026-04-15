use axum::body::Body;
use axum::http::{Method, Request, StatusCode};
use axum::middleware::Next;
use axum::response::Response;

/// Middleware that validates `Content-Type: application/json` for mutating requests.
/// Rejects with `415 Unsupported Media Type` when missing or not JSON.
pub async fn content_type_middleware(
    req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    // Allow preflight and safe methods through
    let method = req.method();
    if method == Method::OPTIONS
        || method == Method::GET
        || method == Method::HEAD
        || method == Method::DELETE
    {
        return Ok(next.run(req).await);
    }

    // Endpoints that accept empty bodies — skip content-type check
    let path = req.uri().path();
    if path == "/v1/api/auth/logout" || path == "/v1/api/auth/refresh" {
        return Ok(next.run(req).await);
    }

    // For POST/PUT/PATCH and others with bodies, require application/json
    if method == Method::POST || method == Method::PUT || method == Method::PATCH {
        let ct = req
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.trim().to_ascii_lowercase());

        match ct {
            Some(v) if v.starts_with("application/json") => Ok(next.run(req).await),
            _ => Err(StatusCode::UNSUPPORTED_MEDIA_TYPE),
        }
    } else {
        // For any other method, pass through
        Ok(next.run(req).await)
    }
}
