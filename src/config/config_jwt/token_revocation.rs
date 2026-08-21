use crate::config::config_redis;
use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};
use std::time::{Duration, Instant};

const MAX_TOKEN_AGE: Duration = Duration::from_secs(7 * 24 * 60 * 60);

// In-memory fallback so revocations take effect immediately for the local process.
static REVOCATION_STORE: LazyLock<Mutex<HashMap<String, Instant>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

/// Locks the revocation store, recovering gracefully if the mutex was
/// poisoned by a panic in another thread (the map itself stays consistent).
fn lock_store() -> std::sync::MutexGuard<'static, HashMap<String, Instant>> {
    REVOCATION_STORE
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

/// Mark a token JTI as revoked. We insert into the local store immediately
/// and attempt to persist to Redis asynchronously so other instances see it.
pub fn revoke_token(jti: &str) {
    {
        let mut store = lock_store();
        store.retain(|_, revoked_at| revoked_at.elapsed() < MAX_TOKEN_AGE);
        store.insert(jti.to_string(), Instant::now());
    }

    let jti = jti.to_string();
    // Fire-and-forget: store in redis with TTL
    let _ = tokio::spawn(async move {
        let _ = config_redis::set_kv(
            &format!("revoked_jti:{}", jti),
            b"1",
            MAX_TOKEN_AGE.as_secs() as usize,
        )
        .await;
    });
}

/// Returns `true` if the given JTI has been revoked. Tries Redis first (authoritative),
/// then falls back to the in-memory store.
pub async fn is_revoked(jti: &str) -> bool {
    let key = format!("revoked_jti:{}", jti);
    match config_redis::get_kv(&key).await {
        Ok(Some(_)) => return true,
        Ok(None) => {}
        Err(_) => {}
    }

    let store = lock_store();
    store
        .get(jti)
        .map(|revoked_at| revoked_at.elapsed() < MAX_TOKEN_AGE)
        .unwrap_or(false)
}
