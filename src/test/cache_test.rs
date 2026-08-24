//! Unit tests for the HTTP cache middleware internals.
//!
//! Lives in `src/test/` per project convention: no `#[cfg(test)]` blocks
//! inside production source files. Items under test are exposed as
//! `pub(crate)` by `config_middleware::cache`.

use axum::http::{HeaderMap, Uri, header};

use crate::config::config_middleware::cache::{
    CACHE_RULES, NEVER_CACHE_PREFIXES, build_cache_key, effective_ttl, resolve_ttl,
};

#[test]
fn rules_are_segment_aware() {
    // "/sale" must not swallow "/sale_item" or "/sale_payment".
    assert!(resolve_ttl("/v1/api/sale").is_some());
    assert!(resolve_ttl("/v1/api/sale/123").is_some());
    assert_eq!(
        resolve_ttl("/v1/api/sale_item"),
        CACHE_RULES
            .iter()
            .find(|(p, _)| *p == "/v1/api/sale_item")
            .map(|(_, t)| *t)
    );
}

#[test]
fn never_cache_prefixes_win() {
    for p in NEVER_CACHE_PREFIXES {
        assert!(resolve_ttl(p).is_none(), "{p} must never be cached");
        assert!(
            resolve_ttl(&format!("{p}/login")).is_none(),
            "{p}/login must never be cached"
        );
    }
}

#[test]
fn unknown_paths_are_not_cached() {
    assert!(resolve_ttl("/v1/api/unknown_resource").is_none());
    assert!(resolve_ttl("/").is_none());
}

/// Guards against new GET route families being added without a cache rule.
#[test]
fn every_known_route_family_has_a_rule() {
    const CACHED_FAMILIES: &[&str] = &[
        // dashboards
        "vw_best_sellers_30d",
        "vw_cash_journal_balance",
        "vw_customer_account_summary",
        "vw_customer_invoice_aging",
        "vw_daily_cash_cut",
        "vw_inventory_stock",
        "vw_sale_items_detail",
        "vw_sales_daily_summary",
        "vw_sales_with_payments",
        // catalogs / reference
        "inventory_locations",
        "payment_methods",
        "permission",
        "role_permissions",
        "role",
        "supplier",
        "tax_profiles",
        "units",
        "category",
        // business masters
        "add_product",
        "customer",
        "customer_credit_account",
        "discount",
        "product",
        "product_barcode",
        "product_lot",
        "product_price",
        "user",
        "user_role",
        // transactional
        "add_sale",
        "cash_entry",
        "cash_journal",
        "inventory_movement",
        "purchase",
        "purchase_item",
        "purchase_payment",
        "sale",
        "sale_item",
        "sale_payment",
    ];
    for family in CACHED_FAMILIES {
        let path = format!("/v1/api/{family}");
        assert!(
            resolve_ttl(&path).is_some(),
            "GET {path} has no cache rule; add it to CACHE_RULES or NEVER_CACHE_PREFIXES"
        );
    }
}

#[test]
fn cache_key_varies_with_method_path_and_query() {
    let mk =
        |m: &str, uri: &str| build_cache_key(m, &uri.parse::<Uri>().unwrap(), &HeaderMap::new());
    assert_ne!(
        mk("GET", "/v1/api/product"),
        mk("GET", "/v1/api/product?page=2")
    );
    assert_ne!(mk("GET", "/v1/api/product"), mk("HEAD", "/v1/api/product"));
}

#[test]
fn excluded_cookies_do_not_fragment_the_key() {
    let uri = "/v1/api/product".parse::<Uri>().unwrap();
    let mut headers = HeaderMap::new();
    headers.insert(
        header::COOKIE,
        header::HeaderValue::from_static("access_token=abc; theme=dark"),
    );
    let with_cookie = build_cache_key("GET", &uri, &headers);

    let mut only_theme = HeaderMap::new();
    only_theme.insert(
        header::COOKIE,
        header::HeaderValue::from_static("theme=dark"),
    );
    let same_session = build_cache_key("GET", &uri, &only_theme);

    assert_eq!(with_cookie, same_session);
}

#[test]
fn effective_ttl_respects_directives() {
    let mut headers = HeaderMap::new();
    headers.insert(
        header::CACHE_CONTROL,
        header::HeaderValue::from_static("no-store"),
    );
    assert_eq!(effective_ttl(&headers, 60), None);

    let mut private = HeaderMap::new();
    private.insert(
        header::CACHE_CONTROL,
        header::HeaderValue::from_static("private"),
    );
    assert_eq!(effective_ttl(&private, 60), None);

    let mut max_age = HeaderMap::new();
    max_age.insert(
        header::CACHE_CONTROL,
        header::HeaderValue::from_static("max-age=10"),
    );
    assert_eq!(effective_ttl(&max_age, 60), Some(10));

    // max-age above the rule can only lower, never raise.
    let mut high = HeaderMap::new();
    high.insert(
        header::CACHE_CONTROL,
        header::HeaderValue::from_static("max-age=9999"),
    );
    assert_eq!(effective_ttl(&high, 60), Some(60));

    // Absent directive falls back to the route rule.
    assert_eq!(effective_ttl(&HeaderMap::new(), 45), Some(45));
}
