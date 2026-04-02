use axum::{
    Json,
    extract::{Path, Query, State},
};

use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, IntoActiveModel, ModelTrait,
    PaginatorTrait, QueryFilter, QueryOrder,
};
use validator::Validate;

use crate::api_module::product_barcodes::product_barcodes_dto::product_barcodes_dto::{
    ProductBarcodeDetailResponse, ProductBarcodeIdResponse, ProductBarcodeRequest,
};
use crate::{
    api_utils::{
        api_error::ApiError,
        api_response::{ApiResponse, PaginationParams},
        api_utils_fun::{to_page_index, to_page_limit},
    },
    config::config_database::config_db_context::AppContext,
};

pub async fn create_product_barcode(
    State(app_ctx): State<AppContext>,
    Json(payload): Json<ProductBarcodeRequest>,
) -> Result<Json<ApiResponse<ProductBarcodeIdResponse>>, ApiError> {
    payload.validate().map_err(ApiError::Validation)?;

    let pb_create = schemas::product_barcodes::ActiveModel::from(payload);

    if pb_create.barcode.is_not_set() {
        return Err(ApiError::Validation(validator::ValidationErrors::new()));
    }

    let new_pb = pb_create
        .save(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    if new_pb.id.is_not_set() {
        return Err(ApiError::ValidationError(
            "Failed to create product barcode".to_string(),
        ));
    }

    Ok(Json(ApiResponse::success(
        ProductBarcodeIdResponse::from(new_pb),
        "Product barcode created successfully".to_string(),
        1,
    )))
}

pub async fn get_product_barcode_by_id(
    State(app_ctx): State<AppContext>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<ProductBarcodeDetailResponse>>, ApiError> {
    let pb = schemas::product_barcodes::Entity::find_by_id(id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    match pb {
        Some(pb) => Ok(Json(ApiResponse::success(
            ProductBarcodeDetailResponse::from(pb),
            "Product barcode retrieved successfully".to_string(),
            1,
        ))),
        None => Err(ApiError::ValidationError(
            "Product barcode not found".to_string(),
        )),
    }
}

pub async fn get_product_barcodes(
    State(app_ctx): State<AppContext>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<ApiResponse<Vec<ProductBarcodeDetailResponse>>>, ApiError> {
    let page_index = to_page_index(pagination.page);
    let page_limit = to_page_limit(pagination.limit);

    let mut select = schemas::product_barcodes::Entity::find();

    if let Some(barcode_filter) = pagination.barcode.clone() {
        if !barcode_filter.is_empty() {
            select = select.filter(schemas::product_barcodes::Column::Barcode.eq(barcode_filter));
        }
    }

    if let Some(product_id) = pagination.product_id {
        select = select.filter(schemas::product_barcodes::Column::ProductId.eq(product_id));
    }

    let paginator = select
        .order_by_asc(schemas::product_barcodes::Column::Id)
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
            .map(ProductBarcodeDetailResponse::from)
            .collect(),
        "Product barcodes retrieved successfully".to_string(),
        total_items as i32,
    )))
}

pub async fn delete_product_barcode(
    State(app_ctx): State<AppContext>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    let pb = schemas::product_barcodes::Entity::find_by_id(id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    match pb {
        Some(pb) => {
            pb.delete(&app_ctx.conn)
                .await
                .map_err(|e| ApiError::Unexpected(Box::new(e)))?;
            Ok(Json(ApiResponse::success(
                (),
                "Product barcode deleted successfully".to_string(),
                0,
            )))
        }
        None => Err(ApiError::ValidationError(
            "Product barcode not found".to_string(),
        )),
    }
}

pub async fn get_product_barcodes_by_barcode(
    State(app_ctx): State<AppContext>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<ApiResponse<Vec<ProductBarcodeDetailResponse>>>, ApiError> {
    let page_index = to_page_index(pagination.page);
    let page_limit = to_page_limit(pagination.limit);
    let barcode_filter = pagination.barcode.clone().unwrap_or_default();

    if barcode_filter.is_empty() {
        return Err(ApiError::ValidationError(
            "Barcode filter cannot be empty".to_string(),
        ));
    }

    let paginator = schemas::product_barcodes::Entity::find()
        .filter(schemas::product_barcodes::Column::Barcode.eq(barcode_filter))
        .order_by_asc(schemas::product_barcodes::Column::Id)
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
            .map(ProductBarcodeDetailResponse::from)
            .collect(),
        "Product barcodes retrieved successfully".to_string(),
        total_items as i32,
    )))
}

pub async fn update_product_barcode(
    State(app_ctx): State<AppContext>,
    Json(payload): Json<ProductBarcodeRequest>,
) -> Result<Json<ApiResponse<ProductBarcodeIdResponse>>, ApiError> {
    payload.validate().map_err(ApiError::Validation)?;

    let pb = schemas::product_barcodes::Entity::find_by_id(payload.id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    match pb {
        Some(pb) => {
            let mut pb_active_model = pb.into_active_model();

            pb_active_model.product_id = ActiveValue::Set(payload.product_id);
            pb_active_model.barcode = ActiveValue::Set(payload.barcode);
            pb_active_model.barcode_type = ActiveValue::Set(payload.barcode_type);

            let updated_pb = pb_active_model
                .save(&app_ctx.conn)
                .await
                .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

            Ok(Json(ApiResponse::success(
                ProductBarcodeIdResponse::from(updated_pb),
                "Product barcode updated successfully".to_string(),
                0,
            )))
        }
        None => Err(ApiError::ValidationError(
            "Product barcode not found".to_string(),
        )),
    }
}
