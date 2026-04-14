use axum::{
    Json,
    extract::{Query, State},
};
use sea_orm::{
    ConnectionTrait, DatabaseBackend, EntityTrait, PaginatorTrait, QueryOrder, Statement,
};

use crate::{
    api_module::vw_sales_daily_summary::vw_sales_daily_summary_dto::VwSalesDailySummaryResponse,
    api_utils::{
        api_error::ApiError,
        api_response::{ApiResponse, PaginationParams},
        api_utils_fun::{parse_mexico_date_range_to_utc, to_page_index, to_page_limit},
    },
    config::config_database::config_db_context::AppContext,
};

pub async fn get_vw_sales_daily_summary(
    State(app_ctx): State<AppContext>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<ApiResponse<Vec<VwSalesDailySummaryResponse>>>, ApiError> {
    let page_index = to_page_index(pagination.page);
    let page_limit = to_page_limit(pagination.limit);

    // Parse date filters (Mexico → UTC)
    let (date_start, date_end) = match (
        pagination.date_init.as_deref(),
        pagination.date_end.as_deref(),
    ) {
        (Some(di), Some(de)) if !di.is_empty() && !de.is_empty() => {
            parse_mexico_date_range_to_utc(di, de)?
        }
        _ => (None, None),
    };

    // Call fn_t_sales_daily_summary(p_start, p_end, p_customer_id, p_user_id, p_status)
    let stmt = Statement::from_sql_and_values(
        DatabaseBackend::Postgres,
        "SELECT pharmacy.fn_t_sales_daily_summary(NULL, NULL, 0, 0, NULL)",
        [
            date_start.into(),
            date_end.into(),
            pagination.customer_id.unwrap_or(0).into(),
            pagination.user_id.unwrap_or(0).into(),
            pagination
                .status
                .clone()
                .map(|s| if s.is_empty() { None } else { Some(s) })
                .unwrap_or(None)
                .into(),
        ],
    );
    app_ctx
        .conn
        .execute(stmt)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    let select = schemas::vw_t_sales_daily_summary::Entity::find();

    let paginator = select
        .order_by_desc(schemas::vw_t_sales_daily_summary::Column::Day)
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
            .map(VwSalesDailySummaryResponse::from)
            .collect(),
        "Sales daily summary retrieved successfully".to_string(),
        total_items as i32,
    )))
}
