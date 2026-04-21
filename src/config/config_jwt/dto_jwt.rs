use serde::{Deserialize, Serialize};
// Estructura para Claims del JWT
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Claims {
    pub sub: String,        // Subject (username)
    pub exp: usize,         // Expiration
    pub iat: usize,         // Issued At
    pub jti: Option<String>, // JWT ID — used for token revocation
    pub user_name: String,
    pub id: i64,
    pub name: String,
    pub role: String,
    pub permissions: Vec<String>,
    pub company: String,
    pub token_type: String, // "access" or "refresh"
}
