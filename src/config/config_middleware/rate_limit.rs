use axum::body::Body;
use axum::http::Method;
use axum::http::Request;
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::Response;
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::time::Instant;

// Async mutex so we don't block the runtime when updating buckets
use tokio::sync::Mutex as AsyncMutex;

lazy_static! {
    static ref LOGIN_BUCKETS: AsyncMutex<HashMap<String, TokenBucket>> =
        AsyncMutex::new(HashMap::new());
}

#[derive(Clone, Debug)]
struct TokenBucket {
    tokens: f64,
    last_refill: Instant,
}

// Configuration
const LOGIN_WINDOW_SECS: u64 = 60; // window in seconds
const LOGIN_CAPACITY: f64 = 15.0; // max tokens per window
const MAX_BUCKETS: usize = 100_000; // safety cap to avoid unbounded memory growth

fn refill_rate_per_sec() -> f64 {
    LOGIN_CAPACITY / LOGIN_WINDOW_SECS as f64
}

async fn allow_request_for_key(key: &str) -> bool {
    let now = Instant::now();
    let mut buckets = LOGIN_BUCKETS.lock().await;

    // Prevent unbounded growth of the buckets map
    if buckets.len() > MAX_BUCKETS && !buckets.contains_key(key) {
        return false;
    }

    let bucket = buckets.entry(key.to_string()).or_insert(TokenBucket {
        tokens: LOGIN_CAPACITY,
        last_refill: now,
    });

    let elapsed = now.duration_since(bucket.last_refill).as_secs_f64();
    let added = elapsed * refill_rate_per_sec();
    bucket.tokens = (bucket.tokens + added).min(LOGIN_CAPACITY);
    bucket.last_refill = now;

    if bucket.tokens >= 1.0 {
        bucket.tokens -= 1.0;
        true
    } else {
        false
    }
}

/// Global rate limit middleware for all routes. Limits by `X-Forwarded-For` or `x-real-ip` header.
pub async fn rate_limit_middleware(req: Request<Body>, next: Next) -> Result<Response, StatusCode> {
    // Allow CORS preflight and other safe methods through without counting them
    if req.method() == Method::OPTIONS {
        return Ok(next.run(req).await);
    }
    let headers = req.headers();
    let key = headers
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .or_else(|| headers.get("x-real-ip").and_then(|v| v.to_str().ok()))
        .map(|s| s.to_string())
        .unwrap_or_else(|| "unknown".to_string());

    if !allow_request_for_key(&key).await {
        return Err(StatusCode::TOO_MANY_REQUESTS);
    }

    Ok(next.run(req).await)
}
