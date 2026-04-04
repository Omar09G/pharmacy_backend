use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use log::info;
use std::env;

use crate::{
    api_utils::api_const::{JWT_TYPE_ACCESS, JWT_TYPE_REFRESH},
    config::config_jwt::dto_jwt::Claims,
};

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
) -> Result<String, String> {
    get_jwt_token_with_role(username, role, jwt_type, id_user, name_user).await
}

pub async fn get_jwt_token_with_role(
    username: String,
    role: String,
    jwt_type: String,
    id_user: i64,
    name_user: String,
) -> Result<String, String> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: username.clone(),
        exp: expiration as usize,
        iat: Utc::now().timestamp() as usize,
        company: "Pharmacy".to_string(),
        role: role.clone(),
        user_name: username.clone(),
        id: id_user,
        name: name_user,
    };

    info!("Generating {} JWT for user: {}", jwt_type, claims.sub);

    // Prefer RSA (RS256) if RSA env keys are present, otherwise fallback to HMAC secret
    let token = if let Ok(private_pem) = get_jwt_private_pem() {
        let header = Header::new(Algorithm::RS256);
        encode(
            &header,
            &claims,
            &EncodingKey::from_rsa_pem(private_pem.as_bytes()).map_err(|e| e.to_string())?,
        )
        .map_err(|e| e.to_string())?
    } else {
        let jwt_secret = if jwt_type == JWT_TYPE_ACCESS {
            get_jwt_secret()?
        } else if jwt_type == JWT_TYPE_REFRESH {
            get_jwt_secret_refresh()?
        } else {
            return Err("Invalid token type".to_string());
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

//Validar Token
pub async fn validate_token(token: &str) -> Result<Claims, String> {
    // Prefer RSA public key if provided, otherwise use HMAC secret
    let decoded = if let Ok(public_pem) = get_jwt_public_pem() {
        decode::<Claims>(
            &token,
            &DecodingKey::from_rsa_pem(public_pem.as_bytes()).map_err(|e| e.to_string())?,
            &Validation::new(Algorithm::RS256),
        )
        .map_err(|e| e.to_string())?
    } else {
        let jwt_secret = get_jwt_secret()?;
        decode::<Claims>(
            &token,
            &DecodingKey::from_secret(jwt_secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|e| e.to_string())?
    };

    info!("Validated JWT for user: {}", decoded.claims.sub);

    Ok(decoded.claims)
}

//Validar Token
pub async fn validate_token_refresh(token: &str) -> Result<Claims, String> {
    let decoded = if let Ok(public_pem) = get_jwt_public_pem() {
        decode::<Claims>(
            &token,
            &DecodingKey::from_rsa_pem(public_pem.as_bytes()).map_err(|e| e.to_string())?,
            &Validation::new(Algorithm::RS256),
        )
        .map_err(|e| e.to_string())?
    } else {
        let jwt_secret = get_jwt_secret_refresh()?;
        decode::<Claims>(
            &token,
            &DecodingKey::from_secret(jwt_secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|e| e.to_string())?
    };

    info!("Validated JWT Refresh for user: {}", decoded.claims.sub);

    Ok(decoded.claims)
}
