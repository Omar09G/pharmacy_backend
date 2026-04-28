use axum::{
    Json,
    extract::{Query, State},
};
use sea_orm::{
    ConnectionTrait, DatabaseBackend, EntityTrait, PaginatorTrait, QueryOrder, Statement,
};

use crate::{
    api_module::vw_inventory_stock::vw_inventory_stock_dto::VwInventoryStockResponse,
    api_utils::{
        api_error::ApiError,
        api_response::{ApiResponse, PaginationParams},
        api_utils_fun::{to_page_index, to_page_limit},
    },
    config::config_database::config_db_context::AppContext,
};
use log::info;

pub async fn get_vw_inventory_stock(
    State(app_ctx): State<AppContext>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<ApiResponse<Vec<VwInventoryStockResponse>>>, ApiError> {
    info!(
        "get_vw_inventory_stock called with pagination: page={:?}, limit={:?}, total={:?}, product_id={:?}",
        pagination.page, pagination.limit, pagination.total, pagination.product_id
    );

    let page_index = to_page_index(pagination.page);
    let page_limit = to_page_limit(pagination.limit);

    let cache_key = format!(
        "vw_inventory_stock:product:{}:page:{}:limit:{}",
        pagination.product_id.unwrap_or(0),
        page_index,
        page_limit
    );
    match crate::config::config_redis::get_json::<Vec<VwInventoryStockResponse>>(&cache_key).await {
        Ok(Some(cached)) => {
            let total = cached.len() as i32;
            return Ok(Json(ApiResponse::success(
                cached,
                "Inventory stock retrieved successfully (cache)".to_string(),
                total,
            )));
        }
        Ok(None) => (),
        Err(e) => info!("redis get_json error: {}", e),
    }

    // Call fn_t_inventory_stock(p_product_id)
    let stmt = Statement::from_sql_and_values(
        DatabaseBackend::Postgres,
        "SELECT pharmacy.fn_t_inventory_stock(0)",
        [pagination.product_id.unwrap_or(0).into()],
    );
    app_ctx
        .conn
        .execute(stmt)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    let select = schemas::vw_t_inventory_stock::Entity::find();

    let paginator = select
        .order_by_asc(schemas::vw_t_inventory_stock::Column::ProductName)
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

    let dto_items: Vec<VwInventoryStockResponse> = items
        .into_iter()
        .map(VwInventoryStockResponse::from)
        .collect();

    // cache inventory view short-term (serialize and move bytes into task)
    match serde_json::to_vec(&dto_items) {
        Ok(bytes) => {
            let key = cache_key.clone();
            let _ = tokio::spawn(async move {
                let _ = crate::config::config_redis::set_kv(&key, &bytes, 30).await;
            });
        }
        Err(e) => info!("failed to serialize inventory dto for cache: {}", e),
    }

    Ok(Json(ApiResponse::success(
        dto_items,
        "Inventory stock retrieved successfully".to_string(),
        total_items as i32,
    )))
}
