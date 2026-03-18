use crate::{
    api_handlers::user::user_dto::{UserRequestDTO, UserResponseDTO},
    api_utils::{
        api_error::ApiError,
        api_response::{ApiResponse, PaginationParams},
    },
    config::{
        config_database::config_db_context::AppContext, config_pass::config_password::generate_hash,
    },
};
use axum::{
    Json,
    extract::{Path, Query, State},
};
use log::info;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, IntoActiveModel, ModelTrait,
    PaginatorTrait, QueryFilter,
};
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
        total: 1,
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
        total: 1,
        message: "User created successfully".to_string(),
        status: "success".to_string(),
        code_error: 201,
        timestamp: chrono::Utc::now().to_rfc3339(),
    };

    Ok(Json(response))
}
/*
fn delete_user_handler
   - Description: Handler to delete a user by ID.
   - Parameters:
       - State(app_ctx): The application context containing the database connection.
       - Path(user_id): The ID of the user to delete.
   - Returns: A JSON response indicating the success or failure of the deletion operation.
*/

pub async fn delete_user_handler(
    State(app_ctx): State<AppContext>,
    Path(user_id): Path<i64>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    info!("Received request to delete user with ID: {}", user_id);

    let user = schemas::user::Entity::find_by_id(user_id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?
        .ok_or_else(|| ApiError::NotFound)?;

    user.delete(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    let response = ApiResponse {
        data: (),
        total: 0,
        message: "User deleted successfully".to_string(),
        status: "success".to_string(),
        code_error: 200,
        timestamp: chrono::Utc::now().to_rfc3339(),
    };

    Ok(Json(response))
}

/*
######################################################################################################
Fn get_all_users_handler
    - Description: Handler to get all users with pagination support.
    - Parameters:
        - State(app_ctx): The application context containing the database connection.
        - Query(_pagination): The pagination parameters sent in the query string.
    - Returns: A JSON response containing a list of users or an error message.
######################################################################################################
*/

pub async fn get_all_users_handler(
    State(app_ctx): State<AppContext>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<ApiResponse<Vec<UserResponseDTO>>>, ApiError> {
    info!("Received request to get all users");

    let users = schemas::user::Entity::find()
        .paginate(&app_ctx.conn, pagination.limit)
        .fetch_page(pagination.page - 1)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    let num_user = users.len();

    let response = ApiResponse {
        data: users.into_iter().map(Into::into).collect(),
        total: num_user as i32,
        message: "Users retrieved successfully".to_string(),
        status: "success".to_string(),
        code_error: 200,
        timestamp: chrono::Utc::now().to_rfc3339(),
    };

    Ok(Json(response))
}

/*
######################################################################################################
fn update_user_handler
   - Description: Handler to update a user by ID.
   - Parameters:
       - State(app_ctx): The application context containing the database connection.
       - Path(user_id): The ID of the user to update.
       - Json(payload): The user data sent in the request body for updating.
   - Returns: A JSON response containing the updated user data or an error message.
   ######################################################################################################
*/

pub async fn update_user_handler(
    State(app_ctx): State<AppContext>,
    Path(user_id): Path<i64>,
    Json(payload): Json<UserRequestDTO>,
) -> Result<Json<ApiResponse<UserResponseDTO>>, ApiError> {
    info!("Received request to update user with ID: {}", user_id);

    payload.validate().map_err(ApiError::Validation)?;

    let mut user = schemas::user::Entity::find_by_id(user_id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?
        .ok_or_else(|| ApiError::NotFound)?;

    if let Some(password) = &payload.password {
        user.password = Some(generate_hash(password).map_err(|s| {
            ApiError::Unexpected(Box::new(std::io::Error::new(std::io::ErrorKind::Other, s)))
        })?);
    }

    user.country = payload.country;
    user.firstname = payload.firstname;
    user.lastname = payload.lastname;
    user.role = payload.role;
    user.username = payload.username;

    let active_user = user.into_active_model();
    let updated_user = active_user.save(&app_ctx.conn).await?;

    let num_user = 1;

    let response = ApiResponse {
        data: updated_user.into(),
        total: num_user,
        message: "User updated successfully".to_string(),
        status: "success".to_string(),
        code_error: 200,
        timestamp: chrono::Utc::now().to_rfc3339(),
    };

    Ok(Json(response))
}

/*
######################################################################################################
fn partial_update_user_handler
   - Description: Handler to partially update a user by ID.
   - Parameters:
       - State(app_ctx): The application context containing the database connection.
       - Path(user_id): The ID of the user to partially update.
       - Json(payload): The user data sent in the request body for partial updating.
   - Returns: A JSON response containing the updated user data or an error message.
######################################################################################################
*/

pub async fn partial_update_user_handler(
    State(app_ctx): State<AppContext>,
    Path(user_id): Path<i64>,
    Json(payload): Json<UserRequestDTO>,
) -> Result<Json<ApiResponse<UserResponseDTO>>, ApiError> {
    info!(
        "Received request to partially update user with ID: {}",
        user_id
    );

    payload.validate().map_err(ApiError::Validation)?;

    let mut user = schemas::user::Entity::find_by_id(user_id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?
        .ok_or_else(|| ApiError::NotFound)?;

    if let Some(password) = &payload.password {
        user.password = Some(generate_hash(password).map_err(|s| {
            ApiError::Unexpected(Box::new(std::io::Error::new(std::io::ErrorKind::Other, s)))
        })?);
    }

    if payload.country.is_some() {
        user.country = payload.country;
    }
    if payload.firstname.is_some() {
        user.firstname = payload.firstname;
    }

    user.lastname = payload.lastname;

    if payload.role.is_some() {
        user.role = payload.role;
    }

    user.username = payload.username;

    let active_user = user.into_active_model();
    let updated_user = active_user.save(&app_ctx.conn).await?;

    let response = ApiResponse {
        data: updated_user.into(),
        total: 1,
        message: "User partially updated successfully".to_string(),
        status: "success".to_string(),
        code_error: 200,
        timestamp: chrono::Utc::now().to_rfc3339(),
    };

    Ok(Json(response))
}
