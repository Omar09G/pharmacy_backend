use axum::{
    Json,
    extract::{Path, Query, State},
};

use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, IntoActiveModel, ModelTrait,
    PaginatorTrait, QueryFilter, QueryOrder,
};
use validator::Validate;

use crate::api_module::cash_entries::cash_entries_dto::cash_entries_dto::{
    CashEntryDetailResponse, CashEntryIdResponse, CashEntryRequest,
};
use crate::{
    api_utils::{
        api_error::ApiError,
        api_response::{ApiResponse, PaginationParams},
        api_utils_fun::{to_page_index, to_page_limit},
    },
    config::config_database::config_db_context::AppContext,
};
use log::info;

pub async fn create_cash_entry(
    State(app_ctx): State<AppContext>,
    Json(payload): Json<CashEntryRequest>,
) -> Result<Json<ApiResponse<CashEntryIdResponse>>, ApiError> {
    info!("create_cash_entry called with payload: {:?}", payload);

    payload.validate().map_err(ApiError::Validation)?;

    let ce_create = schemas::cash_entries::ActiveModel::try_from(payload)
        .map_err(|e| ApiError::Unexpected(Box::new(std::io::Error::other(e))))?;

    let new_ce = ce_create
        .save(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    if new_ce.id.is_not_set() {
        return Err(ApiError::ValidationError(
            "Failed to create cash entry".to_string(),
        ));
    }

    Ok(Json(ApiResponse::success(
        CashEntryIdResponse::from(new_ce),
        "Cash entry created successfully".to_string(),
        1,
    )))
}

pub async fn get_cash_entry_by_id(
    State(app_ctx): State<AppContext>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<CashEntryDetailResponse>>, ApiError> {
    info!("get_cash_entry_by_id called with id: {:?}", id);

    let ce = schemas::cash_entries::Entity::find_by_id(id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    match ce {
        Some(ce) => Ok(Json(ApiResponse::success(
            CashEntryDetailResponse::from(ce),
            "Cash entry retrieved successfully".to_string(),
            1,
        ))),
        None => Err(ApiError::ValidationError(
            "Cash entry not found".to_string(),
        )),
    }
}

pub async fn get_cash_entries(
    State(app_ctx): State<AppContext>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<ApiResponse<Vec<CashEntryDetailResponse>>>, ApiError> {
    info!(
        "get_cash_entries called with pagination: name={:?}, reference_type={:?}, method_id={:?}, reference={:?}, reference_id={:?}, recorded_by={:?}",
        pagination.name,
        pagination.reference_type,
        pagination.method_id,
        pagination.reference,
        pagination.reference_id,
        pagination.recorded_by
    );

    let page_index = to_page_index(pagination.page);
    let page_limit = to_page_limit(pagination.limit);

    let mut select = schemas::cash_entries::Entity::find();

    if let Some(name) = pagination.name.clone()
        && !name.is_empty()
    {
        select = select.filter(schemas::cash_entries::Column::Name.eq(name));
    }

    if let Some(entry_type) = pagination.reference_type.clone()
        && !entry_type.is_empty()
    {
        select = select.filter(schemas::cash_entries::Column::EntryType.eq(entry_type));
    }

    if let Some(method) = pagination.method_id {
        select = select.filter(schemas::cash_entries::Column::MethodId.eq(method));
    }

    if let Some(rel_type) = pagination.reference.clone()
        && !rel_type.is_empty()
    {
        select = select.filter(schemas::cash_entries::Column::RelatedType.eq(rel_type));
    }

    if let Some(rel_id) = pagination.reference_id {
        select = select.filter(schemas::cash_entries::Column::RelatedId.eq(rel_id));
    }

    if let Some(user) = pagination.recorded_by {
        // recorded_by isn't in PaginationParams currently named recorded_by; fall back to user_id
        select = select.filter(schemas::cash_entries::Column::RecordedBy.eq(user));
    }

    let paginator = select
        .order_by_asc(schemas::cash_entries::Column::Id)
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

    Ok(Json(ApiResponse::success(
        items
            .into_iter()
            .map(CashEntryDetailResponse::from)
            .collect(),
        "Cash entries retrieved successfully".to_string(),
        total_items as i32,
    )))
}

pub async fn delete_cash_entry(
    State(app_ctx): State<AppContext>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    info!("delete_cash_entry called with id: {:?}", id);

    let ce = schemas::cash_entries::Entity::find_by_id(id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    match ce {
        Some(ce) => {
            ce.delete(&app_ctx.conn)
                .await
                .map_err(|e| ApiError::Unexpected(Box::new(e)))?;
            Ok(Json(ApiResponse::success(
                (),
                "Cash entry deleted successfully".to_string(),
                0,
            )))
        }
        None => Err(ApiError::ValidationError(
            "Cash entry not found".to_string(),
        )),
    }
}

pub async fn update_cash_entry(
    State(app_ctx): State<AppContext>,
    Path(id): Path<i64>,
    Json(payload): Json<CashEntryRequest>,
) -> Result<Json<ApiResponse<CashEntryIdResponse>>, ApiError> {
    info!(
        "update_cash_entry called with payload: {:?}, id: {:?}",
        payload, id
    );

    payload.validate().map_err(ApiError::Validation)?;

    let ce = schemas::cash_entries::Entity::find_by_id(id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    match ce {
        Some(ce) => {
            let mut ce_active = ce.into_active_model();

            ce_active.name = ActiveValue::Set(payload.name);
            ce_active.entry_type = ActiveValue::Set(payload.entry_type);
            ce_active.amount = ActiveValue::Set(payload.amount);
            ce_active.method_id = ActiveValue::Set(payload.method_id);
            ce_active.related_type = ActiveValue::Set(payload.related_type);
            ce_active.related_id = ActiveValue::Set(payload.related_id);
            ce_active.description = ActiveValue::Set(payload.description);
            ce_active.recorded_at = ActiveValue::Set(payload.recorded_at);
            ce_active.recorded_by = ActiveValue::Set(payload.recorded_by);

            let updated = ce_active
                .save(&app_ctx.conn)
                .await
                .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

            Ok(Json(ApiResponse::success(
                CashEntryIdResponse::from(updated),
                "Cash entry updated successfully".to_string(),
                0,
            )))
        }
        None => Err(ApiError::ValidationError(
            "Cash entry not found".to_string(),
        )),
    }
}
