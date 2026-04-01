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
    api_module::role_permissions::role_permissions_dto::role_permissions_dto::{
        RolePermissionsRequest, RolePermissionsResponse,
    },
    api_utils::{
        api_error::ApiError,
        api_response::{ApiResponse, PaginationParams},
        api_utils_fun::{to_page_index, to_page_limit},
    },
    config::config_database::config_db_context::AppContext,
};

pub async fn create_role_permissions(
    State(app_ctx): State<AppContext>,
    Json(payload): Json<RolePermissionsRequest>,
) -> Result<Json<ApiResponse<RolePermissionsResponse>>, ApiError> {
    payload.validate().map_err(ApiError::Validation)?;

    let role_permissions_create = schemas::role_permissions::ActiveModel::from(payload);

    if role_permissions_create.role_id.is_not_set()
        || role_permissions_create.permission_id.is_not_set()
    {
        return Err(ApiError::Validation(validator::ValidationErrors::new()));
    }

    let new_role_permissions = role_permissions_create
        .save(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    if new_role_permissions.role_id.is_not_set() || new_role_permissions.permission_id.is_not_set()
    {
        return Err(ApiError::ValidationError(
            "Failed to create role permissions".to_string(),
        ));
    }

    Ok(Json(ApiResponse::success(
        RolePermissionsResponse::from(new_role_permissions),
        "Role permissions created successfully".to_string(),
        1,
    )))
}

pub async fn get_role_permissions_by_id(
    State(app_ctx): State<AppContext>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<RolePermissionsResponse>>, ApiError> {
    let role_permissions = schemas::role_permissions::Entity::find()
        .filter(schemas::role_permissions::Column::RoleId.eq(id))
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    match role_permissions {
        Some(model) => Ok(Json(ApiResponse::success(
            RolePermissionsResponse::from(model),
            "Role permissions retrieved successfully".to_string(),
            1,
        ))),
        None => Err(ApiError::ValidationError(format!(
            "Role permissions with id {} not found",
            id
        ))),
    }
}

pub async fn get_role_permissions(
    State(app_ctx): State<AppContext>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<ApiResponse<Vec<RolePermissionsResponse>>>, ApiError> {
    let page_index = to_page_index(pagination.page);
    let page_limit = to_page_limit(pagination.limit);

    let role_permissions = schemas::role_permissions::Entity::find()
        .filter(schemas::role_permissions::Column::RoleId.eq(pagination.role_id))
        .order_by_asc(schemas::role_permissions::Column::RoleId)
        .paginate(&app_ctx.conn, page_limit)
        .fetch_page(page_index)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    let total_items = schemas::role_permissions::Entity::find()
        .count(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    let total_pages = (total_items as f64 / page_limit as f64).ceil() as usize;

    Ok(Json(ApiResponse::success(
        role_permissions
            .into_iter()
            .map(RolePermissionsResponse::from)
            .collect(),
        "Role permissions retrieved successfully".to_string(),
        total_pages as i32,
    )))
}

pub async fn delete_role_permissions(
    State(app_ctx): State<AppContext>,
    Path(role_id): Path<i64>,
    Path(permission_id): Path<i64>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    let role_permission = schemas::role_permissions::Entity::find()
        .filter(schemas::role_permissions::Column::RoleId.eq(role_id))
        .filter(schemas::role_permissions::Column::PermissionId.eq(permission_id))
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    match role_permission {
        Some(model) => {
            model
                .delete(&app_ctx.conn)
                .await
                .map_err(|e| ApiError::Unexpected(Box::new(e)))?;
            Ok(Json(ApiResponse::success(
                (),
                "Role permission deleted successfully".to_string(),
                0,
            )))
        }
        None => Err(ApiError::ValidationError(format!(
            "Role permission with role_id {} and permission_id {} not found",
            role_id, permission_id
        ))),
    }
}

pub async fn update_role_permissions(
    State(app_ctx): State<AppContext>,
    Path(role_id): Path<i64>,
    Path(permission_id): Path<i64>,
    Json(payload): Json<RolePermissionsRequest>,
) -> Result<Json<ApiResponse<RolePermissionsResponse>>, ApiError> {
    payload.validate().map_err(ApiError::Validation)?;

    let role_permission = schemas::role_permissions::Entity::find()
        .filter(schemas::role_permissions::Column::RoleId.eq(role_id))
        .filter(schemas::role_permissions::Column::PermissionId.eq(permission_id))
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    match role_permission {
        Some(model) => {
            let mut active_model: schemas::role_permissions::ActiveModel =
                model.into_active_model();
            active_model.role_id = ActiveValue::Set(payload.role_id);
            active_model.permission_id = ActiveValue::Set(payload.permission_id);

            let updated_role_permission = active_model
                .save(&app_ctx.conn)
                .await
                .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

            Ok(Json(ApiResponse::success(
                RolePermissionsResponse::from(updated_role_permission),
                "Role permission updated successfully".to_string(),
                1,
            )))
        }
        None => Err(ApiError::ValidationError(format!(
            "Role permission with role_id {} and permission_id {} not found",
            role_id, permission_id
        ))),
    }
}
