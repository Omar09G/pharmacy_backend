use crate::api_module::login::dto::login_dto::{LoginRequest, LoginResponseDTO};
use crate::api_utils::api_const::JWT_TYPE_ACCESS;
use crate::config::config_jwt::validate_jwt::{generate_jwt, validate_token};
use crate::{
    api_utils::api_response::ApiResponse, config::config_database::config_db_context::AppContext,
};

use crate::api_utils::api_error::ApiError;

use crate::config::config_pass::config_password::verify_password;
use axum::body::Body;
use axum::http::Request;
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

    let user = schemas::users::Entity::find()
        .filter(schemas::users::Column::Username.eq(payload.username.clone()))
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?
        .ok_or_else(|| ApiError::Unauthorized)?;

    let is_valid_password =
        verify_password(&payload.password, &user.password_hash).map_err(|e| {
            error!("Error: {}", e);

            ApiError::Unauthorized
        })?;

    if !is_valid_password {
        return Err(ApiError::Unauthorized);
    }

    info!("Password verified for user: {}", payload.username);

    let roles = user
        .find_related(schemas::roles::Entity)
        .all(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    let role = roles
        .into_iter()
        .next()
        .ok_or_else(|| ApiError::Unauthorized)?;

    let access_token = generate_jwt(
        user.username.clone(),
        role.name.clone(),
        JWT_TYPE_ACCESS.to_string(),
        user.id,
        user.full_name.clone().unwrap_or_default(),
    )
    .await
    .map_err(|e| {
        ApiError::Unexpected(Box::new(std::io::Error::new(std::io::ErrorKind::Other, e)))
    })?;

    let response = ApiResponse {
        data: LoginResponseDTO::new(
            user.id,
            user.full_name.clone().unwrap_or_default(),
            user.username,
            role.name.clone(),
            access_token,
        ),
        total: 1,
        message: "Login successful".to_string(),
        status: "success".to_string(),
        code_error: 200,
        timestamp: chrono::Utc::now().to_rfc3339(),
    };

    info!("Login successful for user: {}", response.data.username);

    Ok(Json(response))
}

pub async fn get_profile(
    req: Request<Body>,
) -> Result<Json<ApiResponse<LoginResponseDTO>>, ApiError> {
    let auth_header = req
        .headers()
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    let token = match auth_header {
        Some(h) => {
            let h_lower = h.to_lowercase();
            if h_lower.starts_with("bearer ") {
                Some(h[7..].trim().to_string())
            } else {
                None
            }
        }
        None => None,
    };

    let token = match token {
        Some(t) => t,
        None => return Err(ApiError::Unauthorized),
    };

    // Validar token usando la implementación existente
    let claims = validate_token(&token)
        .await
        .map_err(|_| ApiError::Unauthorized)?;

    let token_validate = LoginResponseDTO::new(
        claims.id,
        claims.name,
        claims.user_name,
        claims.role,
        token.clone(),
    );

    Ok(Json(ApiResponse::success(
        token_validate,
        "Token Valid OK".to_string(),
        1,
    )))
}
