use crate::{
    api_handlers::product::dto::product_dto::ProductResponse,
    api_utils::{api_error::ApiError, api_response::ApiResponse},
    config::config_database::config_db_context::AppContext,
};
use axum::{
    Json,
    extract::{Path, State},
};
use log::info;
use sea_orm::EntityTrait;

pub async fn get_product_by_id(
    State(app_context): State<AppContext>,
    Path(product_id): Path<i64>,
) -> Result<Json<ApiResponse<ProductResponse>>, ApiError> {
    info!("Fetching product with ID: {}", product_id);

    let product = schemas::product::Entity::find_by_id(product_id)
        .one(&app_context.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?
        .ok_or_else(|| ApiError::NotFound)?;

    match ProductResponse::try_from(product) {
        Ok(product_response) => Ok(Json(ApiResponse::success(
            product_response,
            "Product retrieved successfully".to_string(),
        ))),
        Err(_) => Err(ApiError::NotFound),
    }
}
