use axum::{
    Json,
    extract::{Path, Query, State},
};

use log::info;
use sea_orm::entity::prelude::*;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, LoaderTrait, PaginatorTrait, QueryFilter,
    QueryOrder, TransactionTrait,
};
use validator::Validate;

use crate::{
    api_module::{
        add_product::add_product_dto::add_product_dto::ProductAddResponseDetail,
        products::products_dto::products_dto::{
            ProductBarcodeRequestAdd, ProductIdResponse, ProductLotRequestAdd,
            ProductPriceRequestAdd, ProductRequestDetail,
        },
    },
    api_utils::{
        api_error::ApiError,
        api_response::{ApiResponse, PaginationParams},
        api_utils_fun::{to_page_index, to_page_limit},
    },
    config::config_database::config_db_context::AppContext,
};

pub async fn add_product(
    State(app_ctx): State<AppContext>,
    Json(payload): Json<ProductRequestDetail>,
) -> Result<Json<ApiResponse<ProductIdResponse>>, ApiError> {
    info!("add_product called with payload: {:?}", payload);

    //Valida el payload
    payload.validate().map_err(ApiError::Validation)?;

    // Ejecutar creación en una transacción: guardar producto, barcode, price y lot
    let txn = app_ctx
        .conn
        .begin()
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    // Encapsular la lógica en un bloque para capturar errores y poder hacer rollback en caso de fallo
    let op_result = (async {
        // Asigna a ActiveModel (usar clone para no mover `payload`)
        let new_product_active_model = schemas::products::ActiveModel::try_from(payload.clone())
            .map_err(|e| ApiError::Unexpected(Box::new(std::io::Error::other(e))))?;

        // Se guarda el producto para obtener el ID generado (usar la transacción)
        let new_product = new_product_active_model
            .save(&txn)
            .await
            .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

        if new_product.id.is_not_set() {
            return Err(ApiError::ValidationError(
                "Failed to create product".to_string(),
            ));
        }

        // clonar para poder extraer el id sin mover el objeto original que se devolverá
        let new_product_response = new_product.clone();
        let product_id = new_product.id.clone().unwrap();

        // Crear requests derivados usando la referencia al payload
        let new_product_barcode_active_model = ProductBarcodeRequestAdd::into_active_model(
            payload.barcodes_detail.clone(),
            product_id,
        );
        let new_product_price_active_model =
            ProductPriceRequestAdd::into_active_model(payload.prices_detail.clone(), product_id);
        let new_product_lot_active_model =
            ProductLotRequestAdd::into_active_model(payload.lots_detail.clone(), product_id);

        // Se guardan el barcode, precio y lote
        let new_pb = new_product_barcode_active_model
            .save(&txn)
            .await
            .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

        if new_pb.id.is_not_set() {
            return Err(ApiError::ValidationError(
                "Failed to create product barcode".to_string(),
            ));
        }

        let new_pp = new_product_price_active_model
            .save(&txn)
            .await
            .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

        if new_pp.id.is_not_set() {
            return Err(ApiError::ValidationError(
                "Failed to create product price".to_string(),
            ));
        }

        let new_pl = new_product_lot_active_model
            .save(&txn)
            .await
            .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

        if new_pl.id.is_not_set() {
            return Err(ApiError::ValidationError(
                "Failed to create product lot".to_string(),
            ));
        }

        Ok(new_product_response)
    })
    .await;

    // Manejar commit/rollback según resultado y devolver respuesta
    return match op_result {
        Ok(saved_product) => {
            txn.commit()
                .await
                .map_err(|e| ApiError::Unexpected(Box::new(e)))?;
            Ok(Json(ApiResponse::success(
                ProductIdResponse::from(saved_product),
                "Product created successfully".to_string(),
                1,
            )))
        }
        Err(err) => {
            // intentar rollback y propagar el error original
            let _ = txn.rollback().await;
            Err(err)
        }
    };
}

pub async fn get_product_by_bar_code(
    State(app_ctx): State<AppContext>,
    Path(barcode): Path<String>,
) -> Result<Json<ApiResponse<ProductAddResponseDetail>>, ApiError> {
    info!("get_product_by_bar_code called with barcode: {:?}", barcode);

    if barcode.trim().is_empty() {
        return Err(ApiError::ValidationError(
            "Barcode cannot be empty".to_string(),
        ));
    }

    // Buscar el registro de barcode
    let pb = schemas::product_barcodes::Entity::find()
        .filter(schemas::product_barcodes::Column::Barcode.eq(barcode.clone()))
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    let pb = match pb {
        Some(p) => p,
        None => {
            return Err(ApiError::ValidationError(
                "Product barcode not found".to_string(),
            ));
        }
    };

    // Obtener producto
    let product = schemas::products::Entity::find_by_id(pb.product_id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    let product = match product {
        Some(p) => p,
        None => return Err(ApiError::ValidationError("Product not found".to_string())),
    };

    // Sumar qty_on_hand de lots
    let lots = schemas::product_lots::Entity::find()
        .filter(schemas::product_lots::Column::ProductId.eq(product.id))
        .all(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    let mut total_qty = Decimal::new(0, 0);
    for lot in &lots {
        total_qty = total_qty + lot.qty_on_hand;
    }

    // Obtener precio vigente (el más reciente por starts_at/created_at)
    let price_rec = schemas::product_prices::Entity::find()
        .filter(schemas::product_prices::Column::ProductId.eq(product.id))
        .order_by_desc(schemas::product_prices::Column::StartsAt)
        .order_by_desc(schemas::product_prices::Column::CreatedAt)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    let price_value = match price_rec {
        Some(pr) => pr.price,
        None => product.sale_price.unwrap_or(Decimal::new(0, 0)),
    };

    let response = ProductAddResponseDetail {
        id: product.id,
        sku: product.sku,
        name: product.name,
        barcode: pb.barcode,
        description: product.description,
        qty_on_hand: total_qty,
        price: price_value,
        tax_profile_id: product.tax_profile_id,
        purchase_price: product.purchase_price,
        wholesale_price: product.wholesale_price,
        sale_price: product.sale_price,
        default_price: product.default_price,
    };

    Ok(Json(ApiResponse::success(
        response,
        "Product retrieved successfully".to_string(),
        1,
    )))
}

///Obtener una lista de productos con su información básica por paginación y filtros opcionales (sku, name) y con la opción de incluir su barcode, precio vigente y qty_on_hand sumando los lotes
pub async fn get_products_with_details(
    State(app_ctx): State<AppContext>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<ApiResponse<Vec<ProductAddResponseDetail>>>, ApiError> {
    info!(
        "get_products_with_details called with pagination: sku_filter={:?}, name_filter={:?}",
        pagination.sku, pagination.name
    );

    let page_index = to_page_index(pagination.page);
    let page_limit = to_page_limit(pagination.limit);

    let mut select = schemas::products::Entity::find();
    if let Some(sku_filter) = pagination.sku.clone()
        && !sku_filter.is_empty()
    {
        select = select.filter(schemas::products::Column::Sku.eq(sku_filter));
    }
    if let Some(name_filter) = pagination.name.clone()
        && !name_filter.is_empty()
    {
        select = select.filter(schemas::products::Column::Name.eq(name_filter));
    }
    // Construir paginador y obtener total
    let paginator = select
        .order_by_asc(schemas::products::Column::Id)
        .paginate(&app_ctx.conn, page_limit);

    // If client provided a total (>0) use it; otherwise query the paginator for the count.
    let total_items = if pagination.total > 0 {
        info!(
            "Using client-provided total items for pagination: {}",
            pagination.total
        );
        pagination.total
    } else {
        info!("Counting total items for pagination...");
        paginator
            .num_items()
            .await
            .map_err(|e| ApiError::Unexpected(Box::new(e)))?
    };

    let products = paginator
        .fetch_page(page_index)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    let mut results: Vec<ProductAddResponseDetail> = Vec::with_capacity(products.len());

    // Batch load related data (3 queries instead of 3*N)
    let all_barcodes: Vec<Vec<schemas::product_barcodes::Model>> = products
        .load_many(schemas::product_barcodes::Entity, &app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    let all_lots: Vec<Vec<schemas::product_lots::Model>> = products
        .load_many(schemas::product_lots::Entity, &app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    let all_prices: Vec<Vec<schemas::product_prices::Model>> = products
        .load_many(schemas::product_prices::Entity, &app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    for ((p, barcodes), (lots, prices)) in products
        .iter()
        .zip(all_barcodes.iter())
        .zip(all_lots.iter().zip(all_prices.iter()))
    {
        // Most recent barcode
        let barcode_str = barcodes
            .last()
            .map(|b| b.barcode.clone())
            .unwrap_or_default();

        // Sum qty_on_hand from lots
        let total_qty = lots
            .iter()
            .fold(Decimal::new(0, 0), |acc, lot| acc + lot.qty_on_hand);

        // Most recent price (by starts_at then created_at)
        let price_value = prices
            .iter()
            .max_by(|a, b| {
                a.starts_at
                    .cmp(&b.starts_at)
                    .then(a.created_at.cmp(&b.created_at))
            })
            .map(|pr| pr.price)
            .unwrap_or_else(|| p.sale_price.unwrap_or(Decimal::new(0, 0)));

        results.push(ProductAddResponseDetail {
            id: p.id,
            sku: p.sku.clone(),
            name: p.name.clone(),
            barcode: barcode_str,
            description: p.description.clone(),
            qty_on_hand: total_qty,
            price: price_value,
            tax_profile_id: p.tax_profile_id,
            purchase_price: p.purchase_price,
            wholesale_price: p.wholesale_price,
            sale_price: p.sale_price,
            default_price: p.default_price,
        });
    }

    Ok(Json(ApiResponse::success(
        results,
        "Products retrieved successfully".to_string(),
        total_items as i32,
    )))
}

pub async fn delete_product(
    State(app_ctx): State<AppContext>,
    Path(product_id): Path<i32>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    info!("delete_product called with product_id: {:?}", product_id);

    // Verificar que el producto existe
    let product = schemas::products::Entity::find_by_id(product_id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    if product.is_none() {
        return Err(ApiError::ValidationError("Product not found".to_string()));
    }

    // Eliminar el producto (se pueden agregar cascadas en la base de datos para eliminar barcodes, precios y lotes relacionados, o hacerlo manualmente aquí antes de eliminar el producto)
    let res = schemas::products::Entity::delete_by_id(product_id)
        .exec(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    if res.rows_affected == 0 {
        return Err(ApiError::ValidationError(
            "Failed to delete product".to_string(),
        ));
    }

    info!("Deleted product with id: {}", product_id);
    info!("Rows affected: {}", res.rows_affected);

    Ok(Json(ApiResponse::success(
        (),
        "Product deleted successfully".to_string(),
        1,
    )))
}
