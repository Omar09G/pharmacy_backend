use axum::{
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
};

use crate::api_utils::api_error::ApiError;
use crate::config::config_jwt::dto_jwt::Claims;

/// Extractor that retrieves the authenticated user's JWT claims from request extensions.
///
/// The claims are inserted by the `auth_middleware` after token validation.
/// Use this in any handler that needs the current user's identity.
pub struct AuthClaims(pub Claims);

impl<S: Send + Sync> FromRequestParts<S> for AuthClaims {
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<Claims>()
            .cloned()
            .map(AuthClaims)
            .ok_or(StatusCode::UNAUTHORIZED)
    }
}

/// Check that the user has the required permission in their JWT claims.
/// Returns `Err(ApiError::Forbidden)` if the permission is missing.
pub fn check_permission(claims: &Claims, permission: &str) -> Result<(), ApiError> {
    if claims.permissions.iter().any(|p| p == permission) {
        Ok(())
    } else {
        Err(ApiError::Forbidden(format!(
            "Missing required permission: {}",
            permission
        )))
    }
}
