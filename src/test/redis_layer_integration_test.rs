//! Integration tests for the Redis layer (pool, circuit breaker, SCAN-based
//! invalidation and the atomic INCR+EXPIRE script).
//!
//! These tests require a reachable Redis instance. REDIS_URL is honoured;
//! when no Redis is available the tests fail loudly so regressions surface.
//!
//! Everything runs inside ONE #[tokio::test] because the module under test
//! keeps a process-global pool bound to the runtime that created it — exactly
//! like production (single runtime), unlike parallel test runtimes.

use crate::config::config_redis;
use crate::test::redis_state_lock;

#[tokio::test]
async fn redis_layer_behaviour() {
    // Exclusive access to the global pool/circuit-breaker state, and full
    // teardown at the end so later tests see a pristine (uninitialized) Redis.
    let _guard = redis_state_lock().lock().await;

    let url = std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379/".into());
    // Reset any breaker trips left by earlier tests before re-initializing.
    config_redis::close_redis().await;
    config_redis::init_redis(&url)
        .await
        .expect("redis must be reachable for integration tests");

    // ── raw roundtrip with TTL ────────────────────────────────────────────
    config_redis::set_raw("it:test:raw", "hello", Some(60))
        .await
        .unwrap();
    assert_eq!(
        config_redis::get_raw("it:test:raw")
            .await
            .unwrap()
            .as_deref(),
        Some("hello")
    );
    config_redis::del_key("it:test:raw").await.unwrap();
    assert_eq!(config_redis::get_raw("it:test:raw").await.unwrap(), None);

    // ── json kv roundtrip ─────────────────────────────────────────────────
    #[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
    struct Payload {
        a: u32,
        b: String,
    }
    let payload = Payload {
        a: 42,
        b: "pharmacy".into(),
    };
    config_redis::set_json("it:test:json", &payload, 60)
        .await
        .unwrap();
    assert_eq!(
        config_redis::get_json::<Payload>("it:test:json")
            .await
            .unwrap(),
        Some(payload)
    );

    // ── del_pattern uses SCAN and purges only matching keys ──────────────
    for i in 0..25 {
        config_redis::set_raw(&format!("it:test:scan:key{i}"), "x", Some(60))
            .await
            .unwrap();
    }
    config_redis::set_raw("it:test:scan:sibling", "keep", Some(60))
        .await
        .unwrap();
    config_redis::del_pattern("it:test:scan:key*")
        .await
        .unwrap();
    for i in 0..25 {
        assert_eq!(
            config_redis::get_raw(&format!("it:test:scan:key{i}"))
                .await
                .unwrap(),
            None,
            "key{i} should have been purged"
        );
    }
    assert_eq!(
        config_redis::get_raw("it:test:scan:sibling")
            .await
            .unwrap()
            .as_deref(),
        Some("keep")
    );

    // ── incr_with_expire: atomic counter + TTL ────────────────────────────
    config_redis::del_key("it:test:counter").await.unwrap();
    assert_eq!(
        config_redis::incr_with_expire("it:test:counter", 30)
            .await
            .unwrap(),
        1
    );
    assert_eq!(
        config_redis::incr_with_expire("it:test:counter", 30)
            .await
            .unwrap(),
        2
    );

    let pool = deadpool_redis::Config::from_url(url.clone())
        .create_pool(Some(deadpool_redis::Runtime::Tokio1))
        .unwrap();
    let mut conn = pool.get().await.unwrap();
    let ttl: i64 = redis::cmd("TTL")
        .arg("it:test:counter")
        .query_async(&mut conn)
        .await
        .unwrap();
    assert!(ttl > 0 && ttl <= 30, "expected TTL in (0,30], got {ttl}");

    // ── fast-fail against a dead endpoint (no long hangs) ─────────────────
    let start = std::time::Instant::now();
    let bad = deadpool_redis::Config::from_url("redis://127.0.0.1:1/".to_string())
        .create_pool(Some(deadpool_redis::Runtime::Tokio1))
        .unwrap()
        .get()
        .await;
    assert!(bad.is_err(), "checkout from dead port must fail");
    assert!(
        start.elapsed() < std::time::Duration::from_secs(5),
        "failure must be fast, took {:?}",
        start.elapsed()
    );

    // Main path still healthy afterwards (circuit breaker closed).
    assert_eq!(
        config_redis::get_raw("it:test:scan:sibling")
            .await
            .unwrap()
            .as_deref(),
        Some("keep")
    );

    config_redis::del_key("it:test:json").await.unwrap();
    config_redis::del_key("it:test:counter").await.unwrap();
    config_redis::del_key("it:test:scan:sibling").await.unwrap();

    // Teardown: release the global pool and reset the circuit breaker so
    // subsequent tests (e.g. api_test's DEGRADED assertions) start clean.
    config_redis::close_redis().await;
}
