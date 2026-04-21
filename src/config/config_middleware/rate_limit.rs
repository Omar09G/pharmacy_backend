use axum::body::Body;
use axum::extract::ConnectInfo;
use axum::http::Method;
use axum::http::Request;
use axum::http::StatusCode;
use axum::middleware::Next;
use axum::response::Response;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::LazyLock;
use std::time::Instant;

// Async mutex so we don't block the runtime when updating buckets
use tokio::sync::Mutex as AsyncMutex;

static RATE_BUCKETS: LazyLock<AsyncMutex<HashMap<String, TokenBucket>>> =
    LazyLock::new(|| AsyncMutex::new(HashMap::new()));

#[derive(Clone, Debug)]
#[allow(dead_code)] // capacity and refill_rate are stored for future per-bucket reconfiguration
struct TokenBucket {
    tokens: f64,
    last_refill: Instant,
    capacity: f64,
    refill_rate: f64, // tokens per second
}

// ── Login endpoint: strict — 10 attempts / 30 minutes ─────────────────────────
const LOGIN_CAPACITY: f64 = 10.0;
const LOGIN_WINDOW_SECS: f64 = 1_800.0; // 30 minutes

// ── Refresh endpoint: moderate — 30 requests / 10 minutes ─────────────────────
const REFRESH_CAPACITY: f64 = 30.0;
const REFRESH_WINDOW_SECS: f64 = 600.0; // 10 minutes

// ── General API: generous — 300 requests / 60 seconds ─────────────────────────
const API_CAPACITY: f64 = 300.0;
const API_WINDOW_SECS: f64 = 60.0;

const MAX_BUCKETS: usize = 100_000; // safety cap against unbounded memory growth

/// Determine bucket config from request path.
fn bucket_config(path: &str) -> (&'static str, f64, f64) {
    // Returns (bucket_label, capacity, window_secs)
    if path == "/v1/api/auth/login" {
        ("login", LOGIN_CAPACITY, LOGIN_WINDOW_SECS)
    } else if path == "/v1/api/auth/refresh" {
        ("refresh", REFRESH_CAPACITY, REFRESH_WINDOW_SECS)
    } else {
        ("api", API_CAPACITY, API_WINDOW_SECS)
    }
}

async fn allow_request(ip: &str, bucket_label: &str, capacity: f64, window_secs: f64) -> bool {
    let key = format!("{}:{}", bucket_label, ip);
    let now = Instant::now();
    let refill_rate = capacity / window_secs;

    let mut buckets = RATE_BUCKETS.lock().await;

    if buckets.len() > MAX_BUCKETS && !buckets.contains_key(&key) {
        return false;
    }

    let bucket = buckets.entry(key).or_insert(TokenBucket {
        tokens: capacity,
        last_refill: now,
        capacity,
        refill_rate,
    });

    let elapsed = now.duration_since(bucket.last_refill).as_secs_f64();
    let added = elapsed * refill_rate;
    bucket.tokens = (bucket.tokens + added).min(capacity);
    bucket.last_refill = now;

    if bucket.tokens >= 1.0 {
        bucket.tokens -= 1.0;
        true
    } else {
        false
    }
}

/// Resolve the real client IP:
/// 1. Use the actual TCP peer address from ConnectInfo (safe, can't be spoofed).
/// 2. Only fall back to X-Forwarded-For / X-Real-IP if TRUSTED_PROXY_HEADER=true is set,
///    meaning the operator explicitly trusts the proxy infrastructure.
fn resolve_client_ip(req: &Request<Body>) -> String {
    // Prefer real peer address — cannot be spoofed by client
    if let Some(ConnectInfo(addr)) = req.extensions().get::<ConnectInfo<SocketAddr>>() {
        let peer_ip = addr.ip().to_string();
        // Only trust forwarded headers when operator explicitly opts-in AND
        // the direct peer is known to be a trusted proxy.
        let use_proxy_headers = std::env::var("TRUSTED_PROXY_HEADER")
            .unwrap_or_default()
            .to_lowercase()
            == "true";

        if use_proxy_headers {
            if let Some(forwarded) = req
                .headers()
                .get("x-forwarded-for")
                .and_then(|v| v.to_str().ok())
                .and_then(|s| s.split(',').next())
                .map(|s| s.trim().to_string())
            {
                return forwarded;
            }
            if let Some(real_ip) = req
                .headers()
                .get("x-real-ip")
                .and_then(|v| v.to_str().ok())
                .map(|s| s.to_string())
            {
                return real_ip;
            }
        }

        return peer_ip;
    }

    // Fallback if ConnectInfo is unavailable (should not happen in normal setup)
    "unknown".to_string()
}

/// Rate limiting middleware with per-endpoint buckets.
/// - Login:   10 req / 30 min per IP
/// - Refresh: 30 req / 10 min per IP
/// - API:    300 req / 60 sec  per IP
pub async fn rate_limit_middleware(req: Request<Body>, next: Next) -> Result<Response, StatusCode> {
    // Allow CORS preflight through without consuming tokens
    if req.method() == Method::OPTIONS {
        return Ok(next.run(req).await);
    }

    let path = req.uri().path().to_string();
    let ip = resolve_client_ip(&req);
    let (label, capacity, window) = bucket_config(&path);

    if !allow_request(&ip, label, capacity, window).await {
        return Err(StatusCode::TOO_MANY_REQUESTS);
    }

    Ok(next.run(req).await)
}

