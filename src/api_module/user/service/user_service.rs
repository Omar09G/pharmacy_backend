use axum::{
    Json,
    extract::{Path, Query, State},
};

use log::info;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, IntoActiveModel, LoaderTrait,
    PaginatorTrait, QueryFilter, QueryOrder, TransactionTrait,
};
use validator::Validate;

use crate::{
    api_module::{
        user::dto::user_dto::{
            UserChangePasswordRequest, UserChangeStatusRequest, UserIdResponse, UserRequestDto,
            UserResponse, UserUpdateRequestDto,
        },
        utils::utils::{ACTIVE_STATUS, INACTIVE_STATUS},
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
) -> Result<Json<ApiResponse<UserIdResponse>>, ApiError> {
    info!("create_user called with payload: {:?}", payload);

    payload.validate().map_err(ApiError::Validation)?;

    let role_name = payload.role.clone();

    // Offload Argon2 hashing to blocking thread to avoid blocking async runtime
    let plain_password = payload.password.clone();
    let new_password_hash = tokio::task::spawn_blocking(move || generate_hash(&plain_password))
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?
        .map_err(|e| ApiError::Unexpected(Box::new(std::io::Error::other(e))))?;

    // Wrap user + user_role creation in a transaction for atomicity
    let txn = app_ctx
        .conn
        .begin()
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    let op_result = (async {
        let user_create = schemas::users::ActiveModel {
            id: ActiveValue::NotSet,
            username: ActiveValue::Set(payload.username),
            password_hash: ActiveValue::Set(new_password_hash),
            full_name: ActiveValue::Set(payload.full_name),
            email: ActiveValue::Set(payload.email),
            phone: ActiveValue::Set(payload.phone),
            status: ActiveValue::Set(payload.status),
            created_at: ActiveValue::Set(get_current_timestamp_now()),
            created_by: ActiveValue::NotSet,
            updated_at: ActiveValue::NotSet,
            updated_by: ActiveValue::NotSet,
            deleted_at: ActiveValue::NotSet,
        };

        let new_user = user_create
            .save(&txn)
            .await
            .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

        let user_id: i64 = new_user.id.unwrap();

        info!(
            "Creating user role with user_id: {} and role: {}",
            user_id, role_name
        );

        let role = schemas::roles::Entity::find()
            .filter(schemas::roles::Column::Name.eq(role_name))
            .one(&txn)
            .await
            .map_err(|e| ApiError::Unexpected(Box::new(e)))?
            .ok_or(ApiError::NotFoundErrorDescription(
                "Role not found".to_string(),
            ))?;

        let user_role_create = schemas::user_roles::ActiveModel {
            user_id: ActiveValue::Set(user_id),
            role_id: ActiveValue::Set(role.id),
        };

        user_role_create
            .insert(&txn)
            .await
            .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

        Ok(user_id)
    })
    .await;

    match op_result {
        Ok(user_id) => {
            txn.commit()
                .await
                .map_err(|e| ApiError::Unexpected(Box::new(e)))?;
            info!("User and role created successfully");
            Ok(Json(ApiResponse::success(
                UserIdResponse { id: user_id },
                "User created successfully".to_string(),
                1,
            )))
        }
        Err(err) => {
            let _ = txn.rollback().await;
            Err(err)
        }
    }
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
    info!("get_user_by_id called with user_id: {:?}", user_id);

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
    info!(
        "get_all_users called with pagination: page={:?}, limit={:?}, total={:?}, full_name={:?}",
        pagination.page,
        pagination.limit,
        pagination.total,
        pagination.full_name
    );

    let mut select = schemas::users::Entity::find();

    if let Some(full_name) = pagination.full_name.clone() {
        if !full_name.is_empty() {
            select = select.filter(schemas::users::Column::FullName.contains(full_name));
        }
    }

    let paginator = select
        .order_by_asc(schemas::users::Column::Id)
        .paginate(&app_ctx.conn, to_page_limit(pagination.limit));

    let total_items = if pagination.total > 0 {
        pagination.total
    } else {
        paginator
            .num_items()
            .await
            .map_err(|e| ApiError::Unexpected(Box::new(e)))?
    };

    let users = paginator
        .fetch_page(to_page_index(pagination.page))
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    let mut users_with_roles: Vec<UserResponse> = Vec::new();

    // Batch load roles for all users (2 queries instead of 2*N)
    let roles_per_user: Vec<Vec<schemas::roles::Model>> = users
        .load_many_to_many(
            schemas::roles::Entity,
            schemas::user_roles::Entity,
            &app_ctx.conn,
        )
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    for (user, roles) in users.iter().zip(roles_per_user.iter()) {
        let role_name = roles.first().map(|r| r.name.clone()).unwrap_or_default();
        users_with_roles.push(UserResponse::from((user.clone(), role_name)));
    }

    if users.is_empty() {
        return Err(ApiError::NotFound);
    }

    Ok(Json(ApiResponse::success(
        users_with_roles,
        "Users retrieved successfully".to_string(),
        total_items as i32,
    )))
}

pub async fn change_user_status(
    State(app_ctx): State<AppContext>,
    Query(payload): Query<UserChangeStatusRequest>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    info!("change_user_status called with payload: {:?}", payload);

    payload.validate().map_err(ApiError::Validation)?;

    if payload.status != ACTIVE_STATUS && payload.status != INACTIVE_STATUS {
        return Err(ApiError::Validation(validator::ValidationErrors::new()));
    }

    let user = schemas::users::Entity::find()
        .filter(schemas::users::Column::Username.eq(payload.username))
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?
        .ok_or_else(|| ApiError::NotFound)?;

    let mut user_active_model = user.into_active_model();

    if payload.status.clone() == ACTIVE_STATUS || payload.status.clone() == INACTIVE_STATUS {
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
    info!("change_user_password called with payload: {:?}", payload);

    payload.validate().map_err(ApiError::Validation)?;

    let user = schemas::users::Entity::find()
        .filter(schemas::users::Column::Username.eq(payload.username.clone()))
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?
        .ok_or_else(|| ApiError::NotFound)?;

    let mut user_active_model = user.into_active_model();

    let plain = payload.password.clone();
    let new_password_hash = tokio::task::spawn_blocking(move || {
        crate::config::config_pass::config_password::generate_hash(&plain)
    })
    .await
    .map_err(|e| ApiError::Unexpected(Box::new(e)))?
    .map_err(|e| ApiError::Unexpected(Box::new(std::io::Error::other(e))))?;

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
    Path(user_id): Path<i64>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    info!("delete_user called with user_id: {:?}", user_id);


    let user = schemas::users::Entity::find_by_id(user_id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?
        .ok_or(ApiError::NotFound)?;

    if user.deleted_at.is_some() {
        return Err(ApiError::NotFoundErrorDescription(
            "User already deleted".to_string(),
        ));
    }

    let mut user_active = user.into_active_model();
    user_active.deleted_at = ActiveValue::Set(Some(get_current_timestamp_now()));
    user_active.status = ActiveValue::Set("DELETED".to_string());
    user_active
        .save(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    Ok(Json(ApiResponse::success(
        (),
        "User deleted successfully".to_string(),
        1,
    )))
}

pub async fn update_user(
    State(app_ctx): State<AppContext>,
    Path(id): Path<i64>,
    Json(payload): Json<UserUpdateRequestDto>,
) -> Result<Json<ApiResponse<UserResponse>>, ApiError> {
    info!("update_user called with payload: {:?}, id: {:?}", payload, id);

    payload.validate().map_err(ApiError::Validation)?;

    let user = schemas::users::Entity::find_by_id(id)
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
    user_active_model.updated_by = ActiveValue::Set(Some(payload.updated_by));

    if let Some(str_status) = &payload.status {
        user_active_model.status = ActiveValue::Set(str_status.clone());
    }

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
