use axum::{
    Json,
    extract::{Query, State},
};
use sea_orm::{
    ColumnTrait, ConnectionTrait, DatabaseBackend, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, Statement,
};

use crate::{
    api_module::vw_customer_invoice_aging::vw_customer_invoice_aging_dto::VwCustomerInvoiceAgingResponse,
    api_utils::{
        api_error::ApiError,
        api_response::{ApiResponse, PaginationParams},
        api_utils_fun::{to_page_index, to_page_limit},
    },
    config::config_database::config_db_context::AppContext,
};
use log::info;

pub async fn get_vw_customer_invoice_aging(
    State(app_ctx): State<AppContext>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<ApiResponse<Vec<VwCustomerInvoiceAgingResponse>>>, ApiError> {
    info!(
        "get_vw_customer_invoice_aging called with pagination: page={:?}, limit={:?}, total={:?}, customer_id={:?}, name={:?}, status={:?}, invoice_no={:?}",
        pagination.page,
        pagination.limit,
        pagination.total,
        pagination.customer_id,
        pagination.name,
        pagination.status,
        pagination.invoice_no
    );

    let page_index = to_page_index(pagination.page);
    let page_limit = to_page_limit(pagination.limit);

    // Call fn_t_customer_invoice_aging(p_customer_id, p_as_of)
    // p_as_of defaults to NULL (current date in the function)
    let stmt = Statement::from_sql_and_values(
        DatabaseBackend::Postgres,
        "SELECT pharmacy.fn_t_customer_invoice_aging(0, NULL)",
        [
            pagination.customer_id.unwrap_or(0).into(),
            Option::<String>::None.into(),
        ],
    );
    app_ctx
        .conn
        .execute(stmt)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    let mut select = schemas::vw_t_customer_invoice_aging::Entity::find();

    if let Some(customer_id) = pagination.customer_id {
        select =
            select.filter(schemas::vw_t_customer_invoice_aging::Column::CustomerId.eq(customer_id));
    }

    if let Some(ref name) = pagination.name
        && !name.is_empty()
    {
        select = select
            .filter(schemas::vw_t_customer_invoice_aging::Column::CustomerName.contains(name));
    }

    if let Some(ref status) = pagination.status
        && !status.is_empty()
    {
        select = select
            .filter(schemas::vw_t_customer_invoice_aging::Column::InvoiceStatus.eq(status.clone()));
    }

    if let Some(ref invoice_no) = pagination.invoice_no
        && !invoice_no.is_empty()
    {
        select = select
            .filter(schemas::vw_t_customer_invoice_aging::Column::InvoiceNo.eq(invoice_no.clone()));
    }

    let fetch_limit = page_limit + 1;
    let paginator = select
        .order_by_desc(schemas::vw_t_customer_invoice_aging::Column::DaysOverdue)
        .paginate(&app_ctx.conn, fetch_limit);

    let items_raw = paginator
        .fetch_page(page_index)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    let has_more = items_raw.len() as u64 > page_limit;
    let total_items = if pagination.total > 0 {
        pagination.total
    } else if has_more {
        page_index * page_limit + page_limit + 1
    } else {
        page_index * page_limit + items_raw.len() as u64
    };

    Ok(Json(ApiResponse::success(
        items_raw
            .into_iter()
            .take(page_limit as usize)
            .map(VwCustomerInvoiceAgingResponse::from)
            .collect(),
        "Customer invoice aging retrieved successfully".to_string(),
        total_items as i32,
    )))
}
