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
    api_module::user_role::user_role_dto::user_role_dto::{UserRoleRequest, UserRoleResponse},
    api_utils::{
        api_error::ApiError,
        api_response::{ApiResponse, PaginationParams},
        api_utils_fun::{to_page_index, to_page_limit},
    },
    config::config_database::config_db_context::AppContext,
};

pub async fn create_user_role(
    State(app_ctx): State<AppContext>,
    Json(payload): Json<UserRoleRequest>,
) -> Result<Json<ApiResponse<UserRoleResponse>>, ApiError> {
    payload.validate().map_err(ApiError::Validation)?;

    let user_role_create = schemas::user_roles::ActiveModel::from(payload);

    if user_role_create.user_id.is_not_set() || user_role_create.role_id.is_not_set() {
        return Err(ApiError::Validation(validator::ValidationErrors::new()));
    }

    let new_user_role = user_role_create
        .save(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    Ok(Json(ApiResponse::success(
        UserRoleResponse::from(new_user_role),
        "User role created successfully".to_string(),
        1,
    )))
}

pub async fn get_user_role_by_user_id(
    State(app_ctx): State<AppContext>,
    Path((user_id, role_id)): Path<(i64, i64)>,
) -> Result<Json<ApiResponse<UserRoleResponse>>, ApiError> {
    let user_role = schemas::user_roles::Entity::find()
        .filter(schemas::user_roles::Column::UserId.eq(user_id))
        .filter(schemas::user_roles::Column::RoleId.eq(role_id))
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    match user_role {
        Some(ur) => Ok(Json(ApiResponse::success(
            UserRoleResponse::from(ur),
            "User role retrieved successfully".to_string(),
            1,
        ))),
        None => Err(ApiError::ValidationError("User role not found".to_string())),
    }
}

pub async fn delete_user_role(
    State(app_ctx): State<AppContext>,
    Path((user_id, role_id)): Path<(i64, i64)>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    let user_role = schemas::user_roles::Entity::find()
        .filter(schemas::user_roles::Column::UserId.eq(user_id))
        .filter(schemas::user_roles::Column::RoleId.eq(role_id))
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    match user_role {
        Some(ur) => {
            ur.delete(&app_ctx.conn)
                .await
                .map_err(|e| ApiError::Unexpected(Box::new(e)))?;
            Ok(Json(ApiResponse::success(
                (),
                "User role deleted successfully".to_string(),
                1,
            )))
        }
        None => Err(ApiError::ValidationError("User role not found".to_string())),
    }
}

pub async fn update_user_role(
    State(app_ctx): State<AppContext>,
    Path((user_id, role_id)): Path<(i64, i64)>,
    Json(payload): Json<UserRoleRequest>,
) -> Result<Json<ApiResponse<UserRoleResponse>>, ApiError> {
    payload.validate().map_err(ApiError::Validation)?;

    let user_role = schemas::user_roles::Entity::find()
        .filter(schemas::user_roles::Column::UserId.eq(user_id))
        .filter(schemas::user_roles::Column::RoleId.eq(role_id))
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    let mut user_role = match user_role {
        Some(ur) => ur.into_active_model(),
        None => return Err(ApiError::ValidationError("User role not found".to_string())),
    };

    user_role.user_id = ActiveValue::Set(payload.user_id);
    user_role.role_id = ActiveValue::Set(payload.role_id);

    let updated_user_role = user_role
        .save(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    Ok(Json(ApiResponse::success(
        UserRoleResponse::from(updated_user_role),
        "User role updated successfully".to_string(),
        1,
    )))
}

pub async fn get_user_roles(
    State(app_ctx): State<AppContext>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<ApiResponse<Vec<UserRoleResponse>>>, ApiError> {
    let page_index = to_page_index(pagination.page);
    let page_limit = to_page_limit(pagination.limit);

    let paginator = schemas::user_roles::Entity::find()
        .filter(schemas::user_roles::Column::UserId.eq(pagination.user_id))
        .order_by_asc(schemas::user_roles::Column::UserId)
        .order_by_asc(schemas::user_roles::Column::RoleId)
        .paginate(&app_ctx.conn, page_limit);

    let total_items = paginator
        .num_items()
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    let user_roles = paginator
        .fetch_page(page_index)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    Ok(Json(ApiResponse::success(
        user_roles.into_iter().map(UserRoleResponse::from).collect(),
        "User roles retrieved successfully".to_string(),
        total_items as i32,
    )))
}
