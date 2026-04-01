use axum::{
    Json,
    extract::{Path, Query, State},
};

use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, IntoActiveModel, ModelTrait,
    PaginatorTrait, QueryFilter, QueryOrder,
};
use validator::Validate;

use crate::{
    api_module::{
        user::dto::user_dto::{
            UserChangePasswordRequest, UserChangeStatusRequest, UserRequestDto, UserResponse,
        },
        utils::utils::ACTIVE_STATUS,
    },
    api_utils::{
        api_error::ApiError,
        api_response::{ApiResponse, PaginationParams},
        api_utils_fun::{get_current_timestamp_now, to_page_index, to_page_limit},
    },
    config::{
        config_database::config_db_context::AppContext, config_pass::config_password::generate_hash,
    },
};

/* Name fn: create_user
Description:   Funcion para crear un user en la base de datos
Parameters:   - State(app_ctx): Contexto de la aplicación que contiene la conexión a la base de datos
              - Json(payload): Payload de la solicitud que contiene los datos del usuario a crear
Returns:      - Result<Json<ApiResponse<UserResponse>>, ApiError>: Resultado de la operación, que puede
                ser un Json con la respuesta del usuario creado o un ApiError en caso de error
*/
pub async fn create_user(
    State(app_ctx): State<AppContext>,
    Json(payload): Json<UserRequestDto>,
) -> Result<Json<ApiResponse<UserResponse>>, ApiError> {
    payload.validate().map_err(ApiError::Validation)?;

    let user_create = schemas::users::ActiveModel::from(payload);

    if user_create.username.is_not_set()
        || user_create.password_hash.is_not_set()
        || user_create.status.is_not_set()
    {
        return Err(ApiError::Validation(validator::ValidationErrors::new()));
    }

    let new_user = user_create
        .save(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    Ok(Json(ApiResponse::success(
        UserResponse::from(new_user),
        "User created successfully".to_string(),
        1,
    )))
}
/* Name fn: get_user_by_id
Description:   Funcion para obtener un user por su ID desde la base de datos
Parameters:   - State(app_ctx): Contexto de la aplicación que contiene la conexión a la base de datos
              - Path(user_id): ID del usuario a obtener, extraído de la ruta de la solicitud
Returns:      - Result<Json<ApiResponse<UserResponse>>, ApiError>: Resultado de la operación, que puede ser un Json con la respuesta del usuario encontrado o un ApiError en caso de error
*/
pub async fn get_user_by_id(
    State(app_ctx): State<AppContext>,
    Path(user_id): Path<i64>,
) -> Result<Json<ApiResponse<UserResponse>>, ApiError> {
    let user = schemas::users::Entity::find_by_id(user_id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?
        .ok_or_else(|| ApiError::NotFound)?;

    if user.deleted_at.is_some() {
        return Err(ApiError::NotFound);
    }

    Ok(Json(ApiResponse::success(
        UserResponse::from(user),
        "User retrieved successfully".to_string(),
        1,
    )))
}

/* Name fn: get_all_users
Description:   Funcion para obtener todos los users desde la base de datos
Parameters:   - State(app_ctx): Contexto de la aplicación que contiene la conexión a la base de datos
              - Query(pagination): Parámetros de paginación extraídos de la consulta de la solicitud, que incluyen el número de página y el límite de items por página
Returns:      - Result<Json<ApiResponse<Vec<UserResponse>>>, ApiError>: Resultado de la operación, que puede ser un Json con la respuesta de la lista de usuarios encontrados o un ApiError en caso de error
*/
pub async fn get_all_users(
    State(app_ctx): State<AppContext>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<ApiResponse<Vec<UserResponse>>>, ApiError> {
    let paginator = schemas::users::Entity::find()
        .order_by_asc(schemas::users::Column::Id)
        .paginate(&app_ctx.conn, to_page_limit(pagination.limit));

    let total_items = paginator
        .num_items()
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    let users = paginator
        .fetch_page(to_page_index(pagination.page))
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    if users.is_empty() {
        return Err(ApiError::NotFound);
    }

    Ok(Json(ApiResponse::success(
        users.into_iter().map(UserResponse::from).collect(),
        "Users retrieved successfully".to_string(),
        total_items as i32,
    )))
}

pub async fn change_user_status(
    State(app_ctx): State<AppContext>,
    Query(payload): Query<UserChangeStatusRequest>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    payload.validate().map_err(ApiError::Validation)?;

    let user = schemas::users::Entity::find()
        .filter(schemas::users::Column::Username.eq(payload.username))
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?
        .ok_or_else(|| ApiError::NotFound)?;

    let mut user_active_model = user.into_active_model();

    if payload.status.clone() == ACTIVE_STATUS {
        user_active_model.deleted_at = ActiveValue::Set(None);
        user_active_model.updated_at = ActiveValue::Set(Some(get_current_timestamp_now()));
        user_active_model.updated_by = ActiveValue::Set(Some(payload.updated_by));
    } else {
        user_active_model.deleted_at = ActiveValue::Set(Some(get_current_timestamp_now()));
    }

    user_active_model.status = ActiveValue::Set(payload.status);

    user_active_model
        .save(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    Ok(Json(ApiResponse::success(
        (),
        "User status changed successfully".to_string(),
        1,
    )))
}

pub async fn change_user_password(
    State(app_ctx): State<AppContext>,
    Json(payload): Json<UserChangePasswordRequest>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    payload.validate().map_err(ApiError::Validation)?;

    let user = schemas::users::Entity::find()
        .filter(schemas::users::Column::Username.eq(payload.username.clone()))
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?
        .ok_or_else(|| ApiError::NotFound)?;

    let mut user_active_model = user.into_active_model();

    let new_password_hash = generate_hash(&payload.password).map_err(|e| {
        ApiError::Unexpected(Box::new(std::io::Error::new(std::io::ErrorKind::Other, e)))
    })?;

    user_active_model.password_hash = ActiveValue::Set(new_password_hash);
    user_active_model.updated_at = ActiveValue::Set(Some(get_current_timestamp_now()));
    user_active_model.updated_by = ActiveValue::Set(Some(payload.updated_by));

    user_active_model
        .save(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    Ok(Json(ApiResponse::success(
        (),
        "User password changed successfully".to_string(),
        1,
    )))
}

pub async fn delete_user(
    State(app_ctx): State<AppContext>,
    Query(payload): Query<UserChangeStatusRequest>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    let user = schemas::users::Entity::find()
        .filter(schemas::users::Column::Username.eq(payload.username.clone()))
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?
        .ok_or_else(|| ApiError::NotFound)?;

    user.delete(&app_ctx.conn).await?;

    Ok(Json(ApiResponse::success(
        (),
        "User deleted successfully".to_string(),
        1,
    )))
}

pub async fn update_user(
    State(app_ctx): State<AppContext>,
    Json(payload): Json<UserRequestDto>,
) -> Result<Json<ApiResponse<UserResponse>>, ApiError> {
    payload.validate().map_err(ApiError::Validation)?;

    let user = schemas::users::Entity::find_by_id(payload.id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?
        .ok_or_else(|| ApiError::NotFound)?;

    let mut user_active_model = user.into_active_model();

    if let Some(str_full_name) = &payload.full_name {
        user_active_model.full_name = ActiveValue::Set(Some(str_full_name.clone()));
    }
    if let Some(str_email) = &payload.email {
        user_active_model.email = ActiveValue::Set(Some(str_email.clone()));
    }
    if let Some(str_phone) = &payload.phone {
        user_active_model.phone = ActiveValue::Set(Some(str_phone.clone()));
    }
    user_active_model.updated_at = ActiveValue::Set(Some(get_current_timestamp_now()));
    user_active_model.updated_by = ActiveValue::Set(Some(payload.updated_by.unwrap_or(0)));

    let updated_user = user_active_model
        .save(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    Ok(Json(ApiResponse::success(
        UserResponse::from(updated_user),
        "User updated successfully".to_string(),
        1,
    )))
}
