use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use log::{info, warn};
use std::env;

use crate::config::config_jwt::dto_jwt::Claims;

pub fn get_jwt_secret() -> Result<String, String> {
    warn!("Attempting to retrieve JWT secret from environment variable");
    env::var("API_JWT_SECRET").map_err(|_| "API_JWT_SECRET must be set".to_string())
}

pub async fn generate_jwt(username: String) -> Result<String, String> {
    get_jwt_token(username).await
}

pub async fn get_jwt_token(username: String) -> Result<String, String> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: username.clone(),
        exp: expiration as usize,
        iat: Utc::now().timestamp() as usize,
        company: "Pharmacy".to_string(),
        role: "admin".to_string(),
        user_name: username.clone(),
    };

    let jwt_secret = get_jwt_secret()?;

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    )
    .map_err(|e| e.to_string())?;

    info!("Generated JWT for user: {}", claims.sub);

    Ok(token)
}

//Validar Token
pub async fn validate_token(token: String) -> Result<Claims, String> {
    let jwt_secret = get_jwt_secret()?;

    let token = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|e| e.to_string())?;

    info!("Validated JWT for user: {}", token.claims.sub);

    Ok(token.claims)
}
