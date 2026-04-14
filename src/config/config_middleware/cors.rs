use axum::body::Body;
use axum::http::{HeaderValue, Method, Request, StatusCode};
use axum::middleware::Next;
use axum::response::Response;
use std::env;

/// CORS middleware that respects `CORS_ALLOWED_ORIGINS` environment variable.
/// - If `CORS_ALLOWED_ORIGINS` is `*` or unset, behaves like wildcard.
/// - Otherwise it should be a comma-separated list of allowed origins and only requests
///   with a matching `Origin` header will be allowed (browsers will enforce CORS).
pub async fn cors_middleware(req: Request<Body>, next: Next) -> Result<Response, StatusCode> {
    let allowed = env::var("CORS_ALLOWED_ORIGINS").unwrap_or_else(|_| "*".to_string());
    let allow_all = allowed.trim() == "*";
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
        // If origin is present and not allowed, reject
        if let Some(ref origin) = origin_header {
            if !allow_all && !allowed_list.iter().any(|a| a == &origin.as_str()) {
                return Err(StatusCode::FORBIDDEN);
            }
        }

        let mut res = Response::new(Body::empty());
        let headers = res.headers_mut();
        let allow_origin_value = if allow_all {
            HeaderValue::from_static("*")
        } else if let Some(origin) = origin_header.clone() {
            HeaderValue::from_str(&origin).unwrap_or_else(|_| HeaderValue::from_static("null"))
        } else {
            HeaderValue::from_static("*")
        };

        headers.insert("access-control-allow-origin", allow_origin_value);
        headers.insert(
            "access-control-allow-methods",
            HeaderValue::from_static("GET,POST,PUT,PATCH,DELETE,OPTIONS"),
        );
        headers.insert(
            "access-control-allow-headers",
            HeaderValue::from_static("authorization,content-type"),
        );
        if !allow_all {
            headers.insert(
                "access-control-allow-credentials",
                HeaderValue::from_static("true"),
            );
        }
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
    if allow_all {
        headers.insert("access-control-allow-origin", HeaderValue::from_static("*"));
    } else if let Some(origin) = origin_header {
        if let Ok(val) = HeaderValue::from_str(&origin) {
            headers.insert("access-control-allow-origin", val);
        }
    }
    if !allow_all {
        headers.insert(
            "access-control-allow-credentials",
            HeaderValue::from_static("true"),
        );
    }
    Ok(response)
}
