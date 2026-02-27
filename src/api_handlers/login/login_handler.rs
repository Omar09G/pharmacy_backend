use crate::api_utils::api_const::{JWT_TYPE_ACCESS, JWT_TYPE_REFRESH};
use crate::config::config_jwt::validate_jwt::generate_jwt;
use crate::{
    api_utils::api_response::ApiResponse, config::config_database::config_db_context::AppContext,
};

use crate::api_handlers::login::login_dto::{LoginRequest, LoginResponseDTO};
use crate::api_utils::api_error::ApiError;

use crate::config::config_pass::config_password::verify_password;
use axum::{Json, extract::State};
use log::info;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use validator::Validate;

pub async fn get_login(
    State(app_ctx): State<AppContext>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<ApiResponse<LoginResponseDTO>>, ApiError> {
    info!("Received login request for username: {}", payload.username);

    payload.validate().map_err(ApiError::Validation)?;

    let user = schemas::user::Entity::find()
        .filter(schemas::user::Column::Username.eq(payload.username.clone()))
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?
        .ok_or_else(|| ApiError::Unauthorized)?;

    let is_valid_password =
        verify_password(&payload.password, &user.password.unwrap()).map_err(|e| {
            ApiError::Unexpected(Box::new(std::io::Error::new(std::io::ErrorKind::Other, e)))
        })?;

    if !is_valid_password {
        return Err(ApiError::Unauthorized);
    }

    let access_token = generate_jwt(
        user.username.clone(),
        user.role.clone().unwrap_or_default(),
        JWT_TYPE_ACCESS.to_string(),
    )
    .await;

    let refresh_token = generate_jwt(
        user.username.clone(),
        user.role.clone().unwrap_or_default(),
        JWT_TYPE_REFRESH.to_string(),
    )
    .await;

    let response = ApiResponse {
        data: LoginResponseDTO::new(
            user.username,
            user.role.unwrap_or_default(),
            access_token.unwrap(),
            refresh_token.unwrap(),
        ),
        message: "Login successful".to_string(),
        status: "success".to_string(),
        code_error: 200,
        timestamp: chrono::Utc::now().to_rfc3339(),
    };

    info!("Login successful for user: {}", response.data.username);

    Ok(Json(response))
}
