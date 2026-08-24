use axum::body::Body;
use axum::http::{Request, Response, StatusCode, header};
use axum::middleware::Next;
use log::{debug, error, warn};
use sha2::{Digest, Sha256};

use crate::config::config_redis;

const CACHE_PREFIX: &str = "http_cache:";

/// Hard cap on cacheable response bodies. Anything larger is served
/// uncached to avoid bloating Redis with megabyte-scale payloads.
const MAX_CACHED_BODY_BYTES: usize = 1024 * 1024;

// ── Allowlist ────────────────────────────────────────────────────────────────
// Only these GET route families are cached. Matching is segment-aware:
// a rule matches when the path equals the rule or lives under it
// ("/v1/api/sale" does NOT match "/v1/api/sale_item").
// Value = TTL seconds.
//
// Tier 4  dashboards/views   30–120s   (expensive SQL functions/views)
// Tier 1  catalogs/reference 300–600s  (units, taxes, methods, categories…)
// Tier 2  business masters   60–300s   (users, customers, products…)
// Tier 3  transactional      15–60s    (sales, purchases, cash — smoothing)
pub(crate) const CACHE_RULES: &[(&str, u64)] = &[
    // Tier 4 — dashboards
    ("/v1/api/vw_inventory_stock", 30),
    ("/v1/api/vw_cash_journal_balance", 60),
    ("/v1/api/vw_daily_cash_cut", 60),
    ("/v1/api/vw_customer_account_summary", 60),
    ("/v1/api/vw_customer_invoice_aging", 60),
    ("/v1/api/vw_sales_with_payments", 60),
    ("/v1/api/vw_sale_items_detail", 60),
    ("/v1/api/vw_best_sellers_30d", 120),
    ("/v1/api/vw_sales_daily_summary", 120),
    // Tier 1 — catalogs / reference data
    ("/v1/api/units", 600),
    ("/v1/api/tax_profiles", 600),
    ("/v1/api/payment_methods", 600),
    ("/v1/api/category", 600),
    ("/v1/api/inventory_locations", 300),
    ("/v1/api/role_permissions", 300),
    ("/v1/api/role", 300),
    ("/v1/api/permission", 300),
    ("/v1/api/supplier", 300),
    // Tier 2 — business masters
    ("/v1/api/customer_credit_account", 180),
    ("/v1/api/customer", 180),
    ("/v1/api/product_price", 120),
    ("/v1/api/product_barcode", 120),
    ("/v1/api/add_product", 120),
    ("/v1/api/product", 120),
    ("/v1/api/user_role", 120),
    ("/v1/api/user", 120),
    ("/v1/api/discount", 45),
    ("/v1/api/product_lot", 60),
    // Tier 3 — transactional (read smoothing only)
    ("/v1/api/inventory_movement", 30),
    ("/v1/api/purchase_payment", 30),
    ("/v1/api/purchase_item", 30),
    ("/v1/api/purchase", 30),
    ("/v1/api/add_sale", 30),
    ("/v1/api/sale_payment", 30),
    ("/v1/api/sale_item", 30),
    ("/v1/api/sale", 30),
    ("/v1/api/cash_journal", 30),
    ("/v1/api/cash_entry", 30),
];

// Defense-in-depth: never cache these even if a future rule matches them.
pub(crate) const NEVER_CACHE_PREFIXES: &[&str] =
    &["/v1/api/auth", "/v1/api/audit", "/v1/api/health"];

// Cookies excluded from the cache-key hash: they vary per user/session but the
// cacheable endpoints return tenant-wide payloads identical for every caller.
// Hashing them would fragment the cache and tank the hit ratio.
const EXCLUDED_COOKIE_NAMES: &[&str] = &[
    "access_token",
    "refresh_token",
    "jwt",
    "sid",
    "session_id",
    "connect.sid",
];

pub async fn cache_middleware(
    req: Request<Body>,
    next: Next,
) -> Result<Response<Body>, StatusCode> {
    let method = req.method().to_string();
    let path = req.uri().path().to_string();

    let is_get = method == "GET";

    if is_get {
        if let Some(ttl) = resolve_ttl(&path) {
            let cache_key = build_cache_key(&method, req.uri(), req.headers());

            if let Some(cached) = try_get_cached(&cache_key).await {
                debug!("Cache HIT for {}", path);
                return Ok(cached);
            }

            debug!("Cache MISS for {}", path);
            let response = next.run(req).await;
            return store_if_cacheable(response, &cache_key, ttl).await;
        }
    } else {
        let response = next.run(req).await;
        // Successful mutations invalidate every cache entry that depends on
        // the affected resource family (plus aggregate dashboard views).
        if response.status().is_success() {
            invalidate_for_mutation(&path).await;
        }
        return Ok(response);
    }

    Ok(next.run(req).await)
}

pub(crate) fn rule_matches(path: &str, prefix: &str) -> bool {
    path == prefix || path.starts_with(&format!("{}/", prefix))
}

pub(crate) fn resolve_ttl(path: &str) -> Option<u64> {
    if NEVER_CACHE_PREFIXES.iter().any(|p| rule_matches(path, p)) {
        return None;
    }
    CACHE_RULES
        .iter()
        .find(|(prefix, _)| rule_matches(path, prefix))
        .map(|(_, ttl)| *ttl)
}

pub(crate) fn build_cache_key(
    method: &str,
    uri: &axum::http::Uri,
    headers: &axum::http::HeaderMap,
) -> String {
    let mut hasher = Sha256::new();
    hasher.update(method.as_bytes());
    hasher.update(uri.path().as_bytes());
    if let Some(query) = uri.query() {
        hasher.update(b"?");
        hasher.update(query.as_bytes());
    }

    if let Some(cookie) = headers.get(header::COOKIE).and_then(|v| v.to_str().ok()) {
        let filtered: Vec<&str> = cookie
            .split(';')
            .map(str::trim)
            .filter(|pair| {
                let name = pair.split('=').next().unwrap_or("").trim().to_lowercase();
                !EXCLUDED_COOKIE_NAMES.contains(&name.as_str())
            })
            .collect();
        hasher.update(filtered.join("; "));
    }

    let hash = hex::encode(hasher.finalize());
    format!(
        "{}{}:{}",
        CACHE_PREFIX,
        uri.path().replace(['/', '{', '}', ':'], "_"),
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
                    // Hop-by-hop and identity headers are rebuilt per response
                    if is_skipped_header(&k) {
                        continue;
                    }
                    if let (Ok(name), Ok(value)) = (
                        header::HeaderName::try_from(&k),
                        header::HeaderValue::try_from(&v),
                    ) {
                        response = response.header(name, value);
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

async fn store_if_cacheable(
    response: Response<Body>,
    cache_key: &str,
    rule_ttl: u64,
) -> Result<Response<Body>, StatusCode> {
    if !response.status().is_success() {
        return Ok(response);
    }

    // Responses that mint credentials must never be persisted or replayed.
    if response.headers().contains_key(header::SET_COOKIE) {
        return Ok(response);
    }

    let ttl = match effective_ttl(response.headers(), rule_ttl) {
        Some(ttl) => ttl,
        None => return Ok(response),
    };

    let (parts, body) = response.into_parts();
    let body_bytes = match axum::body::to_bytes(body, MAX_CACHED_BODY_BYTES).await {
        Ok(bytes) => bytes,
        Err(e) => {
            error!("Failed to read response body for caching: {}", e);
            return Ok(Response::from_parts(parts, Body::empty()));
        }
    };

    // Over the cap → serve uncached rather than polluting Redis.
    if body_bytes.len() >= MAX_CACHED_BODY_BYTES {
        return Ok(Response::from_parts(parts, Body::from(body_bytes)));
    }

    let body_str = String::from_utf8_lossy(&body_bytes).to_string();
    let headers: std::collections::HashMap<String, String> = parts
        .headers
        .iter()
        .filter_map(|(k, v)| v.to_str().ok().map(|vs| (k.to_string(), vs.to_string())))
        .filter(|(k, _)| !is_skipped_header(k))
        .collect();

    let cached = CachedResponse {
        status: parts.status.as_u16(),
        headers,
        body: body_str.clone(),
    };

    if let Ok(json) = serde_json::to_string(&cached)
        && let Err(e) = config_redis::set_raw(cache_key, &json, Some(ttl as usize)).await
    {
        error!("Cache set error: {}", e);
    }

    let mut rebuilt_response = Response::from_parts(parts, Body::from(body_str));
    rebuilt_response
        .headers_mut()
        .insert("X-Cache", header::HeaderValue::from_static("MISS"));
    Ok(rebuilt_response)
}

/// Combines the route-rule TTL with response cache directives.
/// Absent directives default to the route-rule TTL; `no-store` / `private`
/// disable caching entirely; an explicit `max-age` can only lower the rule TTL.
pub(crate) fn effective_ttl(headers: &axum::http::HeaderMap, rule_ttl: u64) -> Option<u64> {
    let Some(directive) = headers
        .get(header::CACHE_CONTROL)
        .and_then(|v| v.to_str().ok())
    else {
        // Handlers don't set Cache-Control; the route rule decides.
        return Some(rule_ttl);
    };

    if directive.contains("no-store") || directive.contains("private") {
        return None;
    }

    directive
        .split(',')
        .filter_map(|part| part.trim().strip_prefix("max-age="))
        .find_map(|secs| secs.parse::<u64>().ok())
        .map(|max_age| max_age.min(rule_ttl))
        .or(Some(rule_ttl))
}

fn is_skipped_header(name: &str) -> bool {
    const SKIPPED: &[&str] = &[
        "content-length",
        "transfer-encoding",
        "connection",
        "keep-alive",
        "date",
        "x-cache",
        // Per-request correlation id: never persisted or replayed from cache.
        "x-request-id",
    ];
    SKIPPED.iter().any(|s| name.eq_ignore_ascii_case(s))
}

// ── Mutation-driven invalidation ─────────────────────────────────────────────
// After any successful POST/PATCH/DELETE under /v1/api/<resource>, purge the
// HTTP-cache entries of every affected route family plus the aggregate
// dashboard views. Stock-affecting domains also purge the service-level
// `vw_inventory_stock:*` keys written by vw_inventory_stock_service.
//
// Patterns operate on the sanitized path used inside cache keys
// ('/' → '_'), so "_v1_api_sale*" covers sale, sale_item and sale_payment.

const STOCK_RAW_PATTERN: &str = "vw_inventory_stock:*";

async fn invalidate_for_mutation(path: &str) {
    let resource = path.split('/').nth(3); // /v1/api/<resource>/...
    let Some(resource) = resource else {
        return;
    };
    if resource.is_empty() {
        return;
    }

    let mut patterns: Vec<String> = Vec::new();

    let mut push = |p: &str| {
        let pat = format!("{}{}*", CACHE_PREFIX, p);
        if !patterns.contains(&pat) {
            patterns.push(pat);
        }
    };

    match resource {
        r if r.starts_with("user") => push("_v1_api_user"),
        r if r.starts_with("role") && r != "role" => push("_v1_api_role_permissions"),
        "role" => {
            push("_v1_api_role");
            push("_v1_api_role_permissions");
        }
        "permission" => {
            push("_v1_api_permission");
            push("_v1_api_role_permissions");
        }
        "units" => push("_v1_api_units"),
        "tax_profiles" => push("_v1_api_tax_profiles"),
        "payment_methods" => push("_v1_api_payment_methods"),
        "supplier" => {
            push("_v1_api_supplier");
            push("_v1_api_purchase");
        }
        r if r.starts_with("customer") => push("_v1_api_customer"),
        "category" => {
            push("_v1_api_category");
            push("_v1_api_product");
            push("_v1_api_add_product");
        }
        "product" | "product_barcode" | "product_price" | "product_lot" => {
            push("_v1_api_product");
            push("_v1_api_add_product");
        }
        r if r.starts_with("inventory") => push("_v1_api_inventory"),
        r if r.starts_with("sale") || r == "add_sale" => {
            push("_v1_api_sale");
            push("_v1_api_add_sale");
        }
        r if r.starts_with("purchase") => push("_v1_api_purchase"),
        r if r.starts_with("cash") => push("_v1_api_cash"),
        "discount" => push("_v1_api_discount"),
        _ => {}
    }

    // Aggregate dashboards are derived from nearly every domain: always purge.
    push("_v1_api_vw");

    let touches_stock = matches!(
        resource,
        "category"
            | "product"
            | "product_barcode"
            | "product_price"
            | "product_lot"
            | "inventory_movement"
            | "inventory_locations"
            | "add_sale"
            | "sale"
            | "sale_item"
            | "purchase"
            | "purchase_item"
    );

    for pat in &patterns {
        if let Err(e) = config_redis::del_pattern(pat).await {
            warn!("cache invalidation failed for {}: {}", pat, e);
        }
    }

    if touches_stock && let Err(e) = config_redis::del_pattern(STOCK_RAW_PATTERN).await {
        warn!("stock raw cache invalidation failed: {}", e);
    }

    debug!("cache invalidated for {} ({})", path, resource);
}

// ── Manual invalidation helpers (kept public for admin/tooling use) ──────────

pub async fn invalidate_cache_pattern(pattern: &str) -> Result<(), String> {
    let cache_pattern = format!("{}{}", CACHE_PREFIX, pattern);
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

#[derive(serde::Serialize, serde::Deserialize)]
struct CachedResponse {
    status: u16,
    headers: std::collections::HashMap<String, String>,
    body: String,
}
