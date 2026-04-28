use axum::{
    Json,
    extract::{Path, Query, State},
};

use log::info;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, IntoActiveModel, ModelTrait,
    PaginatorTrait, QueryFilter, QueryOrder,
};
use std::collections::HashMap;
use validator::Validate;

use crate::{
    api_module::products::products_dto::products_dto::{
        ProductDetailResponse, ProductIdResponse, ProductRequest, ProductResponse,
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
    info!("create_product called with payload: {:?}", payload);

    payload.validate().map_err(ApiError::Validation)?;

    let product_create = schemas::products::ActiveModel::try_from(payload)
        .map_err(|e| ApiError::Unexpected(Box::new(std::io::Error::other(e))))?;

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

    // invalidate product caches
    let _ = tokio::spawn(async move {
        let _ = crate::config::config_redis::del_pattern("products:*").await;
        let _ = crate::config::config_redis::del_pattern("product:*").await;
    });

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
    info!("get_product_by_id called with id: {:?}", id);
    let cache_key = format!("product:{}", id);
    match crate::config::config_redis::get_json::<ProductDetailResponse>(&cache_key).await {
        Ok(Some(cached)) => {
            return Ok(Json(ApiResponse::success(
                cached,
                "Product retrieved successfully (cache)".to_string(),
                1,
            )));
        }
        Ok(None) => (),
        Err(e) => info!("redis get_json error: {}", e),
    }

    let product = schemas::products::Entity::find_by_id(id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    match product {
        Some(product) => {
            let dto = ProductDetailResponse::from(product.clone());
            // Serialize dto for background caching to avoid moving dto into the task
            match serde_json::to_vec(&dto) {
                Ok(bytes) => {
                    let key = cache_key.clone();
                    let _ = tokio::spawn(async move {
                        let _ = crate::config::config_redis::set_kv(&key, &bytes, 3600).await;
                    });
                }
                Err(e) => info!("failed to serialize product dto for cache: {}", e),
            }
            Ok(Json(ApiResponse::success(
                dto,
                "Product retrieved successfully".to_string(),
                1,
            )))
        }
        None => Err(ApiError::ValidationError("Product not found".to_string())),
    }
}

pub async fn get_products(
    State(app_ctx): State<AppContext>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<ApiResponse<Vec<ProductResponse>>>, ApiError> {
    info!(
        "get_products called with pagination: page={:?}, limit={:?}, total={:?}, sku={:?}, name={:?}, brand={:?}, category_id={:?}, is_sellable={:?}, track_batches={:?}, price_min={:?}, price_max={:?}",
        pagination.page,
        pagination.limit,
        pagination.total,
        pagination.sku,
        pagination.name,
        pagination.brand,
        pagination.category_id,
        pagination.is_sellable,
        pagination.track_batches,
        pagination.price_min,
        pagination.price_max
    );

    let page_index = to_page_index(pagination.page);
    let page_limit = to_page_limit(pagination.limit);

    let mut products_detail_responses: Vec<ProductResponse> = Vec::new();

    let mut select = schemas::products::Entity::find();

    if let Some(sku_filter) = pagination.sku.clone()
        && !sku_filter.is_empty()
    {
        select = select.filter(schemas::products::Column::Sku.eq(sku_filter));
    }

    if let Some(name_filter) = pagination.name.clone()
        && !name_filter.is_empty()
    {
        select = select.filter(schemas::products::Column::Name.starts_with(name_filter));
    }

    if let Some(brand_filter) = pagination.brand.clone()
        && !brand_filter.is_empty()
    {
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

    let total_items = if pagination.total > 0 {
        pagination.total
    } else {
        paginator
            .num_items()
            .await
            .map_err(|e| ApiError::Unexpected(Box::new(e)))?
    };

    let products = paginator
        .fetch_page(page_index)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;
    // Batch load related records to avoid N+1 queries
    let product_ids: Vec<i64> = products.iter().map(|p| p.id).collect();

    if !product_ids.is_empty() {
        let product_barcodes_list = schemas::product_barcodes::Entity::find()
            .filter(schemas::product_barcodes::Column::ProductId.is_in(product_ids.clone()))
            .all(&app_ctx.conn)
            .await
            .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

        let product_lots_list = schemas::product_lots::Entity::find()
            .filter(schemas::product_lots::Column::ProductId.is_in(product_ids.clone()))
            .all(&app_ctx.conn)
            .await
            .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

        let product_prices_list = schemas::product_prices::Entity::find()
            .filter(schemas::product_prices::Column::ProductId.is_in(product_ids.clone()))
            .all(&app_ctx.conn)
            .await
            .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

        let mut barcodes_map: HashMap<i64, schemas::product_barcodes::Model> = HashMap::new();
        for b in product_barcodes_list {
            barcodes_map.entry(b.product_id).or_insert(b);
        }

        let mut lots_map: HashMap<i64, schemas::product_lots::Model> = HashMap::new();
        for l in product_lots_list {
            lots_map.entry(l.product_id).or_insert(l);
        }

        let mut prices_map: HashMap<i64, schemas::product_prices::Model> = HashMap::new();
        for pr in product_prices_list {
            prices_map.entry(pr.product_id).or_insert(pr);
        }

        for product in products.iter() {
            let id_product = product.id;

            let barcode_opt = barcodes_map.get(&id_product);
            let lot_opt = lots_map.get(&id_product);
            let price_opt = prices_map.get(&id_product);

            if barcode_opt.is_none() || lot_opt.is_none() || price_opt.is_none() {
                info!("Related data not found for product ID {}", id_product);
                continue;
            }

            let product_response_detail = ProductResponse::from((
                product.clone(),
                barcode_opt.unwrap().clone(),
                lot_opt.unwrap().clone(),
                price_opt.unwrap().clone(),
            ));

            products_detail_responses.push(product_response_detail);
        }
    }

    Ok(Json(ApiResponse::success(
        products_detail_responses,
        "Products retrieved successfully".to_string(),
        total_items as i32,
    )))
}

pub async fn delete_product(
    State(app_ctx): State<AppContext>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    info!("delete_product called with id: {:?}", id);

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
    info!(
        "get_products_by_name called with pagination: page={:?}, limit={:?}, total={:?}, name={:?}",
        pagination.page, pagination.limit, pagination.total, pagination.name
    );

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

    let total_items = if pagination.total > 0 {
        pagination.total
    } else {
        paginator
            .num_items()
            .await
            .map_err(|e| ApiError::Unexpected(Box::new(e)))?
    };

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
    Path(id): Path<i64>,
    Json(payload): Json<ProductRequest>,
) -> Result<Json<ApiResponse<ProductIdResponse>>, ApiError> {
    info!(
        "update_product called with payload: {:?}, id: {:?}",
        payload, id
    );

    payload.validate().map_err(ApiError::Validation)?;

    let product = schemas::products::Entity::find_by_id(id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    match product {
        Some(product) => {
            let mut product_active_model = product.into_active_model();

            if let Some(sku) = payload.sku.clone()
                && !sku.is_empty()
            {
                product_active_model.sku = ActiveValue::Set(Some(sku));
            }
            if !payload.name.trim().is_empty() {
                product_active_model.name = ActiveValue::Set(payload.name.clone());
            }
            if let Some(description) = payload.description.clone()
                && !description.is_empty()
            {
                product_active_model.description = ActiveValue::Set(Some(description));
            }
            if let Some(brand) = payload.brand.clone()
                && !brand.is_empty()
            {
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

            // invalidate product caches for this product and lists
            let pid = id;
            let _ = tokio::spawn(async move {
                let _ = crate::config::config_redis::del_key(&format!("product:{}", pid)).await;
                let _ = crate::config::config_redis::del_pattern("products:*").await;
            });

            Ok(Json(ApiResponse::success(
                ProductIdResponse::from(updated_product),
                "Product updated successfully".to_string(),
                0,
            )))
        }
        None => Err(ApiError::ValidationError("Product not found".to_string())),
    }
}
