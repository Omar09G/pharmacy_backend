use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(
        length(
            min = 3,
            max = 15,
            message = "Username must be at least 3 characters long"
        ),
        custom(function = "crate::api_utils::api_utils_fun::validate_special_chars")
    )]
    pub username: String,
    #[validate(
        length(
            min = 6,
            max = 50,
            message = "Password must be at least 6 characters long"
        ),
        custom(function = "crate::api_utils::api_utils_fun::validate_special_chars")
    )]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct LoginResponseDTO {
    pub username: String,
    pub role: String,
    pub token: String,
    pub token_refresh: String,
}

impl LoginResponseDTO {
    pub fn new(username: String, role: String, token: String, token_refresh: String) -> Self {
        Self {
            username,
            role,
            token,
            token_refresh,
        }
    }
}
