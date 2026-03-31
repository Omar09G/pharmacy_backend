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
            min = 4,
            max = 25,
            message = "Password must be at least 4 characters long"
        ),
        custom(function = "crate::api_utils::api_utils_fun::validate_special_chars")
    )]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct LoginResponseDTO {
    pub id: i64,
    pub full_name: String,
    pub username: String,
    pub role: String,
    pub token: String,
}

impl LoginResponseDTO {
    pub fn new(id: i64, full_name: String, username: String, role: String, token: String) -> Self {
        Self {
            id,
            full_name,
            username,
            role,
            token,
        }
    }
}
