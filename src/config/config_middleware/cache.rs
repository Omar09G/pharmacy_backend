use axum::body::Body;
use axum::http::{Request, Response, StatusCode, header};
use axum::middleware::Next;
use log::{debug, error, warn};
use sha2::{Digest, Sha256};

use crate::config::config_redis;

const DEFAULT_TTL_SECS: usize = 60;
const CACHE_PREFIX: &str = "http_cache:";

pub async fn cache_middleware(
    req: Request<Body>,
    next: Next,
) -> Result<Response<Body>, StatusCode> {
    let path = req.uri().path().to_string();
    let method = req.method().to_string();

    if !is_cacheable(&method, &path) {
        return Ok(next.run(req).await);
    }

    let cache_key = build_cache_key(&method, &path, req.headers());

    if let Some(cached) = try_get_cached(&cache_key).await {
        debug!("Cache HIT for {}", path);
        return Ok(cached);
    }

    debug!("Cache MISS for {}", path);
    let response = next.run(req).await;

    if response.status().is_success() {
        let ttl = extract_cache_ttl(&response);
        let (parts, body) = response.into_parts();
        let body_bytes = match axum::body::to_bytes(body, usize::MAX).await {
            Ok(bytes) => bytes,
            Err(e) => {
                error!("Failed to read response body for caching: {}", e);
                return Ok(Response::from_parts(parts, Body::empty()));
            }
        };

        let body_str = String::from_utf8_lossy(&body_bytes).to_string();
        let headers: std::collections::HashMap<String, String> = parts
            .headers
            .iter()
            .filter_map(|(k, v)| v.to_str().ok().map(|vs| (k.to_string(), vs.to_string())))
            .collect();

        let cached = CachedResponse {
            status: parts.status.as_u16(),
            headers,
            body: body_str.clone(),
        };

        if let Ok(json) = serde_json::to_string(&cached) {
            if let Err(e) = config_redis::set_raw(&cache_key, &json, Some(ttl)).await {
                error!("Cache set error: {}", e);
            }
        }

        let mut rebuilt_response = Response::from_parts(parts, Body::from(body_str));
        rebuilt_response
            .headers_mut()
            .insert("X-Cache", header::HeaderValue::from_static("MISS"));
        Ok(rebuilt_response)
    } else {
        Ok(response)
    }
}

fn is_cacheable(method: &str, path: &str) -> bool {
    if method != "GET" {
        return false;
    }

    let skip_prefixes = [
        "/v1/api/auth",
        "/v1/api/user",
        "/v1/api/role",
        "/v1/api/permission",
        "/v1/api/user_role",
        "/v1/api/role_permissions",
        "/v1/api/audit",
        "/v1/api/product",
        "/v1/api/order",
        "/v1/api/cart",
        "/v1/api/payment",
        "/v1/api/checkout",
        "/v1/api/sale",
        "/v1/api/",
    ];

    for prefix in &skip_prefixes {
        if path.starts_with(prefix) {
            return false;
        }
    }

    true
}

fn build_cache_key(method: &str, path: &str, headers: &axum::http::HeaderMap) -> String {
    let mut hasher = Sha256::new();
    hasher.update(method.as_bytes());
    hasher.update(path.as_bytes());

    if let Some(query) = path.split('?').nth(1) {
        hasher.update(query.as_bytes());
    }

    if let Some(cookie) = headers.get("cookie") {
        hasher.update(cookie.as_bytes());
    }

    let hash = hex::encode(hasher.finalize());
    format!(
        "{}{}:{}",
        CACHE_PREFIX,
        path.replace(['/', '{', '}', ':'], "_"),
        hash
    )
}

async fn try_get_cached(key: &str) -> Option<Response<Body>> {
    match config_redis::get_raw(key).await {
        Ok(Some(cached_json)) => match serde_json::from_str::<CachedResponse>(&cached_json) {
            Ok(cached) => {
                let mut response = Response::builder()
                    .status(StatusCode::from_u16(cached.status).unwrap_or(StatusCode::OK));

                for (k, v) in cached.headers {
                    if k.eq_ignore_ascii_case("x-cache") {
                        continue;
                    }
                    if let Ok(name) = header::HeaderName::try_from(&k) {
                        if let Ok(value) = header::HeaderValue::try_from(&v) {
                            response = response.header(name, value);
                        }
                    }
                }

                response = response.header("X-Cache", "HIT");

                match response.body(Body::from(cached.body)) {
                    Ok(resp) => Some(resp),
                    Err(e) => {
                        error!("Failed to build cached response: {}", e);
                        None
                    }
                }
            }
            Err(e) => {
                error!("Failed to deserialize cached response: {}", e);
                None
            }
        },
        Ok(None) => None,
        Err(e) => {
            warn!("Cache get error (continuing without cache): {}", e);
            None
        }
    }
}

fn extract_cache_ttl(response: &Response<Body>) -> usize {
    if let Some(cache_control) = response.headers().get(header::CACHE_CONTROL) {
        if let Ok(directive) = cache_control.to_str() {
            if directive.contains("no-store") || directive.contains("private") {
                return 0;
            }

            for part in directive.split(',') {
                let part = part.trim();
                if let Some(max_age) = part.strip_prefix("max-age=") {
                    if let Ok(secs) = max_age.parse::<usize>() {
                        return secs.min(3600);
                    }
                }
            }
        }
    }

    DEFAULT_TTL_SECS
}

#[derive(serde::Serialize, serde::Deserialize)]
struct CachedResponse {
    status: u16,
    headers: std::collections::HashMap<String, String>,
    body: String,
}

pub async fn invalidate_cache_pattern(pattern: &str) -> Result<(), String> {
    let cache_pattern = format!("{}*", CACHE_PREFIX);
    config_redis::del_pattern(&cache_pattern).await?;
    debug!("Cache invalidated for pattern: {}", pattern);
    Ok(())
}

pub async fn invalidate_cache_path(path: &str) -> Result<(), String> {
    let cache_pattern = format!(
        "{}{}*",
        CACHE_PREFIX,
        path.replace(['/', '{', '}', ':'], "_")
    );
    config_redis::del_pattern(&cache_pattern).await?;
    debug!("Cache invalidated for path: {}", path);
    Ok(())
}
