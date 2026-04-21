/// Integration tests for JWT generation and validation (HMAC mode).
///
/// These tests exercise the public API of `config_jwt::validate_jwt` and
/// `config_jwt::token_revocation` using the HMAC fallback path so no RSA key
/// material is required at test time.
///
/// **IMPORTANT – test isolation**: several tests mutate env vars that the JWT
/// helpers read on every call (`API_JWT_SECRET`, `API_JWT_SECRET_REFRESH`).
/// Because `#[tokio::test]` runs tests concurrently we serialise *all* tests
/// in this file through `JWT_ENV_LOCK` so they see a consistent env-var state.
use std::sync::OnceLock;

use pharmacy_backend::api_utils::api_const::{JWT_TYPE_ACCESS, JWT_TYPE_REFRESH};
use pharmacy_backend::config::config_jwt::dto_jwt::Claims;
use pharmacy_backend::config::config_jwt::token_revocation::{is_revoked, revoke_token};
use pharmacy_backend::config::config_jwt::validate_jwt::{
    generate_jwt, validate_token, validate_token_refresh,
};
use tokio::sync::Mutex;

/// One global async mutex — held for the duration of every test that touches
/// env vars so that concurrent test threads don't interfere.
static JWT_ENV_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

fn jwt_env_lock() -> &'static Mutex<()> {
    JWT_ENV_LOCK.get_or_init(|| Mutex::new(()))
}

/// Set both HMAC secrets so `init_jwt_keys_if_needed` falls through to the
/// HMAC path (no RSA vars must be present).
fn setup_hmac_env() {
    // Safety: caller holds `JWT_ENV_LOCK`; no other test can race on these vars.
    unsafe {
        std::env::remove_var("API_JWT_PRIVATE_PEM");
        std::env::remove_var("API_JWT_PRIVATE_PEM_PATH");
        std::env::remove_var("API_JWT_PUBLIC_PEM");
        std::env::remove_var("API_JWT_PUBLIC_PEM_PATH");
        std::env::set_var("API_JWT_SECRET", "test-access-secret-32-chars-long!!");
        std::env::set_var(
            "API_JWT_SECRET_REFRESH",
            "test-refresh-secret-32-chars-long!",
        );
    }
}

// ── generate + validate (access) ─────────────────────────────────────────────

#[tokio::test]
async fn generate_and_validate_access_token_succeeds() {
    let _guard = jwt_env_lock().lock().await;
    setup_hmac_env();

    let token: String = generate_jwt(
        "alice".to_string(),
        "admin".to_string(),
        JWT_TYPE_ACCESS.to_string(),
        1,
        "Alice Smith".to_string(),
        vec!["read".to_string()],
    )
    .await
    .expect("token generation must succeed");

    let claims: Claims = validate_token(&token)
        .await
        .expect("access token must validate");

    assert_eq!(claims.sub, "alice");
    assert_eq!(claims.role, "admin");
    assert_eq!(claims.id, 1);
    assert_eq!(claims.name, "Alice Smith");
    assert_eq!(claims.token_type, JWT_TYPE_ACCESS);
    assert!(claims.permissions.contains(&"read".to_string()));
}

#[tokio::test]
async fn access_token_rejects_wrong_secret() {
    let _guard = jwt_env_lock().lock().await;
    setup_hmac_env();

    let token: String = generate_jwt(
        "bob".to_string(),
        "user".to_string(),
        JWT_TYPE_ACCESS.to_string(),
        2,
        "Bob".to_string(),
        vec![],
    )
    .await
    .expect("generation must succeed");

    // Swap in a different secret so validation fails
    unsafe {
        std::env::set_var("API_JWT_SECRET", "completely-different-secret-abcdef!!");
    }

    let result: Result<Claims, String> = validate_token(&token).await;
    assert!(
        result.is_err(),
        "token signed with a different secret must fail"
    );

    // Restore for subsequent tests
    setup_hmac_env();
}

// ── generate + validate (refresh) ────────────────────────────────────────────

#[tokio::test]
async fn generate_and_validate_refresh_token_succeeds() {
    let _guard = jwt_env_lock().lock().await;
    setup_hmac_env();

    let token: String = generate_jwt(
        "carol".to_string(),
        "pharmacist".to_string(),
        JWT_TYPE_REFRESH.to_string(),
        3,
        "Carol Jones".to_string(),
        vec!["write".to_string()],
    )
    .await
    .expect("refresh token generation must succeed");

    let claims: Claims = validate_token_refresh(&token)
        .await
        .expect("refresh token must validate");

    assert_eq!(claims.sub, "carol");
    assert_eq!(claims.token_type, JWT_TYPE_REFRESH);
}

// ── cross-type usage must fail ────────────────────────────────────────────────

#[tokio::test]
async fn access_token_rejected_by_validate_token_refresh() {
    let _guard = jwt_env_lock().lock().await;
    setup_hmac_env();

    let token: String = generate_jwt(
        "dave".to_string(),
        "user".to_string(),
        JWT_TYPE_ACCESS.to_string(),
        4,
        "Dave".to_string(),
        vec![],
    )
    .await
    .unwrap();

    // validate_token_refresh must reject a token whose type is "access"
    let result: Result<Claims, String> = validate_token_refresh(&token).await;
    assert!(
        result.is_err(),
        "validate_token_refresh must reject an access token"
    );
}

#[tokio::test]
async fn refresh_token_rejected_by_validate_token() {
    let _guard = jwt_env_lock().lock().await;
    setup_hmac_env();

    let token: String = generate_jwt(
        "eve".to_string(),
        "user".to_string(),
        JWT_TYPE_REFRESH.to_string(),
        5,
        "Eve".to_string(),
        vec![],
    )
    .await
    .unwrap();

    // validate_token (access) must reject a refresh token
    let result: Result<Claims, String> = validate_token(&token).await;
    assert!(
        result.is_err(),
        "validate_token must reject a refresh token"
    );
}

// ── invalid token type ────────────────────────────────────────────────────────

#[tokio::test]
async fn generate_jwt_fails_for_unknown_token_type() {
    let _guard = jwt_env_lock().lock().await;
    setup_hmac_env();
    let result: Result<String, String> = generate_jwt(
        "frank".to_string(),
        "user".to_string(),
        "unknown_type".to_string(),
        6,
        "Frank".to_string(),
        vec![],
    )
    .await;
    assert!(result.is_err(), "unknown token type must return an error");
}

// ── revocation integration ────────────────────────────────────────────────────

#[tokio::test]
async fn revoked_access_token_is_rejected_by_validate_token() {
    let _guard = jwt_env_lock().lock().await;
    setup_hmac_env();

    let token: String = generate_jwt(
        "grace".to_string(),
        "admin".to_string(),
        JWT_TYPE_ACCESS.to_string(),
        7,
        "Grace".to_string(),
        vec![],
    )
    .await
    .unwrap();

    // Decode to get jti without going through full validate_token
    use jsonwebtoken::{DecodingKey, Validation, decode};

    let secret =
        std::env::var("API_JWT_SECRET").expect("API_JWT_SECRET must be set after setup_hmac_env");
    let data = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .expect("decode must succeed");

    let jti = data.claims.jti.expect("jti must be present");
    revoke_token(&jti);
    assert!(is_revoked(&jti));

    // validate_token should now return an error
    let result: Result<Claims, String> = validate_token(&token).await;
    assert!(
        result.is_err(),
        "validate_token must reject a revoked access token"
    );
}

#[tokio::test]
async fn revoked_refresh_token_is_rejected_by_validate_token_refresh() {
    let _guard = jwt_env_lock().lock().await;
    setup_hmac_env();

    let token: String = generate_jwt(
        "henry".to_string(),
        "user".to_string(),
        JWT_TYPE_REFRESH.to_string(),
        8,
        "Henry".to_string(),
        vec![],
    )
    .await
    .unwrap();

    use jsonwebtoken::{DecodingKey, Validation, decode};

    let secret = std::env::var("API_JWT_SECRET_REFRESH")
        .expect("API_JWT_SECRET_REFRESH must be set after setup_hmac_env");
    let data = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .expect("decode must succeed");

    let jti = data.claims.jti.expect("jti must be present");
    revoke_token(&jti);

    let result: Result<Claims, String> = validate_token_refresh(&token).await;
    assert!(
        result.is_err(),
        "validate_token_refresh must reject a revoked refresh token"
    );
}

// ── claims payload correctness ────────────────────────────────────────────────

#[tokio::test]
async fn access_token_claims_include_company_pharmacy() {
    let _guard = jwt_env_lock().lock().await;
    setup_hmac_env();

    let token: String = generate_jwt(
        "ivy".to_string(),
        "cashier".to_string(),
        JWT_TYPE_ACCESS.to_string(),
        9,
        "Ivy".to_string(),
        vec!["pos".to_string(), "reports".to_string()],
    )
    .await
    .unwrap();

    let claims: Claims = validate_token(&token).await.unwrap();
    assert_eq!(claims.company, "Pharmacy");
    assert_eq!(claims.permissions.len(), 2);
    assert!(
        claims.jti.is_some(),
        "JTI must be present for revocation support"
    );
}
