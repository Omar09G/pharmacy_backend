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

pub async fn get_vw_customer_invoice_aging(
    State(app_ctx): State<AppContext>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<ApiResponse<Vec<VwCustomerInvoiceAgingResponse>>>, ApiError> {
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

    if let Some(ref name) = pagination.name {
        if !name.is_empty() {
            select = select
                .filter(schemas::vw_t_customer_invoice_aging::Column::CustomerName.contains(name));
        }
    }

    if let Some(ref status) = pagination.status {
        if !status.is_empty() {
            select = select.filter(
                schemas::vw_t_customer_invoice_aging::Column::InvoiceStatus.eq(status.clone()),
            );
        }
    }

    if let Some(ref invoice_no) = pagination.invoice_no {
        if !invoice_no.is_empty() {
            select = select.filter(
                schemas::vw_t_customer_invoice_aging::Column::InvoiceNo.eq(invoice_no.clone()),
            );
        }
    }

    let paginator = select
        .order_by_desc(schemas::vw_t_customer_invoice_aging::Column::DaysOverdue)
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
            .map(VwCustomerInvoiceAgingResponse::from)
            .collect(),
        "Customer invoice aging retrieved successfully".to_string(),
        total_items as i32,
    )))
}
