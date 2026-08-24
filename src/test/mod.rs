//! Centralized test suite.
//!
//! Project convention: **all** tests live here, organized by module under
//! test — never as inline `#[cfg(test)] mod tests` blocks inside production
//! source files.
//!
//! Layout:
//! - `api_test`                    → HTTP router end-to-end (middlewares included)
//! - `cache_test`                  → HTTP cache rules/keys/TTL semantics
//! - `jwt_test`                    → JWT generate/validate/revoke (HMAC path)
//! - `pagination_test`             → PaginationParams deserialization boundaries
//! - `redis_layer_integration_test`→ Redis pool/circuit-breaker/SCAN (needs live Redis)
//! - `request_id_test`             → X-Request-ID correlation middleware
//!
//! Items that need crate-internal access are exposed by their modules as
//! `pub(crate)`. The suite is only compiled when testing (`#[cfg(test)]` in
//! `lib.rs`), so release builds carry zero test code.

pub mod api_test;
pub mod cache_test;
pub mod jwt_test;
pub mod pagination_test;
pub mod redis_layer_integration_test;
pub mod request_id_test;

use std::sync::OnceLock;

/// Serialises every test that touches the process-global Redis state
/// (`REDIS_POOL` + circuit breaker).
///
/// Rationale: the whole suite now compiles into ONE test binary, and
/// `#[tokio::test]` runs tests on parallel threads. `api_test` requires an
/// *uninitialized* Redis (it asserts the DEGRADED/fallback paths) while
/// `redis_layer_integration_test` initializes a live pool — running both
/// concurrently would interleave pool/circuit-breaker mutations and flake.
/// Hold this lock for the duration of such tests; `jwt_test` shows the same
/// pattern for env-var isolation (`JWT_ENV_LOCK`).
pub(crate) fn redis_state_lock() -> &'static tokio::sync::Mutex<()> {
    static LOCK: OnceLock<tokio::sync::Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(tokio::sync::Mutex::default)
}
