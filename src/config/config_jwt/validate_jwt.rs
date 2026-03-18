use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
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

    let jwt_secret = if jwt_type == JWT_TYPE_ACCESS {
        get_jwt_secret()?
    } else if jwt_type == JWT_TYPE_REFRESH {
        get_jwt_secret_refresh()?
    } else {
        return Err("Invalid token type".to_string());
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    )
    .map_err(|e| e.to_string())?;

    Ok(token)
}

//Validar Token
pub async fn validate_token(token: &str) -> Result<Claims, String> {
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

//Validar Token
pub async fn validate_token_refresh(token: &str) -> Result<Claims, String> {
    let jwt_secret = get_jwt_secret_refresh()?;

    let token = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|e| e.to_string())?;

    info!("Validated JWT Refresh for user: {}", token.claims.sub);

    Ok(token.claims)
}
