use std::collections::HashMap;
use std::sync::{LazyLock, Mutex};
use std::time::{Duration, Instant};

/// In-memory token revocation store.
///
/// Maps a JTI (JWT ID) to the instant it was revoked.
/// Entries are automatically evicted after MAX_TOKEN_AGE (7 days — max refresh token lifetime).
///
/// ⚠️  LIMITATION: This store is local to the process. In multi-instance (horizontal scale)
/// deployments, revocations will NOT be shared across instances.
/// For distributed deployments, replace with Redis:
///   Key → `revoked_jti:{jti}`   TTL → 7 days
static REVOCATION_STORE: LazyLock<Mutex<HashMap<String, Instant>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

/// Maximum age of revocation entries (matches max refresh token lifetime).
const MAX_TOKEN_AGE: Duration = Duration::from_secs(7 * 24 * 60 * 60);

/// Mark a token JTI as revoked. Subsequent calls to [`is_revoked`] with the same JTI
/// will return `true` until the entry expires.
pub fn revoke_token(jti: &str) {
    let mut store = REVOCATION_STORE.lock().unwrap();
    // Evict stale entries while we hold the lock
    store.retain(|_, revoked_at| revoked_at.elapsed() < MAX_TOKEN_AGE);
    store.insert(jti.to_string(), Instant::now());
}

/// Returns `true` if the given JTI has been revoked and the revocation has not expired.
pub fn is_revoked(jti: &str) -> bool {
    let store = REVOCATION_STORE.lock().unwrap();
    store
        .get(jti)
        .map(|revoked_at| revoked_at.elapsed() < MAX_TOKEN_AGE)
        .unwrap_or(false)
}
