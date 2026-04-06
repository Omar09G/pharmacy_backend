use axum::{
    Json,
    extract::{Path, Query, State},
};

use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, IntoActiveModel, ModelTrait,
    PaginatorTrait, QueryFilter, QueryOrder,
};
use validator::Validate;

use crate::api_module::cash_journals::cash_journals_dto::cash_journals_dto::{
    CashJournalDetailResponse, CashJournalIdResponse, CashJournalRequest,
};
use crate::{
    api_utils::{
        api_error::ApiError,
        api_response::{ApiResponse, PaginationParams},
        api_utils_fun::{to_page_index, to_page_limit},
    },
    config::config_database::config_db_context::AppContext,
};

pub async fn create_cash_journal(
    State(app_ctx): State<AppContext>,
    Json(payload): Json<CashJournalRequest>,
) -> Result<Json<ApiResponse<CashJournalIdResponse>>, ApiError> {
    payload.validate().map_err(ApiError::Validation)?;

    let cj_create = schemas::cash_journals::ActiveModel::try_from(payload)
        .map_err(|e| ApiError::Unexpected(Box::new(std::io::Error::other(e))))?;

    let new_cj = cj_create
        .save(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    if new_cj.id.is_not_set() {
        return Err(ApiError::ValidationError(
            "Failed to create cash journal".to_string(),
        ));
    }

    Ok(Json(ApiResponse::success(
        CashJournalIdResponse::from(new_cj),
        "Cash journal created successfully".to_string(),
        1,
    )))
}

pub async fn get_cash_journal_by_id(
    State(app_ctx): State<AppContext>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<CashJournalDetailResponse>>, ApiError> {
    let cj = schemas::cash_journals::Entity::find_by_id(id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    match cj {
        Some(cj) => Ok(Json(ApiResponse::success(
            CashJournalDetailResponse::from(cj),
            "Cash journal retrieved successfully".to_string(),
            1,
        ))),
        None => Err(ApiError::ValidationError(
            "Cash journal not found".to_string(),
        )),
    }
}

pub async fn get_cash_journals(
    State(app_ctx): State<AppContext>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<ApiResponse<Vec<CashJournalDetailResponse>>>, ApiError> {
    let page_index = to_page_index(pagination.page);
    let page_limit = to_page_limit(pagination.limit);

    let mut select = schemas::cash_journals::Entity::find();

    if let Some(name) = pagination.name.clone()
        && !name.is_empty()
    {
        select = select.filter(schemas::cash_journals::Column::Name.eq(name));
    }

    if let Some(status) = pagination.status.clone()
        && !status.is_empty()
    {
        select = select.filter(schemas::cash_journals::Column::Status.eq(status));
    }

    if let Some(user) = pagination.user_id {
        select = select.filter(schemas::cash_journals::Column::OpenedBy.eq(user));
    }

    let paginator = select
        .order_by_asc(schemas::cash_journals::Column::Id)
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
            .map(CashJournalDetailResponse::from)
            .collect(),
        "Cash journals retrieved successfully".to_string(),
        total_items as i32,
    )))
}

pub async fn delete_cash_journal(
    State(app_ctx): State<AppContext>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    let cj = schemas::cash_journals::Entity::find_by_id(id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    match cj {
        Some(cj) => {
            cj.delete(&app_ctx.conn)
                .await
                .map_err(|e| ApiError::Unexpected(Box::new(e)))?;
            Ok(Json(ApiResponse::success(
                (),
                "Cash journal deleted successfully".to_string(),
                0,
            )))
        }
        None => Err(ApiError::ValidationError(
            "Cash journal not found".to_string(),
        )),
    }
}

pub async fn update_cash_journal(
    State(app_ctx): State<AppContext>,
    Path(id): Path<i64>,
    Json(payload): Json<CashJournalRequest>,
) -> Result<Json<ApiResponse<CashJournalIdResponse>>, ApiError> {
    payload.validate().map_err(ApiError::Validation)?;

    let cj = schemas::cash_journals::Entity::find_by_id(id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    match cj {
        Some(cj) => {
            let mut cj_active = cj.into_active_model();

            cj_active.name = ActiveValue::Set(payload.name);
            cj_active.description = ActiveValue::Set(payload.description);
            cj_active.opening_amount = ActiveValue::Set(payload.opening_amount);
            cj_active.opened_at = ActiveValue::Set(payload.opened_at);
            cj_active.closed_at = ActiveValue::Set(payload.closed_at);
            cj_active.opened_by = ActiveValue::Set(payload.opened_by);
            cj_active.closed_by = ActiveValue::Set(payload.closed_by);
            cj_active.status = ActiveValue::Set(payload.status);
            cj_active.created_at = ActiveValue::Set(payload.created_at);

            let updated = cj_active
                .save(&app_ctx.conn)
                .await
                .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

            Ok(Json(ApiResponse::success(
                CashJournalIdResponse::from(updated),
                "Cash journal updated successfully".to_string(),
                0,
            )))
        }
        None => Err(ApiError::ValidationError(
            "Cash journal not found".to_string(),
        )),
    }
}
