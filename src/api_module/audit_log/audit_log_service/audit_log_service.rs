use axum::{
    Json,
    extract::{Path, Query, State},
};

use log::info;
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
    info!("create_audit_log called with payload: {:?}", payload);

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
    info!("get_audit_log_by_id called with id: {:?}", id);

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
    info!("get_audit_logs called with pagination: {:?}", pagination);

    let page_index = to_page_index(pagination.page);
    let page_limit = to_page_limit(pagination.limit);

    info!(
        "Fetching audit logs with pagination: page {}, limit {}",
        pagination.page, pagination.limit
    );

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

    // If client provided a total (>0) use it; otherwise query the paginator for the count.
    let total_items = if pagination.total > 0 {
        pagination.total
    } else {
        paginator
            .num_items()
            .await
            .map_err(|e| ApiError::Unexpected(Box::new(e)))?
    };

    let items = paginator
        .fetch_page(page_index)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    if items.is_empty() {
        return Ok(Json(ApiResponse::success(
            Vec::new(),
            "No audit logs found".to_string(),
            0,
        )));
    }

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
    info!("delete_audit_log called with id: {:?}", id);

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
    Path(id): Path<i64>,
    Json(payload): Json<AuditLogRequest>,
) -> Result<Json<ApiResponse<AuditLogIdResponse>>, ApiError> {
    info!("update_audit_log called with payload: {:?}, id: {:?}", payload, id);

    payload.validate().map_err(ApiError::Validation)?;

    let al = schemas::audit_log::Entity::find_by_id(id)
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
