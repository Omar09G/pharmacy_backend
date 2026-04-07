use axum::{
    Json,
    extract::{Path, Query, State},
};

use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, IntoActiveModel, ModelTrait,
    PaginatorTrait, QueryFilter, QueryOrder,
};
use validator::Validate;

use crate::api_module::permissions::permissions_dto::permissions_dto::{
    PermissionDetailResponse, PermissionRequest, PermissionResponse,
};
use crate::{
    api_utils::{
        api_error::ApiError,
        api_response::{ApiResponse, PaginationParams},
        api_utils_fun::{to_page_index, to_page_limit},
    },
    config::config_database::config_db_context::AppContext,
};

pub async fn create_permission(
    State(app_ctx): State<AppContext>,
    Json(payload): Json<PermissionRequest>,
) -> Result<Json<ApiResponse<PermissionResponse>>, ApiError> {
    payload.validate().map_err(ApiError::Validation)?;

    let permission_create = schemas::permissions::ActiveModel::try_from(payload)
        .map_err(|e| ApiError::Unexpected(Box::new(std::io::Error::other(e))))?;

    if permission_create.name.is_not_set() {
        return Err(ApiError::Validation(validator::ValidationErrors::new()));
    }

    let new_permission = permission_create
        .save(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    if new_permission.id.is_not_set() {
        return Err(ApiError::ValidationError(
            "Failed to create permission".to_string(),
        ));
    }

    Ok(Json(ApiResponse::success(
        PermissionResponse::from(new_permission),
        "Permission created successfully".to_string(),
        1,
    )))
}

pub async fn get_permission_by_id(
    State(app_ctx): State<AppContext>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<PermissionDetailResponse>>, ApiError> {
    let permission = schemas::permissions::Entity::find_by_id(id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    match permission {
        Some(permission) => Ok(Json(ApiResponse::success(
            PermissionDetailResponse::from(permission),
            "Permission retrieved successfully".to_string(),
            1,
        ))),
        None => Err(ApiError::ValidationError(format!(
            "Permission with id {} not found",
            id
        ))),
    }
}

pub async fn get_permissions_by_name(
    State(app_ctx): State<AppContext>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<ApiResponse<Vec<PermissionDetailResponse>>>, ApiError> {
    let name_str = pagination.name.clone().unwrap_or_default();

    if name_str.is_empty() {
        return Err(ApiError::ValidationError(
            "Name query parameter is required".to_string(),
        ));
    }

    let paginator = schemas::permissions::Entity::find()
        .filter(schemas::permissions::Column::Name.eq(name_str))
        .order_by_asc(schemas::permissions::Column::Id)
        .paginate(&app_ctx.conn, to_page_limit(pagination.limit));
     
     let total_items = if pagination.total > 0 {
        pagination.total
    } else {
        paginator
            .num_items()
            .await
            .map_err(|e| ApiError::Unexpected(Box::new(e)))?
    };

    let items = paginator
        .fetch_page(to_page_index(pagination.page))
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    Ok(Json(ApiResponse::success(
        items
            .into_iter()
            .map(PermissionDetailResponse::from)
            .collect(),
        "Permissions retrieved successfully".to_string(),
        total_items as i32,
    )))
}
pub async fn get_permissions(
    State(app_ctx): State<AppContext>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<ApiResponse<Vec<PermissionDetailResponse>>>, ApiError> {
    let paginator = schemas::permissions::Entity::find()
        .order_by_asc(schemas::permissions::Column::Id)
        .paginate(&app_ctx.conn, to_page_limit(pagination.limit));
    
    let total_items = if pagination.total > 0 {
        pagination.total
    } else {
        paginator
            .num_items()
            .await
            .map_err(|e| ApiError::Unexpected(Box::new(e)))?
    };

    let items = paginator
        .fetch_page(to_page_index(pagination.page))
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    Ok(Json(ApiResponse::success(
        items
            .into_iter()
            .map(PermissionDetailResponse::from)
            .collect(),
        "Permissions retrieved successfully".to_string(),
        total_items as i32,
    )))
}

pub async fn delete_permission(
    State(app_ctx): State<AppContext>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    let permission = schemas::permissions::Entity::find_by_id(id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    match permission {
        Some(permission) => {
            permission
                .delete(&app_ctx.conn)
                .await
                .map_err(|e| ApiError::Unexpected(Box::new(e)))?;
            Ok(Json(ApiResponse::success(
                (),
                "Permission deleted successfully".to_string(),
                0,
            )))
        }
        None => Err(ApiError::ValidationError(format!(
            "Permission with id {} not found",
            id
        ))),
    }
}

pub async fn update_permission(
    State(app_ctx): State<AppContext>,
    Path(id): Path<i64>,
    Json(payload): Json<PermissionRequest>,
) -> Result<Json<ApiResponse<PermissionResponse>>, ApiError> {
    payload.validate().map_err(ApiError::Validation)?;

    let permission = schemas::permissions::Entity::find_by_id(id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    match permission {
        Some(permission) => {
            let mut permission_active_model = permission.into_active_model();
            permission_active_model.name = ActiveValue::Set(payload.name);
            permission_active_model.description = ActiveValue::Set(payload.description);

            let updated_permission = permission_active_model
                .save(&app_ctx.conn)
                .await
                .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

            Ok(Json(ApiResponse::success(
                PermissionResponse::from(updated_permission),
                "Permission updated successfully".to_string(),
                1,
            )))
        }
        None => Err(ApiError::ValidationError(format!(
            "Permission with id {} not found",
            id
        ))),
    }
}
