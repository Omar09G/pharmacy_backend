use axum::{
    Json,
    extract::{Query, State},
};
use sea_orm::{
    ConnectionTrait, DatabaseBackend, EntityTrait, PaginatorTrait, QueryOrder, Statement,
};

use crate::{
    api_module::vw_cash_journal_balance::vw_cash_journal_balance_dto::VwCashJournalBalanceResponse,
    api_utils::{
        api_error::ApiError,
        api_response::{ApiResponse, PaginationParams},
        api_utils_fun::{parse_local_date_range_to_utc, to_page_index, to_page_limit},
    },
    config::config_database::config_db_context::AppContext,
};
use log::info;

pub async fn get_vw_cash_journal_balance(
    State(app_ctx): State<AppContext>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<ApiResponse<Vec<VwCashJournalBalanceResponse>>>, ApiError> {
    info!(
        "get_vw_cash_journal_balance called with pagination: page={:?}, limit={:?}, total={:?}, id={:?}, date_init={:?}, date_end={:?}",
        pagination.page, pagination.limit, pagination.total, pagination.id, pagination.date_init, pagination.date_end
    );

    let page_index = to_page_index(pagination.page);
    let page_limit = to_page_limit(pagination.limit);

    // Parse date filters (Mexico → UTC)
    let (date_start, date_end) = match (
        pagination.date_init.as_deref(),
        pagination.date_end.as_deref(),
    ) {
        (Some(di), Some(de)) if !di.is_empty() && !de.is_empty() => {
            parse_local_date_range_to_utc(di, de)?
        }
        _ => (None, None),
    };

    // Call fn_t_cash_journal_balance(p_cash_journal_id, p_start, p_end)
    let stmt = Statement::from_sql_and_values(
        DatabaseBackend::Postgres,
        "SELECT pharmacy.fn_t_cash_journal_balance($1, $2, $3)",
        [
            pagination.id.unwrap_or(0).into(),
            date_start.into(),
            date_end.into(),
        ],
    );
    app_ctx
        .conn
        .execute(stmt)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    let select = schemas::vw_t_cash_journal_balance::Entity::find();

    let paginator = select
        .order_by_desc(schemas::vw_t_cash_journal_balance::Column::CashJournalId)
        .paginate(&app_ctx.conn, page_limit);

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
            .map(VwCashJournalBalanceResponse::from)
            .collect(),
        "Cash journal balance retrieved successfully".to_string(),
        total_items as i32,
    )))
}
