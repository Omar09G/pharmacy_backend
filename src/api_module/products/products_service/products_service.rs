use axum::{
    Json,
    extract::{Path, Query, State},
};

use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, IntoActiveModel, ModelTrait,
    PaginatorTrait, QueryFilter, QueryOrder,
};
use validator::Validate;

use crate::{
    api_module::products::products_dto::products_dto::{
        ProductDetailResponse, ProductIdResponse, ProductRequest,
    },
    api_utils::api_utils_fun::get_current_timestamp_now,
};
use crate::{
    api_utils::{
        api_error::ApiError,
        api_response::{ApiResponse, PaginationParams},
        api_utils_fun::{to_page_index, to_page_limit},
    },
    config::config_database::config_db_context::AppContext,
};

pub async fn create_product(
    State(app_ctx): State<AppContext>,
    Json(payload): Json<ProductRequest>,
) -> Result<Json<ApiResponse<ProductIdResponse>>, ApiError> {
    payload.validate().map_err(ApiError::Validation)?;

    let product_create = schemas::products::ActiveModel::from(payload);

    if product_create.name.is_not_set() {
        return Err(ApiError::Validation(validator::ValidationErrors::new()));
    }

    let new_product = product_create
        .save(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    if new_product.id.is_not_set() {
        return Err(ApiError::ValidationError(
            "Failed to create product".to_string(),
        ));
    }

    Ok(Json(ApiResponse::success(
        ProductIdResponse::from(new_product),
        "Product created successfully".to_string(),
        1,
    )))
}

pub async fn get_product_by_id(
    State(app_ctx): State<AppContext>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<ProductDetailResponse>>, ApiError> {
    let product = schemas::products::Entity::find_by_id(id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    match product {
        Some(product) => Ok(Json(ApiResponse::success(
            ProductDetailResponse::from(product),
            "Product retrieved successfully".to_string(),
            1,
        ))),
        None => Err(ApiError::ValidationError("Product not found".to_string())),
    }
}

pub async fn get_products(
    State(app_ctx): State<AppContext>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<ApiResponse<Vec<ProductDetailResponse>>>, ApiError> {
    let page_index = to_page_index(pagination.page);
    let page_limit = to_page_limit(pagination.limit);

    let mut select = schemas::products::Entity::find();

    if let Some(sku_filter) = pagination.sku.clone()
        && !sku_filter.is_empty() {
            select = select.filter(schemas::products::Column::Sku.eq(sku_filter));
        }

    if let Some(name_filter) = pagination.name.clone()
        && !name_filter.is_empty() {
            select = select.filter(schemas::products::Column::Name.eq(name_filter));
        }

    if let Some(brand_filter) = pagination.brand.clone()
        && !brand_filter.is_empty() {
            select = select.filter(schemas::products::Column::Brand.eq(brand_filter));
        }

    if let Some(category) = pagination.category_id {
        select = select.filter(schemas::products::Column::CategoryId.eq(category));
    }

    if let Some(is_sellable) = pagination.is_sellable {
        select = select.filter(schemas::products::Column::IsSellable.eq(is_sellable));
    }

    if let Some(track_batches) = pagination.track_batches {
        select = select.filter(schemas::products::Column::TrackBatches.eq(track_batches));
    }

    // price range filtering using price_min/price_max
    if let Some(min) = pagination.price_min {
        select = select.filter(schemas::products::Column::DefaultPrice.gte(min));
    }
    if let Some(max) = pagination.price_max {
        select = select.filter(schemas::products::Column::DefaultPrice.lte(max));
    }

    let paginator = select
        .order_by_asc(schemas::products::Column::Id)
        .paginate(&app_ctx.conn, page_limit);

    let total_items = paginator
        .num_items()
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    let products = paginator
        .fetch_page(page_index)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    Ok(Json(ApiResponse::success(
        products
            .into_iter()
            .map(ProductDetailResponse::from)
            .collect(),
        "Products retrieved successfully".to_string(),
        total_items as i32,
    )))
}

pub async fn delete_product(
    State(app_ctx): State<AppContext>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    let product = schemas::products::Entity::find_by_id(id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    match product {
        Some(product) => {
            product
                .delete(&app_ctx.conn)
                .await
                .map_err(|e| ApiError::Unexpected(Box::new(e)))?;
            Ok(Json(ApiResponse::success(
                (),
                "Product deleted successfully".to_string(),
                0,
            )))
        }
        None => Err(ApiError::ValidationError("Product not found".to_string())),
    }
}

pub async fn get_products_by_name(
    State(app_ctx): State<AppContext>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<ApiResponse<Vec<ProductDetailResponse>>>, ApiError> {
    let page_index = to_page_index(pagination.page);
    let page_limit = to_page_limit(pagination.limit);
    let name_filter = pagination.name.clone().unwrap_or_default();

    if name_filter.is_empty() {
        return Err(ApiError::ValidationError(
            "Name filter cannot be empty".to_string(),
        ));
    }

    let paginator = schemas::products::Entity::find()
        .filter(schemas::products::Column::Name.eq(name_filter))
        .order_by_asc(schemas::products::Column::Id)
        .paginate(&app_ctx.conn, page_limit);

    let total_items = paginator
        .num_items()
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    let products = paginator
        .fetch_page(page_index)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    Ok(Json(ApiResponse::success(
        products
            .into_iter()
            .map(ProductDetailResponse::from)
            .collect(),
        "Products retrieved successfully".to_string(),
        total_items as i32,
    )))
}

pub async fn update_product(
    State(app_ctx): State<AppContext>,
    Json(payload): Json<ProductRequest>,
) -> Result<Json<ApiResponse<ProductIdResponse>>, ApiError> {
    payload.validate().map_err(ApiError::Validation)?;

    let product = schemas::products::Entity::find_by_id(payload.id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    match product {
        Some(product) => {
            let mut product_active_model = product.into_active_model();

            if let Some(sku) = payload.sku.clone()
                && !sku.is_empty() {
                    product_active_model.sku = ActiveValue::Set(Some(sku));
                }
            if !payload.name.trim().is_empty() {
                product_active_model.name = ActiveValue::Set(payload.name.clone());
            }
            if let Some(description) = payload.description.clone()
                && !description.is_empty() {
                    product_active_model.description = ActiveValue::Set(Some(description));
                }
            if let Some(brand) = payload.brand.clone()
                && !brand.is_empty() {
                    product_active_model.brand = ActiveValue::Set(Some(brand));
                }
            if let Some(category_id) = payload.category_id {
                product_active_model.category_id = ActiveValue::Set(Some(category_id));
            }
            if let Some(unit_id) = payload.unit_id {
                product_active_model.unit_id = ActiveValue::Set(Some(unit_id));
            }

            product_active_model.is_sellable = ActiveValue::Set(payload.is_sellable);

            product_active_model.track_batches = ActiveValue::Set(payload.track_batches);

            if let Some(tax_profile_id) = payload.tax_profile_id {
                product_active_model.tax_profile_id = ActiveValue::Set(Some(tax_profile_id));
            }
            if let Some(default_cost) = payload.default_cost {
                product_active_model.default_cost = ActiveValue::Set(Some(default_cost));
            }
            if let Some(purchase_price) = payload.purchase_price {
                product_active_model.purchase_price = ActiveValue::Set(Some(purchase_price));
            }
            if let Some(wholesale_price) = payload.wholesale_price {
                product_active_model.wholesale_price = ActiveValue::Set(Some(wholesale_price));
            }
            if let Some(sale_price) = payload.sale_price {
                product_active_model.sale_price = ActiveValue::Set(Some(sale_price));
            }
            if let Some(default_price) = payload.default_price {
                product_active_model.default_price = ActiveValue::Set(Some(default_price));
            }

            product_active_model.updated_at = ActiveValue::Set(Some(get_current_timestamp_now()));

            let updated_product = product_active_model
                .save(&app_ctx.conn)
                .await
                .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

            Ok(Json(ApiResponse::success(
                ProductIdResponse::from(updated_product),
                "Product updated successfully".to_string(),
                0,
            )))
        }
        None => Err(ApiError::ValidationError("Product not found".to_string())),
    }
}
