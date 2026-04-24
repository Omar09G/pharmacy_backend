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
    /// Only populated for native clients (`X-Client-Platform: native`).
    /// Web clients receive tokens exclusively via HttpOnly cookies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    /// Only populated for native clients (`X-Client-Platform: native`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_token: Option<String>,
}

impl LoginResponseDTO {
    pub fn new(id: i64, full_name: String, username: String, role: String) -> Self {
        Self {
            id,
            full_name,
            username,
            role,
            access_token: None,
            refresh_token: None,
        }
    }

    /// Attach tokens to the response body (native clients only).
    pub fn with_tokens(mut self, access: String, refresh: String) -> Self {
        self.access_token = Some(access);
        self.refresh_token = Some(refresh);
        self
    }
}

/// Request body for `/auth/refresh` from native clients.
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct RefreshRequest {
    pub refresh_token: Option<String>,
}

/// Request body for `/auth/logout` from native clients.
#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LogoutRequest {
    pub refresh_token: Option<String>,
}
