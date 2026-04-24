use axum::body::Body;
use axum::http::{HeaderValue, Method, Request, StatusCode};
use axum::middleware::Next;
use axum::response::Response;
use std::env;

/// CORS middleware that respects `CORS_ALLOWED_ORIGINS` environment variable.
/// - If `CORS_ALLOWED_ORIGINS` is `*` or unset, it echoes back the request Origin
///   (credentials mode requires a specific origin, not `*`).
/// - Otherwise it should be a comma-separated list of allowed origins and only requests
///   with a matching `Origin` header will be allowed (browsers will enforce CORS).
///
/// Always sets `Access-Control-Allow-Credentials: true` so that HttpOnly cookies
/// are sent cross-origin.
pub async fn cors_middleware(req: Request<Body>, next: Next) -> Result<Response, StatusCode> {
    // If CORS_ALLOWED_ORIGINS is not set or empty, deny all cross-origin requests by default.
    // Set it to "*" explicitly to allow all origins (not recommended in production).
    let allowed = env::var("CORS_ALLOWED_ORIGINS").unwrap_or_default();
    let allowed = allowed.trim();
    let allow_all = allowed == "*";
    let allowed_list: Vec<&str> = if allow_all {
        vec![]
    } else {
        allowed
            .split(',')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect()
    };

    let origin_header = req
        .headers()
        .get("origin")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    // Preflight handling
    if req.method() == Method::OPTIONS {
        // If origin is present and not in the allow-list, reject (unless allow_all)
        if let Some(ref origin) = origin_header {
            if !allow_all && !allowed_list.iter().any(|a| a == &origin.as_str()) {
                return Err(StatusCode::FORBIDDEN);
            }
        }

        let mut res = Response::new(Body::empty());
        let headers = res.headers_mut();

        // Echo back the origin (required when credentials=true; wildcard `*` is forbidden)
        let allow_origin_value = if let Some(ref origin) = origin_header {
            HeaderValue::from_str(origin).unwrap_or_else(|_| HeaderValue::from_static("null"))
        } else {
            HeaderValue::from_static("null")
        };

        headers.insert("access-control-allow-origin", allow_origin_value);
        headers.insert(
            "access-control-allow-methods",
            HeaderValue::from_static("GET,POST,PUT,PATCH,DELETE,OPTIONS"),
        );
        headers.insert(
            "access-control-allow-headers",
            HeaderValue::from_static("authorization,content-type,x-client-platform"),
        );
        headers.insert(
            "access-control-allow-credentials",
            HeaderValue::from_static("true"),
        );
        return Ok(res);
    }

    // For non-preflight requests, if origin is present and not allowed, reject
    if let Some(ref origin) = origin_header {
        if !allow_all && !allowed_list.iter().any(|a| a == &origin.as_str()) {
            return Err(StatusCode::FORBIDDEN);
        }
    }

    // Call the next handler and then attach CORS headers
    let mut response = next.run(req).await;
    let headers = response.headers_mut();

    if let Some(origin) = origin_header {
        if let Ok(val) = HeaderValue::from_str(&origin) {
            headers.insert("access-control-allow-origin", val);
        }
    }
    headers.insert(
        "access-control-allow-credentials",
        HeaderValue::from_static("true"),
    );

    Ok(response)
}
