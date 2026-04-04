use axum::{
    Json,
    extract::{Path, Query, State},
};

use log::{info, warn};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, ModelTrait, PaginatorTrait, QueryFilter, QueryOrder,
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

    let role_id_str = payload.role_id;
    let permission_id_str = payload.permission_id;

    info!(
        "Creating role permissions with role_id: {} and permission_id: {}",
        payload.role_id, payload.permission_id
    );

    let role_permissions_create = schemas::role_permissions::ActiveModel::try_from(payload)
        .map_err(|e| ApiError::Unexpected(Box::new(std::io::Error::other(e))))?;

    if role_permissions_create.role_id.is_not_set()
        || role_permissions_create.permission_id.is_not_set()
    {
        return Err(ApiError::Validation(validator::ValidationErrors::new()));
    }

    warn!(
        "Attempting to create role permissions with role_id: {} and permission_id: {}",
        role_permissions_create.role_id.clone().unwrap(),
        role_permissions_create.permission_id.clone().unwrap()
    );

    role_permissions_create
        .insert(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    Ok(Json(ApiResponse::success(
        RolePermissionsResponse::new(role_id_str, permission_id_str),
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
    let role_id = pagination.role_id.unwrap_or_default();

    info!(
        "Retrieving role permissions for role_id: {} with page: {} and limit: {}",
        role_id, pagination.page, pagination.limit
    );

    let paginator = schemas::role_permissions::Entity::find()
        .filter(schemas::role_permissions::Column::RoleId.eq(role_id))
        .order_by_asc(schemas::role_permissions::Column::PermissionId)
        .paginate(&app_ctx.conn, page_limit);
    let total_items = paginator
        .num_items()
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    let items = paginator
        .fetch_page(page_index)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    Ok(Json(ApiResponse::success(
        items
            .into_iter()
            .map(RolePermissionsResponse::from)
            .collect(),
        "Role permissions retrieved successfully".to_string(),
        total_items as i32,
    )))
}

pub async fn delete_role_permissions(
    State(app_ctx): State<AppContext>,
    Path((role_id, permission_id)): Path<(i64, i64)>,
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
    Path((role_id, permission_id)): Path<(i64, i64)>,
    Json(payload): Json<RolePermissionsRequest>,
) -> Result<Json<ApiResponse<RolePermissionsResponse>>, ApiError> {
    payload.validate().map_err(ApiError::Validation)?;

    let role_id_str = payload.role_id;
    let permission_id_str = payload.permission_id;

    let role_permission = schemas::role_permissions::Entity::find()
        .filter(schemas::role_permissions::Column::RoleId.eq(role_id))
        .filter(schemas::role_permissions::Column::PermissionId.eq(permission_id))
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    if role_permission.is_none() {
        return Err(ApiError::ValidationError(format!(
            "Role permission with role_id {} and permission_id {} not found",
            role_id, permission_id
        )));
    }

    //Eliminar con la funcion delete y luego insertar con la funcion create
    let _ = delete_role_permissions(State(app_ctx.clone()), Path((role_id, permission_id))).await?;
    let _ = create_role_permissions(State(app_ctx.clone()), Json(payload)).await?;

    let role_permission_update = schemas::role_permissions::Entity::find()
        .filter(schemas::role_permissions::Column::RoleId.eq(role_id_str))
        .filter(schemas::role_permissions::Column::PermissionId.eq(permission_id_str))
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    Ok(Json(ApiResponse::success(
        RolePermissionsResponse::from(role_permission_update.unwrap()),
        "Role permissions updated successfully".to_string(),
        1,
    )))
}
