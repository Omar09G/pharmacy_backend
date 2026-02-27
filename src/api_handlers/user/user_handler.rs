use crate::{
    api_handlers::user::user_dto::{UserRequestDTO, UserResponseDTO},
    api_utils::{api_error::ApiError, api_response::ApiResponse},
    config::config_database::config_db_context::AppContext,
    config::config_pass::config_password::generate_hash,
};
use axum::{
    Json,
    extract::{Path, State},
};
use log::info;
use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, QueryFilter};
use validator::Validate;

/*
######################################################################################################
Fn get_user_handler
    - Description: Handler to get a user by ID.
    - Parameters:
        - State(app_ctx): The application context containing the database connection.
        - Path(user_id): The ID of the user to retrieve.
    - Returns: A JSON response containing the user data or an error message.
######################################################################################################
    */
pub async fn get_user_handler(
    State(app_ctx): State<AppContext>,
    Path(user_id): Path<i64>,
) -> Result<Json<ApiResponse<UserResponseDTO>>, ApiError> {
    info!("Received request to get user with ID: {}", user_id);

    let user = schemas::user::Entity::find_by_id(user_id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?
        .ok_or_else(|| ApiError::NotFound)?;

    let response = ApiResponse {
        data: user.into(),
        message: "User retrieved successfully".to_string(),
        status: "success".to_string(),
        code_error: 200,
        timestamp: chrono::Utc::now().to_rfc3339(),
    };

    Ok(Json(response))
}

/*
#############
######################################################################################################
Fn get_user_by_username_exit
    - Description: Handler to get a user by username.
    - Parameters:
        - State(app_ctx): The application context containing the database connection.
        - Path(username): The username of the user to retrieve.
    - Returns: A JSON response containing the user data or an error message.
######################################################################################################

*/

pub async fn get_user_by_username_exit(
    State(app_ctx): State<AppContext>,
    Path(username): Path<String>,
) -> Result<UserResponseDTO, ApiError> {
    info!("Received request to get user with username: {}", username);

    let user = schemas::user::Entity::find()
        .filter(schemas::user::Column::Username.eq(username.clone()))
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?
        .ok_or_else(|| ApiError::NotFound)?;

    Ok(user.into())
}

/*
######################################################################################################
Fn create_user_handler
    - Description: Handler to create a new user.
    - Parameters:
        - State(app_ctx): The application context containing the database connection.
        - Json(payload): The user data sent in the request body.
    - Returns: A JSON response containing the created user data or an error message.
######################################################################################################
*/

pub async fn create_user_handler(
    State(app_ctx): State<AppContext>,
    Json(payload): Json<UserRequestDTO>,
) -> Result<Json<ApiResponse<UserResponseDTO>>, ApiError> {
    info!(
        "Received request to create user with username: {}",
        payload.username
    );

    payload.validate().map_err(ApiError::Validation)?;

    let password_hash: Option<String> = if let Some(password) = &payload.password {
        Some(generate_hash(password).map_err(|s| {
            ApiError::Unexpected(Box::new(std::io::Error::new(std::io::ErrorKind::Other, s)))
        })?)
    } else {
        None
    };

    let user_model = schemas::user::ActiveModel {
        id: ActiveValue::NotSet,
        country: ActiveValue::Set(payload.country.into()),
        firstname: ActiveValue::Set(payload.firstname.into()),
        lastname: ActiveValue::Set(payload.lastname.into()),
        password: ActiveValue::Set(password_hash),
        role: ActiveValue::Set(payload.role.into()),
        username: ActiveValue::Set(payload.username.into()),
    }
    .save(&app_ctx.conn)
    .await?;

    let response = ApiResponse {
        data: user_model.into(),
        message: "User created successfully".to_string(),
        status: "success".to_string(),
        code_error: 201,
        timestamp: chrono::Utc::now().to_rfc3339(),
    };

    Ok(Json(response))
}
