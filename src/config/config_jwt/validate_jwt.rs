use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use log::info;
use std::env;
use std::sync::{Arc, LazyLock, Mutex};

use crate::{
    api_utils::api_const::{JWT_TYPE_ACCESS, JWT_TYPE_REFRESH},
    config::config_jwt::dto_jwt::Claims,
};

static JWT_ENCODING_RS: LazyLock<Mutex<Option<Arc<EncodingKey>>>> =
    LazyLock::new(|| Mutex::new(None));
static JWT_DECODING_RS: LazyLock<Mutex<Option<Arc<DecodingKey>>>> =
    LazyLock::new(|| Mutex::new(None));
static JWT_ALGO: LazyLock<Mutex<Option<Algorithm>>> = LazyLock::new(|| Mutex::new(None));

pub fn init_jwt_keys_if_needed() -> Result<(), String> {
    // Fast path
    if JWT_ALGO.lock().unwrap().is_some() {
        return Ok(());
    }
    info!("Initializing JWT keys if needed...");
    // Try env var containing the PEM directly
    if let Ok(private_pem) = env::var("API_JWT_PRIVATE_PEM") {
        let public_pem = env::var("API_JWT_PUBLIC_PEM")
            .map_err(|_| "API_JWT_PUBLIC_PEM required for RSA".to_string())?;
        let encoding =
            EncodingKey::from_rsa_pem(private_pem.as_bytes()).map_err(|e| e.to_string())?;
        let decoding =
            DecodingKey::from_rsa_pem(public_pem.as_bytes()).map_err(|e| e.to_string())?;
        *JWT_ENCODING_RS.lock().unwrap() = Some(Arc::new(encoding));
        *JWT_DECODING_RS.lock().unwrap() = Some(Arc::new(decoding));
        *JWT_ALGO.lock().unwrap() = Some(Algorithm::RS256);
        return Ok(());
    }

    // Try env var paths
    if let Ok(private_path) = env::var("API_JWT_PRIVATE_PEM_PATH") {
        let private_pem = std::fs::read_to_string(&private_path)
            .map_err(|e| format!("failed to read private pem: {}", e))?;
        let public_path = env::var("API_JWT_PUBLIC_PEM_PATH")
            .map_err(|_| "API_JWT_PUBLIC_PEM_PATH required for RSA".to_string())?;
        let public_pem = std::fs::read_to_string(&public_path)
            .map_err(|e| format!("failed to read public pem: {}", e))?;
        let encoding =
            EncodingKey::from_rsa_pem(private_pem.as_bytes()).map_err(|e| e.to_string())?;
        let decoding =
            DecodingKey::from_rsa_pem(public_pem.as_bytes()).map_err(|e| e.to_string())?;
        *JWT_ENCODING_RS.lock().unwrap() = Some(Arc::new(encoding));
        *JWT_DECODING_RS.lock().unwrap() = Some(Arc::new(decoding));
        *JWT_ALGO.lock().unwrap() = Some(Algorithm::RS256);
        return Ok(());
    }
    info!("No RSA keys found for JWT; falling back to HMAC secrets");
    // No RSA keys found: mark uninitialized and let callers fallback to HMAC secrets per-token-type
    Ok(())
}

pub fn get_jwt_secret() -> Result<String, String> {
    env::var("API_JWT_SECRET").map_err(|_| "API_JWT_SECRET must be set".to_string())
}

pub fn get_jwt_secret_refresh() -> Result<String, String> {
    env::var("API_JWT_SECRET_REFRESH").map_err(|_| "API_JWT_SECRET_REFRESH must be set".to_string())
}

// RSA keys (PEM) for RS256 signing
pub fn get_jwt_private_pem() -> Result<String, String> {
    // Try direct env var first
    if let Ok(pem) = env::var("API_JWT_PRIVATE_PEM") {
        return Ok(pem);
    }

    // Fallback to file path
    if let Ok(path) = env::var("API_JWT_PRIVATE_PEM_PATH") {
        return std::fs::read_to_string(path)
            .map_err(|e| format!("failed to read private pem: {}", e));
    }

    Err("API_JWT_PRIVATE_PEM or API_JWT_PRIVATE_PEM_PATH must be set".to_string())
}

pub fn get_jwt_public_pem() -> Result<String, String> {
    if let Ok(pem) = env::var("API_JWT_PUBLIC_PEM") {
        return Ok(pem);
    }

    if let Ok(path) = env::var("API_JWT_PUBLIC_PEM_PATH") {
        return std::fs::read_to_string(path)
            .map_err(|e| format!("failed to read public pem: {}", e));
    }

    Err("API_JWT_PUBLIC_PEM or API_JWT_PUBLIC_PEM_PATH must be set".to_string())
}

pub async fn generate_jwt(
    username: String,
    role: String,
    jwt_type: String,
    id_user: i64,
    name_user: String,
    permissions: Vec<String>,
) -> Result<String, String> {
    get_jwt_token_with_role(username, role, jwt_type, id_user, name_user, permissions).await
}

pub async fn get_jwt_token_with_role(
    username: String,
    role: String,
    jwt_type: String,
    id_user: i64,
    name_user: String,
    permissions: Vec<String>,
) -> Result<String, String> {
    // Access tokens: 1 day; Refresh tokens: 7 days
    let duration = if jwt_type == JWT_TYPE_ACCESS {
        Duration::days(1)
    } else if jwt_type == JWT_TYPE_REFRESH {
        Duration::days(7)
    } else {
        return Err("Invalid token type".to_string());
    };

    let expiration = Utc::now()
        .checked_add_signed(duration)
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: username.clone(),
        exp: expiration as usize,
        iat: Utc::now().timestamp() as usize,
        company: "Pharmacy".to_string(),
        role: role.clone(),
        permissions,
        user_name: username.clone(),
        id: id_user,
        name: name_user,
        token_type: jwt_type.clone(),
    };

    info!("Generating {} JWT for user: {}", jwt_type, claims.sub);

    // Prefer RSA (RS256) if RSA env keys are present; otherwise fallback to HMAC secret
    init_jwt_keys_if_needed()?;
    let token = if let Some(enc_arc) = JWT_ENCODING_RS.lock().unwrap().as_ref().cloned() {
        let header = Header::new(Algorithm::RS256);
        encode(&header, &claims, &*enc_arc).map_err(|e| e.to_string())?
    } else {
        let jwt_secret = if jwt_type == JWT_TYPE_ACCESS {
            get_jwt_secret()?
        } else {
            get_jwt_secret_refresh()?
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(jwt_secret.as_bytes()),
        )
        .map_err(|e| e.to_string())?
    };

    Ok(token)
}

//Validar Token — solo acepta access tokens
pub async fn validate_token(token: &str) -> Result<Claims, String> {
    init_jwt_keys_if_needed()?;
    let decoded = if let Some(dec_arc) = JWT_DECODING_RS.lock().unwrap().as_ref().cloned() {
        decode::<Claims>(token, &*dec_arc, &Validation::new(Algorithm::RS256))
            .map_err(|e| e.to_string())?
    } else {
        let jwt_secret = get_jwt_secret()?;
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(jwt_secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|e| e.to_string())?
    };

    // Reject refresh tokens used as access tokens
    if decoded.claims.token_type != JWT_TYPE_ACCESS {
        return Err("Invalid token type: expected access token".to_string());
    }

    Ok(decoded.claims)
}

//Validar Token Refresh — solo acepta refresh tokens
pub async fn validate_token_refresh(token: &str) -> Result<Claims, String> {
    init_jwt_keys_if_needed()?;
    let decoded = if let Some(dec_arc) = JWT_DECODING_RS.lock().unwrap().as_ref().cloned() {
        decode::<Claims>(token, &*dec_arc, &Validation::new(Algorithm::RS256))
            .map_err(|e| e.to_string())?
    } else {
        let jwt_secret = get_jwt_secret_refresh()?;
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(jwt_secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|e| e.to_string())?
    };

    // Reject access tokens used as refresh tokens
    if decoded.claims.token_type != JWT_TYPE_REFRESH {
        return Err("Invalid token type: expected refresh token".to_string());
    }

    Ok(decoded.claims)
}
