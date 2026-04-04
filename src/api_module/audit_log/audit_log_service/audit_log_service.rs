use axum::{
    Json,
    extract::{Path, Query, State},
};

use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, IntoActiveModel, ModelTrait,
    PaginatorTrait, QueryFilter, QueryOrder,
};
use validator::Validate;

use crate::api_module::audit_log::audit_log_dto::audit_log_dto::{
    AuditLogDetailResponse, AuditLogIdResponse, AuditLogRequest,
};
use crate::{
    api_utils::{
        api_error::ApiError,
        api_response::{ApiResponse, PaginationParams},
        api_utils_fun::{to_page_index, to_page_limit},
    },
    config::config_database::config_db_context::AppContext,
};

pub async fn create_audit_log(
    State(app_ctx): State<AppContext>,
    Json(payload): Json<AuditLogRequest>,
) -> Result<Json<ApiResponse<AuditLogIdResponse>>, ApiError> {
    payload.validate().map_err(ApiError::Validation)?;

    let al_create = schemas::audit_log::ActiveModel::try_from(payload)
        .map_err(|e| ApiError::Unexpected(Box::new(std::io::Error::other(e))))?;

    let new_al = al_create
        .save(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    if new_al.id.is_not_set() {
        return Err(ApiError::ValidationError(
            "Failed to create audit log".to_string(),
        ));
    }

    Ok(Json(ApiResponse::success(
        AuditLogIdResponse::from(new_al),
        "Audit log created successfully".to_string(),
        1,
    )))
}

pub async fn get_audit_log_by_id(
    State(app_ctx): State<AppContext>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<AuditLogDetailResponse>>, ApiError> {
    let al = schemas::audit_log::Entity::find_by_id(id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    match al {
        Some(al) => Ok(Json(ApiResponse::success(
            AuditLogDetailResponse::from(al),
            "Audit log retrieved successfully".to_string(),
            1,
        ))),
        None => Err(ApiError::ValidationError("Audit log not found".to_string())),
    }
}

pub async fn get_audit_logs(
    State(app_ctx): State<AppContext>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<ApiResponse<Vec<AuditLogDetailResponse>>>, ApiError> {
    let page_index = to_page_index(pagination.page);
    let page_limit = to_page_limit(pagination.limit);

    let mut select = schemas::audit_log::Entity::find();

    if let Some(entity_type) = pagination.entity_type.clone()
        && !entity_type.is_empty()
    {
        select = select.filter(schemas::audit_log::Column::EntityType.eq(entity_type));
    }

    if let Some(table_name) = pagination.table_name.clone()
        && !table_name.is_empty()
    {
        select = select.filter(schemas::audit_log::Column::TableName.eq(table_name));
    }

    if let Some(action) = pagination.action.clone()
        && !action.is_empty()
    {
        select = select.filter(schemas::audit_log::Column::Action.eq(action));
    }

    if let Some(eid) = pagination.entity_id {
        select = select.filter(schemas::audit_log::Column::EntityId.eq(eid));
    }

    if let Some(user) = pagination.changed_by.or(pagination.user_id) {
        select = select.filter(schemas::audit_log::Column::ChangedBy.eq(user));
    }

    let paginator = select
        .order_by_desc(schemas::audit_log::Column::ChangedAt)
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
            .map(AuditLogDetailResponse::from)
            .collect(),
        "Audit logs retrieved successfully".to_string(),
        total_items as i32,
    )))
}

pub async fn delete_audit_log(
    State(app_ctx): State<AppContext>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    let al = schemas::audit_log::Entity::find_by_id(id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    match al {
        Some(al) => {
            al.delete(&app_ctx.conn)
                .await
                .map_err(|e| ApiError::Unexpected(Box::new(e)))?;
            Ok(Json(ApiResponse::success(
                (),
                "Audit log deleted successfully".to_string(),
                0,
            )))
        }
        None => Err(ApiError::ValidationError("Audit log not found".to_string())),
    }
}

pub async fn update_audit_log(
    State(app_ctx): State<AppContext>,
    Json(payload): Json<AuditLogRequest>,
) -> Result<Json<ApiResponse<AuditLogIdResponse>>, ApiError> {
    payload.validate().map_err(ApiError::Validation)?;

    let al = schemas::audit_log::Entity::find_by_id(payload.id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    match al {
        Some(al) => {
            let mut al_active = al.into_active_model();

            al_active.entity_type = ActiveValue::Set(payload.entity_type);
            al_active.table_name = ActiveValue::Set(payload.table_name);
            al_active.entity_id = ActiveValue::Set(payload.entity_id);
            al_active.action = ActiveValue::Set(payload.action);
            al_active.changed_by = ActiveValue::Set(payload.changed_by);
            al_active.changed_at = ActiveValue::Set(payload.changed_at);
            al_active.change_data = ActiveValue::Set(payload.change_data);

            let updated = al_active
                .save(&app_ctx.conn)
                .await
                .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

            Ok(Json(ApiResponse::success(
                AuditLogIdResponse::from(updated),
                "Audit log updated successfully".to_string(),
                0,
            )))
        }
        None => Err(ApiError::ValidationError("Audit log not found".to_string())),
    }
}
