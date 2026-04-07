use axum::{
    Json,
    extract::{Path, Query, State},
};

use log::info;
use sea_orm::entity::prelude::*;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, IntoActiveModel, ModelTrait,
    PaginatorTrait, QueryFilter, QueryOrder, TransactionTrait,
};
use validator::Validate;

use crate::{
    api_module::{
        add_product::add_product_dto::add_product_dto::{
            ProductAddRequest, ProductAddResponseDetail,
        },
        product_barcodes::product_barcodes_dto::product_barcodes_dto::ProductBarcodeRequest,
        product_lots::ProductLotRequest,
        product_prices::ProductPriceRequest,
        products::products_dto::products_dto::{ProductIdResponse, ProductRequest},
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
    Json(payload): Json<ProductAddRequest>,
) -> Result<Json<ApiResponse<ProductIdResponse>>, ApiError> {
    info!("Adding new product: {:?}", payload);

    //Valida el payload
    payload.validate().map_err(ApiError::Validation)?;

    //Crea el producto (usar referencia para no mover `payload`)
    let new_product_create = ProductRequest::from(&payload);

    new_product_create
        .validate()
        .map_err(ApiError::Validation)?;

    // Ejecutar creación en una transacción: guardar producto, barcode, price y lot
    let txn = app_ctx
        .conn
        .begin()
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    // Encapsular la lógica en un bloque para capturar errores y poder hacer rollback en caso de fallo
    let op_result = (async {
        // Asigna a ActiveModel
        let new_product_active_model = schemas::products::ActiveModel::try_from(new_product_create)
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
        let new_product_barcode = ProductBarcodeRequest::from((&payload, product_id));
        let new_product_price = ProductPriceRequest::from((&payload, product_id));
        let new_product_lot = ProductLotRequest::from((&payload, product_id));

        // Validaciones
        new_product_barcode
            .validate()
            .map_err(ApiError::Validation)?;
        new_product_price.validate().map_err(ApiError::Validation)?;
        new_product_lot.validate().map_err(ApiError::Validation)?;

        // Convertir a ActiveModel y guardar (en transacción)
        let new_product_barcode_active_model =
            schemas::product_barcodes::ActiveModel::try_from(new_product_barcode)
                .map_err(|e| ApiError::Unexpected(Box::new(std::io::Error::other(e))))?;
        let new_product_price_active_model =
            schemas::product_prices::ActiveModel::try_from(new_product_price)
                .map_err(|e| ApiError::Unexpected(Box::new(std::io::Error::other(e))))?;
        let new_product_lot_active_model =
            schemas::product_lots::ActiveModel::try_from(new_product_lot)
                .map_err(|e| ApiError::Unexpected(Box::new(std::io::Error::other(e))))?;

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

    let mut results: Vec<ProductAddResponseDetail> = Vec::with_capacity(products.len());

    for p in products {
        // obtener barcode más reciente si existe
        let pb = schemas::product_barcodes::Entity::find()
            .filter(schemas::product_barcodes::Column::ProductId.eq(p.id))
            .order_by_desc(schemas::product_barcodes::Column::Id)
            .one(&app_ctx.conn)
            .await
            .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

        let barcode_str = pb.map(|b| b.barcode).unwrap_or_default();

        // sumar qty_on_hand de lots
        let lots = schemas::product_lots::Entity::find()
            .filter(schemas::product_lots::Column::ProductId.eq(p.id))
            .all(&app_ctx.conn)
            .await
            .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

        let mut total_qty = Decimal::new(0, 0);
        for lot in &lots {
            total_qty = total_qty + lot.qty_on_hand;
        }

        // precio vigente
        let price_rec = schemas::product_prices::Entity::find()
            .filter(schemas::product_prices::Column::ProductId.eq(p.id))
            .order_by_desc(schemas::product_prices::Column::StartsAt)
            .order_by_desc(schemas::product_prices::Column::CreatedAt)
            .one(&app_ctx.conn)
            .await
            .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

        let price_value = match price_rec {
            Some(pr) => pr.price,
            None => p.sale_price.unwrap_or(Decimal::new(0, 0)),
        };

        results.push(ProductAddResponseDetail {
            id: p.id,
            sku: p.sku,
            name: p.name,
            barcode: barcode_str,
            description: p.description,
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
