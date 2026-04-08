use axum::{
    Json,
    extract::{Path, Query, State},
};

use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, IntoActiveModel, ModelTrait,
    PaginatorTrait, QueryFilter, QueryOrder,
};
use validator::Validate;

use crate::api_module::role::role_dto::role_dto::{
    RoleDetailResponse, RoleIdResponse, RoleRequest,
};
use crate::{
    api_utils::{
        api_error::ApiError,
        api_response::{ApiResponse, PaginationParams},
        api_utils_fun::{to_page_index, to_page_limit},
    },
    config::config_database::config_db_context::AppContext,
};

pub async fn create_role(
    State(app_ctx): State<AppContext>,
    Json(payload): Json<RoleRequest>,
) -> Result<Json<ApiResponse<RoleIdResponse>>, ApiError> {
    payload.validate().map_err(ApiError::Validation)?;

    let role_create = schemas::roles::ActiveModel::try_from(payload)
        .map_err(|e| ApiError::Unexpected(Box::new(std::io::Error::other(e))))?;

    if role_create.name.is_not_set() {
        return Err(ApiError::Validation(validator::ValidationErrors::new()));
    }

    let new_role = role_create
        .save(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    if new_role.id.is_not_set() {
        return Err(ApiError::ValidationError(
            "Failed to create role".to_string(),
        ));
    }

    Ok(Json(ApiResponse::success(
        RoleIdResponse::from(new_role),
        "Role created successfully".to_string(),
        1,
    )))
}

pub async fn get_role_by_id(
    State(app_ctx): State<AppContext>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<RoleDetailResponse>>, ApiError> {
    let role = schemas::roles::Entity::find_by_id(id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    match role {
        Some(role) => Ok(Json(ApiResponse::success(
            RoleDetailResponse::from(role),
            "Role retrieved successfully".to_string(),
            1,
        ))),
        None => Err(ApiError::ValidationError("Role not found".to_string())),
    }
}

pub async fn get_roles(
    State(app_ctx): State<AppContext>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<ApiResponse<Vec<RoleDetailResponse>>>, ApiError> {
    let page_index = to_page_index(pagination.page);
    let page_limit = to_page_limit(pagination.limit);

    let paginator = schemas::roles::Entity::find()
        .order_by_asc(schemas::roles::Column::Id)
        .paginate(&app_ctx.conn, page_limit);

    let total_items = if pagination.total > 0 {
        pagination.total
    } else {
        paginator
            .num_items()
            .await
            .map_err(|e| ApiError::Unexpected(Box::new(e)))?
    };

    let roles = paginator
        .fetch_page(page_index)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    Ok(Json(ApiResponse::success(
        roles.into_iter().map(RoleDetailResponse::from).collect(),
        "Roles retrieved successfully".to_string(),
        total_items as i32,
    )))
}

pub async fn delete_role(
    State(app_ctx): State<AppContext>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    let role = schemas::roles::Entity::find_by_id(id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    match role {
        Some(role) => {
            role.delete(&app_ctx.conn)
                .await
                .map_err(|e| ApiError::Unexpected(Box::new(e)))?;
            Ok(Json(ApiResponse::success(
                (),
                "Role deleted successfully".to_string(),
                0,
            )))
        }
        None => Err(ApiError::ValidationError("Role not found".to_string())),
    }
}

pub async fn get_roles_by_name(
    State(app_ctx): State<AppContext>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<ApiResponse<Vec<RoleDetailResponse>>>, ApiError> {
    let page_index = to_page_index(pagination.page);
    let page_limit = to_page_limit(pagination.limit);
    let name_filter = pagination.name.clone().unwrap_or_default();

    if name_filter.is_empty() {
        return Err(ApiError::ValidationError(
            "Name filter cannot be empty".to_string(),
        ));
    }

    let paginator = schemas::roles::Entity::find()
        .filter(schemas::roles::Column::Name.eq(name_filter))
        .order_by_asc(schemas::roles::Column::Id)
        .paginate(&app_ctx.conn, page_limit);

    let total_items = if pagination.total > 0 {
        pagination.total
    } else {
        paginator
            .num_items()
            .await
            .map_err(|e| ApiError::Unexpected(Box::new(e)))?
    };

    let roles = paginator
        .fetch_page(page_index)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    Ok(Json(ApiResponse::success(
        roles.into_iter().map(RoleDetailResponse::from).collect(),
        "Roles retrieved successfully".to_string(),
        total_items as i32,
    )))
}

pub async fn update_role(
    State(app_ctx): State<AppContext>,
    Path(id): Path<i64>,
    Json(payload): Json<RoleRequest>,
) -> Result<Json<ApiResponse<RoleIdResponse>>, ApiError> {
    payload.validate().map_err(ApiError::Validation)?;

    let role = schemas::roles::Entity::find_by_id(id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    match role {
        Some(role) => {
            let mut role_active_model = role.into_active_model();

            role_active_model.name = ActiveValue::Set(payload.name);
            role_active_model.description = ActiveValue::Set(payload.description);

            let updated_role = role_active_model
                .save(&app_ctx.conn)
                .await
                .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

            Ok(Json(ApiResponse::success(
                RoleIdResponse::from(updated_role),
                "Role updated successfully".to_string(),
                0,
            )))
        }
        None => Err(ApiError::ValidationError("Role not found".to_string())),
    }
}
