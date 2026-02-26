use serde::{Deserialize, Serialize};
// Estructura para Claims del JWT
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Claims {
    pub sub: String, // Subject (username)
    pub exp: usize,  // Expiration
    pub iat: usize,  // Issued At
    pub user_name: String,
    pub role: String,
    pub company: String,
}
