use redis::AsyncCommands;
use std::sync::LazyLock;
use std::sync::atomic::{AtomicI64, AtomicUsize, Ordering};
use std::time::Duration;

use base64::Engine;
use deadpool_redis::{Config as PoolConfig, Pool, PoolConfig as DpPoolConfig, Runtime};
use log::{info, warn};

// ── Connection pool ──────────────────────────────────────────────────────────
// A real pool avoids opening a TCP connection per operation (latency + socket
// exhaustion under load). Size is configurable via REDIS_POOL_MAX.
static REDIS_POOL: LazyLock<parking_lot::RwLock<Option<Pool>>> =
    LazyLock::new(|| parking_lot::RwLock::new(None));

const DEFAULT_POOL_MAX: usize = 16;

// Per-operation deadline. Cache reads/writes are small payloads; if Redis
// takes longer than this the request is served from the database instead of
// stalling the caller.
const OP_TIMEOUT_MS: u64 = 500;

// ── Circuit breaker ──────────────────────────────────────────────────────────
// Prevents cascading latency when Redis is down: after N consecutive failures
// the circuit opens and calls fail fast for a cooldown window, then a single
// probe is allowed through (half-open). Successful probes close the circuit.
const CB_FAILURE_THRESHOLD: usize = 3;
const CB_COOLDOWN_SECS: i64 = 10;

static CB_FAILURES: AtomicUsize = AtomicUsize::new(0);
static CB_OPENED_AT: AtomicI64 = AtomicI64::new(0);

fn now_secs() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

/// Shuts the pool down so in-flight connections can be released on shutdown.
pub async fn close_redis() {
    *REDIS_POOL.write() = None;
    CB_FAILURES.store(0, Ordering::Relaxed);
    info!("Redis pool closed");
}

pub async fn init_redis(url: &str) -> Result<(), String> {
    info!("Initializing Redis pool at {}", url);
    let max_size = std::env::var("REDIS_POOL_MAX")
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .filter(|&v| v > 0 && v <= 256)
        .unwrap_or(DEFAULT_POOL_MAX);

    let mut pool_cfg = PoolConfig::from_url(url);
    let mut dp_pool = DpPoolConfig::new(max_size);
    // Fail fast when the pool is saturated or Redis is unreachable.
    dp_pool.timeouts.wait = Some(Duration::from_millis(OP_TIMEOUT_MS));
    dp_pool.timeouts.create = Some(Duration::from_secs(2));
    dp_pool.timeouts.recycle = Some(Duration::from_millis(OP_TIMEOUT_MS));
    pool_cfg.pool = Some(dp_pool);

    let pool = pool_cfg
        .create_pool(Some(Runtime::Tokio1))
        .map_err(|e| format!("failed to create redis pool: {}", e))?;

    // Eagerly validate connectivity once so misconfiguration surfaces at boot
    // (the app still starts without Redis — callers degrade gracefully).
    match pool.get().await {
        Ok(mut conn) => {
            if let Err(e) = redis::cmd("PING").query_async::<_, ()>(&mut conn).await {
                warn!("Redis PING failed after init (will retry lazily): {}", e);
            }
        }
        Err(e) => warn!("Redis pool warm-up failed (will retry lazily): {}", e),
    }

    *REDIS_POOL.write() = Some(pool);
    info!("Redis pool ready (max_size={})", max_size);
    Ok(())
}

fn pool() -> Option<Pool> {
    REDIS_POOL.read().clone()
}

async fn try_connection() -> Result<deadpool_redis::Connection, String> {
    match pool() {
        Some(p) => p
            .get()
            .await
            .map_err(|e| format!("redis pool checkout error: {}", e)),
        None => Err("redis pool not initialized".to_string()),
    }
}

/// Returns a pooled Redis connection guarded by the circuit breaker and an
/// operation deadline. All cache operations should go through this function so
/// that a Redis outage fails fast instead of stalling every request.
pub async fn get_connection() -> Result<deadpool_redis::Connection, String> {
    let failures = CB_FAILURES.load(Ordering::Relaxed);
    if failures >= CB_FAILURE_THRESHOLD {
        let remaining = CB_COOLDOWN_SECS - (now_secs() - CB_OPENED_AT.load(Ordering::Relaxed));
        if remaining > 0 {
            return Err(format!("redis circuit open (will retry in {}s)", remaining));
        }
        // Half-open: allow this call through as a probe.
    }

    match tokio::time::timeout(Duration::from_millis(OP_TIMEOUT_MS), try_connection()).await {
        Ok(Ok(conn)) => {
            if failures > 0 {
                info!("Redis recovered; circuit closed");
            }
            CB_FAILURES.store(0, Ordering::Relaxed);
            Ok(conn)
        }
        Ok(Err(e)) => {
            record_failure();
            Err(e)
        }
        Err(_) => {
            record_failure();
            Err("redis checkout timed out".to_string())
        }
    }
}

fn record_failure() {
    let prev = CB_FAILURES.fetch_add(1, Ordering::Relaxed);
    if prev + 1 == CB_FAILURE_THRESHOLD {
        CB_OPENED_AT.store(now_secs(), Ordering::Relaxed);
        warn!(
            "Redis circuit breaker OPEN after {} consecutive failures; \
             failing fast for {}s",
            CB_FAILURE_THRESHOLD, CB_COOLDOWN_SECS
        );
    }
}

/// Runs a Redis command with a hard deadline; converts timeouts into errors
/// so the circuit breaker can react to a hung Redis server.
async fn with_deadline<T>(
    fut: impl std::future::Future<Output = Result<T, String>>,
) -> Result<T, String> {
    match tokio::time::timeout(Duration::from_millis(OP_TIMEOUT_MS), fut).await {
        Ok(r) => r,
        Err(_) => {
            record_failure();
            Err("redis operation timed out".to_string())
        }
    }
}

/// Atomically increments `key` and sets TTL only on the first increment,
/// using a single round-trip Lua script (no INCR/EXPIRE race window).
pub async fn incr_with_expire(key: &str, ttl_secs: usize) -> Result<i64, String> {
    let mut conn = get_connection().await?;
    with_deadline(async move {
        let res: i64 = redis::cmd("EVAL")
            .arg(
                "local c = redis.call('INCR', KEYS[1]) \
                 if c == 1 then redis.call('EXPIRE', KEYS[1], ARGV[1]) end \
                 return c",
            )
            .arg(1)
            .arg(key)
            .arg(ttl_secs)
            .query_async(&mut conn)
            .await
            .map_err(|e| format!("redis incr_with_expire failed: {}", e))?;
        Ok(res)
    })
    .await
}

pub async fn set_kv(key: &str, value: &[u8], ttl_secs: usize) -> Result<(), String> {
    let mut conn = get_connection().await?;
    let b64 = base64::engine::general_purpose::STANDARD.encode(value);
    with_deadline(async move {
        let _: () = redis::cmd("SET")
            .arg(key)
            .arg(b64)
            .arg("EX")
            .arg(ttl_secs)
            .query_async(&mut conn)
            .await
            .map_err(|e| format!("redis set failed: {}", e))?;
        Ok(())
    })
    .await
}

pub async fn get_kv(key: &str) -> Result<Option<Vec<u8>>, String> {
    let mut conn = get_connection().await?;
    with_deadline(async move {
        let res: Option<String> = conn
            .get(key)
            .await
            .map_err(|e| format!("redis get failed: {}", e))?;
        if let Some(s) = res {
            let decoded = base64::engine::general_purpose::STANDARD
                .decode(s)
                .map_err(|e| format!("base64 decode failed: {}", e))?;
            Ok(Some(decoded))
        } else {
            Ok(None)
        }
    })
    .await
}

pub async fn set_json<T: serde::Serialize>(
    key: &str,
    value: &T,
    ttl_secs: usize,
) -> Result<(), String> {
    let bytes = serde_json::to_vec(value).map_err(|e| format!("serde json encode: {}", e))?;
    set_kv(key, &bytes, ttl_secs).await
}

pub async fn get_json<T: serde::de::DeserializeOwned>(key: &str) -> Result<Option<T>, String> {
    if let Some(bytes) = get_kv(key).await? {
        let v =
            serde_json::from_slice::<T>(&bytes).map_err(|e| format!("serde json decode: {}", e))?;
        Ok(Some(v))
    } else {
        Ok(None)
    }
}

pub async fn del_key(key: &str) -> Result<(), String> {
    let mut conn = get_connection().await?;
    with_deadline(async move {
        let _del_count: i32 = redis::cmd("DEL")
            .arg(key)
            .query_async(&mut conn)
            .await
            .map_err(|e| format!("redis del failed: {}", e))?;
        Ok(())
    })
    .await
}

/// Deletes every key matching `pattern` using SCAN (non-blocking, cursor-based)
/// instead of KEYS, which stalls the entire Redis server on large keyspaces.
/// Deletes happen in batches while scanning to bound memory usage.
pub async fn del_pattern(pattern: &str) -> Result<(), String> {
    const SCAN_COUNT: u64 = 200;
    const BATCH_SIZE: usize = 200;
    const MAX_KEYS: usize = 50_000; // safety cap against pathological patterns

    let mut conn = get_connection().await?;
    let pattern = pattern.to_string();
    with_deadline(async move {
        let mut total_deleted: usize = 0;
        let mut batch: Vec<String> = Vec::with_capacity(BATCH_SIZE);
        let mut cursor: u64 = 0;

        loop {
            let (next_cursor, page): (u64, Vec<String>) = redis::cmd("SCAN")
                .arg(cursor)
                .arg("MATCH")
                .arg(&pattern)
                .arg("COUNT")
                .arg(SCAN_COUNT)
                .query_async(&mut conn)
                .await
                .map_err(|e| format!("redis scan failed: {}", e))?;

            cursor = next_cursor;
            batch.extend(page);

            if batch.len() >= BATCH_SIZE {
                let deleted: i32 = redis::cmd("DEL")
                    .arg(&batch)
                    .query_async(&mut conn)
                    .await
                    .map_err(|e| format!("redis del failed: {}", e))?;
                total_deleted += deleted as usize;
                batch.clear();

                if total_deleted > MAX_KEYS {
                    warn!("del_pattern aborted: exceeded {} keys", MAX_KEYS);
                    break;
                }
            }

            if cursor == 0 {
                break;
            }
        }

        if !batch.is_empty() {
            let _: i32 = redis::cmd("DEL")
                .arg(&batch)
                .query_async(&mut conn)
                .await
                .map_err(|e| format!("redis del failed: {}", e))?;
        }
        Ok(())
    })
    .await
}

pub async fn set_raw(key: &str, value: &str, ttl_secs: Option<usize>) -> Result<(), String> {
    let mut conn = get_connection().await?;
    let value = value.to_string();
    with_deadline(async move {
        if let Some(ttl) = ttl_secs {
            let _: () = redis::cmd("SET")
                .arg(key)
                .arg(value)
                .arg("EX")
                .arg(ttl)
                .query_async(&mut conn)
                .await
                .map_err(|e| format!("redis set raw failed: {}", e))?;
        } else {
            let _: () = conn
                .set(key, value)
                .await
                .map_err(|e| format!("redis set raw failed: {}", e))?;
        }
        Ok(())
    })
    .await
}

pub async fn get_raw(key: &str) -> Result<Option<String>, String> {
    let mut conn = get_connection().await?;
    with_deadline(async move {
        let res: Option<String> = conn
            .get(key)
            .await
            .map_err(|e| format!("redis get raw failed: {}", e))?;
        Ok(res)
    })
    .await
}

pub async fn incr_by(key: &str, delta: i64) -> Result<i64, String> {
    let mut conn = get_connection().await?;
    with_deadline(async move {
        let res: i64 = conn
            .incr(key, delta)
            .await
            .map_err(|e| format!("redis incr failed: {}", e))?;
        Ok(res)
    })
    .await
}

pub async fn incr_by_float_str(key: &str, delta_str: &str) -> Result<f64, String> {
    let mut conn = get_connection().await?;
    let key = key.to_string();
    let delta_str = delta_str.to_string();
    with_deadline(async move {
        let res: f64 = redis::cmd("INCRBYFLOAT")
            .arg(key)
            .arg(delta_str)
            .query_async(&mut conn)
            .await
            .map_err(|e| format!("redis incrbyfloat failed: {}", e))?;
        Ok(res)
    })
    .await
}

pub async fn expire(key: &str, ttl_secs: usize) -> Result<(), String> {
    let mut conn = get_connection().await?;
    with_deadline(async move {
        let _: i32 = redis::cmd("EXPIRE")
            .arg(key)
            .arg(ttl_secs)
            .query_async(&mut conn)
            .await
            .map_err(|e| format!("redis expire failed: {}", e))?;
        Ok(())
    })
    .await
}

pub async fn enqueue_json<T: serde::Serialize>(list_key: &str, value: &T) -> Result<(), String> {
    let payload = serde_json::to_string(value).map_err(|e| format!("serde json encode: {}", e))?;
    let list_key = list_key.to_string();
    let mut conn = get_connection().await?;
    with_deadline(async move {
        let _: i64 = redis::cmd("RPUSH")
            .arg(list_key)
            .arg(payload)
            .query_async(&mut conn)
            .await
            .map_err(|e| format!("redis rpush failed: {}", e))?;
        Ok(())
    })
    .await
}

pub async fn set_session<T: serde::Serialize>(
    session_id: &str,
    value: &T,
    ttl_secs: usize,
) -> Result<(), String> {
    let key = format!("session:{}", session_id);
    set_json(&key, value, ttl_secs).await
}

pub async fn get_session<T: serde::de::DeserializeOwned>(
    session_id: &str,
) -> Result<Option<T>, String> {
    let key = format!("session:{}", session_id);
    get_json(&key).await
}

pub async fn del_session(session_id: &str) -> Result<(), String> {
    let key = format!("session:{}", session_id);
    del_key(&key).await
}
