use crate::api_module::login::dto::login_dto::{
    LoginRequest, LoginResponseDTO, RefreshRequest, RefreshResponse,
};
use crate::api_utils::api_const::{JWT_TYPE_ACCESS, JWT_TYPE_REFRESH};
use crate::config::config_jwt::validate_jwt::{generate_jwt, validate_token_refresh};
use crate::{
    api_utils::api_response::ApiResponse, config::config_database::config_db_context::AppContext,
};

use crate::api_utils::api_error::ApiError;

use crate::config::config_pass::config_password::verify_password;
use axum::{Json, extract::State};
use log::{error, info};
use sea_orm::{ColumnTrait, EntityTrait, ModelTrait, QueryFilter};
use validator::Validate;

pub async fn get_login(
    State(app_ctx): State<AppContext>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<ApiResponse<LoginResponseDTO>>, ApiError> {
    info!("Received login request for username: {}", payload.username);

    payload.validate().map_err(ApiError::Validation)?;

    // Verify user exists, is active, and not soft-deleted
    let user = schemas::users::Entity::find()
        .filter(schemas::users::Column::Username.eq(payload.username.clone()))
        .filter(schemas::users::Column::Status.eq("ACTIVE"))
        .filter(schemas::users::Column::DeletedAt.is_null())
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?
        .ok_or(ApiError::Unauthorized)?;

    let pwd = payload.password.clone();
    let user_hash = user.password_hash.clone();
    let is_valid_password = tokio::task::spawn_blocking(move || verify_password(&pwd, &user_hash))
        .await
        .map_err(|e| {
            error!("Join error verifying password: {}", e);
            ApiError::Unauthorized
        })?
        .map_err(|e| {
            error!("Error verifying password: {}", e);
            ApiError::Unauthorized
        })?;

    if !is_valid_password {
        return Err(ApiError::Unauthorized);
    }

    let roles = user
        .find_related(schemas::roles::Entity)
        .all(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    let role = roles.into_iter().next().ok_or(ApiError::Unauthorized)?;

    // Fetch permissions for this role
    let permission_models = role
        .find_related(schemas::permissions::Entity)
        .all(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;
    let permissions: Vec<String> = permission_models.into_iter().map(|p| p.name).collect();

    let full_name = user.full_name.clone().unwrap_or_default();

    let access_token = generate_jwt(
        user.username.clone(),
        role.name.clone(),
        JWT_TYPE_ACCESS.to_string(),
        user.id,
        full_name.clone(),
        permissions.clone(),
    )
    .await
    .map_err(|e| ApiError::Unexpected(Box::new(std::io::Error::other(e))))?;

    let refresh_token = generate_jwt(
        user.username.clone(),
        role.name.clone(),
        JWT_TYPE_REFRESH.to_string(),
        user.id,
        full_name.clone(),
        permissions,
    )
    .await
    .map_err(|e| ApiError::Unexpected(Box::new(std::io::Error::other(e))))?;

    let response = ApiResponse::success(
        LoginResponseDTO::new(
            user.id,
            full_name,
            user.username,
            role.name.clone(),
            access_token,
            refresh_token,
        ),
        "Login successful".to_string(),
        1,
    );

    Ok(Json(response))
}

/// Endpoint: POST /v1/api/auth/refresh
/// Accepts a valid refresh token and returns a new access token + refresh token pair.
pub async fn refresh_token(
    Json(payload): Json<RefreshRequest>,
) -> Result<Json<ApiResponse<RefreshResponse>>, ApiError> {
    payload.validate().map_err(ApiError::Validation)?;

    let claims = validate_token_refresh(&payload.refresh_token)
        .await
        .map_err(|_| ApiError::Unauthorized)?;

    let new_access = generate_jwt(
        claims.user_name.clone(),
        claims.role.clone(),
        JWT_TYPE_ACCESS.to_string(),
        claims.id,
        claims.name.clone(),
        claims.permissions.clone(),
    )
    .await
    .map_err(|e| ApiError::Unexpected(Box::new(std::io::Error::other(e))))?;

    let new_refresh = generate_jwt(
        claims.user_name.clone(),
        claims.role.clone(),
        JWT_TYPE_REFRESH.to_string(),
        claims.id,
        claims.name.clone(),
        claims.permissions,
    )
    .await
    .map_err(|e| ApiError::Unexpected(Box::new(std::io::Error::other(e))))?;

    let data = RefreshResponse {
        token: new_access,
        refresh_token: new_refresh,
    };

    Ok(Json(ApiResponse::success(
        data,
        "Token refreshed successfully".to_string(),
        1,
    )))
}

pub async fn get_profile(
    crate::api_utils::extractors::AuthClaims(claims): crate::api_utils::extractors::AuthClaims,
) -> Result<Json<ApiResponse<LoginResponseDTO>>, ApiError> {
    let token_validate = LoginResponseDTO::new(
        claims.id,
        claims.name,
        claims.user_name,
        claims.role,
        String::new(), // Profile doesn't re-expose the token
        String::new(), // Profile doesn't return a new refresh token
    );

    Ok(Json(ApiResponse::success(
        token_validate,
        "Token Valid OK".to_string(),
        1,
    )))
}
