use axum::{
    Json,
    extract::{Path, Query, State},
};

use log::info;

use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, IntoActiveModel, LoaderTrait,
    ModelTrait, PaginatorTrait, QueryFilter, QueryOrder, TransactionTrait,
};
use validator::Validate;

use crate::{
    api_module::{
        add_sale::add_sale_dto::add_sale_dto::{SaleAddDetailResponse, SaleAddRequest},
        sale_items::{SaleItemDetailResponse, SaleItemRequest},
        sale_payment_allocations::sale_payment_allocations_dto::sale_payment_allocations_dto::{
            SalePaymentAllocationDetailResponse, SalePaymentAllocationRequest,
        },
        sale_payments::{SalePaymentDetailResponse, SalePaymentRequest},
        sales::{SaleDetailResponse, SaleIdResponse, SaleRequest},
    },
    api_utils::{api_response::PaginationParams, api_utils_fun::validate_date_time_range_date},
};

use crate::{
    api_utils::api_error::ApiError, api_utils::api_response::ApiResponse,
    config::config_database::config_db_context::AppContext,
};

pub async fn create_add_sale(
    State(app_ctx): State<AppContext>,
    Json(payload): Json<SaleAddRequest>,
) -> Result<Json<ApiResponse<SaleIdResponse>>, ApiError> {
    info!("create_add_sale called with payload: {:?}", payload);

    payload.validate().map_err(ApiError::Validation)?;

    let new_sale_create = SaleRequest::from(&payload);

    new_sale_create.validate().map_err(ApiError::Validation)?;

    // Ejecutar creación en una transacción: guardar producto, barcode, price y lot
    let txn = app_ctx
        .conn
        .begin()
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    // Encapsular la lógica en un bloque para capturar errores y poder hacer rollback en caso de fallo
    let op_result = (async {
        let new_sale_create_active_model =
            schemas::sales::ActiveModel::try_from(new_sale_create)
                .map_err(|e| ApiError::Unexpected(Box::new(std::io::Error::other(e))))?;

        // Se guarda el producto para obtener el ID generado (usar la transacción)
        let new_sale = new_sale_create_active_model
            .save(&txn)
            .await
            .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

        info!("Sale created valid ID");

        if new_sale.id.is_not_set() {
            return Err(ApiError::ValidationError(
                "Failed to create sale".to_string(),
            ));
        }
        // clonar para poder extraer el id sin mover el objeto original que se devolverá
        let new_sale_response = new_sale.clone();
        let sale_id = new_sale.id.clone().unwrap();
        info!("Sale created with ID: {}", sale_id);
        // Crear requests derivados usando la referencia al payload

        let payment_request = SalePaymentRequest::from((&payload, sale_id));
        payment_request.validate().map_err(ApiError::Validation)?;

        info!("Derived payment request: {:?}", payment_request);

        let payment_active_model =
            schemas::sale_payments::ActiveModel::try_from(payment_request)
                .map_err(|e| ApiError::Unexpected(Box::new(std::io::Error::other(e))))?;

        //Se guarda
        let new_payment = payment_active_model
            .save(&txn)
            .await
            .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

        if new_payment.id.is_not_set() {
            return Err(ApiError::ValidationError(
                "Failed to create payment".to_string(),
            ));
        }

        info!(
            "Payment created with ID: {}",
            new_payment.id.clone().unwrap()
        );
        let payment_id = new_payment.id.clone().unwrap();

        let payment_allocation_request =
            SalePaymentAllocationRequest::from((&payload, payment_id, sale_id));
        let item_requests: Vec<SaleItemRequest> = payload
            .items
            .iter()
            .map(|item| SaleItemRequest::from((item, sale_id)))
            .collect();

        payment_allocation_request
            .validate()
            .map_err(ApiError::Validation)?;
        item_requests
            .iter()
            .try_for_each(|item| item.validate().map_err(ApiError::Validation))?;

        info!(
            "Derived  allocation request: {:?}, item requests: {:?}",
            payment_allocation_request, item_requests
        );

        // Convertir a ActiveModel y guardar (en transacción)

        let payment_allocation_active_model =
            schemas::sale_payment_allocations::ActiveModel::try_from(payment_allocation_request)
                .map_err(|e| ApiError::Unexpected(Box::new(std::io::Error::other(e))))?;

        let item_active_models = item_requests
            .into_iter()
            .map(|item| {
                schemas::sale_items::ActiveModel::try_from(item)
                    .map_err(|e| ApiError::Unexpected(Box::new(std::io::Error::other(e))))
            })
            .collect::<Result<Vec<_>, ApiError>>()?;

        info!("Inicialización de ActiveModels para payment, allocation y items completada");

        let new_payment_allocation = payment_allocation_active_model
            .save(&txn)
            .await
            .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

        if new_payment_allocation.id.is_not_set() {
            return Err(ApiError::ValidationError(
                "Failed to create payment allocation".to_string(),
            ));
        }

        info!(
            "Payment allocation created with ID: {}",
            new_payment_allocation.id.unwrap()
        );

        for item_active_model in item_active_models {
            let saved_item = item_active_model
                .save(&txn)
                .await
                .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

            if saved_item.id.is_not_set() {
                return Err(ApiError::ValidationError(
                    "Failed to create sale item".to_string(),
                ));
            }
            info!("Sale item created with ID: {}", saved_item.id.unwrap());
        }
        Ok(new_sale_response)
    })
    .await;

    // Manejar commit/rollback según resultado y devolver respuesta
    return match op_result {
        Ok(new_sale_response) => {
            txn.commit()
                .await
                .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

            // enqueue async job to process sale (reports/notifications/payments worker)
            let sale_id_for_job = new_sale_response.id.clone();
            let _ = tokio::spawn(async move {
                match sale_id_for_job {
                    sea_orm::ActiveValue::Set(sid) => {
                        let job = serde_json::json!({"type": "sale_created", "sale_id": sid});
                        let _ = crate::config::config_redis::enqueue_json("jobs:sales", &job).await;
                    }
                    _ => {}
                }
            });

            Ok(Json(ApiResponse::success(
                SaleIdResponse::from(new_sale_response),
                "Sale created successfully".to_string(),
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

pub async fn get_add_sale_by_id(
    State(app_ctx): State<AppContext>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<SaleAddDetailResponse>>, ApiError> {
    info!("get_add_sale_by_id called with id: {:?}", id);

    let sale = schemas::sales::Entity::find_by_id(id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    match sale {
        Some(sale) => {
            // Obtener items, payment y payment allocation relacionados
            let items = sale
                .find_related(schemas::sale_items::Entity)
                .all(&app_ctx.conn)
                .await
                .map_err(|e| ApiError::Unexpected(Box::new(e)))?;
            let payment = sale
                .find_related(schemas::sale_payments::Entity)
                .one(&app_ctx.conn)
                .await
                .map_err(|e| ApiError::Unexpected(Box::new(e)))?;
            let payment_allocation = match &payment {
                Some(p) => p
                    .find_related(schemas::sale_payment_allocations::Entity)
                    .one(&app_ctx.conn)
                    .await
                    .map_err(|e| ApiError::Unexpected(Box::new(e)))?,
                None => None,
            };
            let sale_detail = SaleDetailResponse::from(sale);
            let items_detail = items
                .into_iter()
                .map(SaleItemDetailResponse::from)
                .collect::<Vec<_>>();
            let payment_detail = payment.map(SalePaymentDetailResponse::from);
            let allocation_detail =
                payment_allocation.map(SalePaymentAllocationDetailResponse::from);

            let response = SaleAddDetailResponse::from((
                sale_detail,
                items_detail,
                payment_detail,
                allocation_detail,
            ));
            Ok(Json(ApiResponse::success(
                response,
                "Sale retrieved successfully".to_string(),
                1,
            )))
        }
        None => Err(ApiError::ValidationError("Sale not found".to_string())),
    }
}

pub async fn get_add_sales_with_details(
    State(app_ctx): State<AppContext>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<ApiResponse<Vec<SaleAddDetailResponse>>>, ApiError> {
    let page_index = if pagination.page > 0 {
        pagination.page - 1
    } else {
        0
    };
    //Retornar SaleAddDetailResponse que incluye los detalles de items, payment y payment allocation
    let select = schemas::sales::Entity::find();
    //Aplicar filtros de búsqueda si se proporcionan (ejemplo: por fecha, cliente, etc.) - esto se puede extender según los campos disponibles en PaginationParams
    let select = if let Some(customer_id) = pagination.customer_id {
        select.filter(schemas::sales::Column::CustomerId.eq(customer_id))
    } else {
        select
    };
    //Por fecha rango de fecha se pueden agregar filtros similares usando Column::Date.gte(start_date) y Column::Date.lte(end_date) si se proporcionan en PaginationParams
    let fecha_init = pagination.date_init.as_deref();
    let fecha_end = pagination.date_end.as_deref();
    info!(
        "Fetching sales with pagination: page {}, limit {}, customer_id {:?}, date range {:?} - {:?}",
        pagination.page, pagination.limit, pagination.customer_id, fecha_init, fecha_end
    );
    let select = if let (Some(fecha_init), Some(fecha_end)) = (fecha_init, fecha_end) {
        let (fecha_init_dt, fecha_end_dt) = validate_date_time_range_date(fecha_init, fecha_end)?;

        if let (Some(fecha_init_dt), Some(fecha_end_dt)) = (fecha_init_dt, fecha_end_dt) {
            select.filter(
                schemas::sales::Column::Date
                    .gte(fecha_init_dt)
                    .and(schemas::sales::Column::Date.lte(fecha_end_dt)),
            )
        } else {
            select
        }
    } else {
        select
    };

    //User ID
    let select = if let Some(user_id) = pagination.user_id {
        select.filter(schemas::sales::Column::UserId.eq(user_id))
    } else {
        select
    };

    let paginator = select
        .order_by_asc(schemas::sales::Column::Id)
        .paginate(&app_ctx.conn, pagination.limit as u64);
    let total_items = if pagination.total > 0 {
        pagination.total
    } else {
        paginator
            .num_items()
            .await
            .map_err(|e| ApiError::Unexpected(Box::new(e)))?
    };
    let sales = paginator
        .fetch_page(page_index)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    // Batch load related data (3 queries instead of 3*N)
    let all_items: Vec<Vec<schemas::sale_items::Model>> = sales
        .load_many(schemas::sale_items::Entity, &app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    let all_payments: Vec<Vec<schemas::sale_payments::Model>> = sales
        .load_many(schemas::sale_payments::Entity, &app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    // Collect all payment models for batch loading allocations
    let payment_ids: Vec<i64> = all_payments
        .iter()
        .flat_map(|ps| ps.iter().map(|p| p.id))
        .collect();

    let allocations: Vec<schemas::sale_payment_allocations::Model> = if !payment_ids.is_empty() {
        schemas::sale_payment_allocations::Entity::find()
            .filter(schemas::sale_payment_allocations::Column::PaymentId.is_in(payment_ids))
            .all(&app_ctx.conn)
            .await
            .map_err(|e| ApiError::Unexpected(Box::new(e)))?
    } else {
        Vec::new()
    };

    // Build a map from payment_id to first allocation
    let mut allocation_map: std::collections::HashMap<
        i64,
        schemas::sale_payment_allocations::Model,
    > = std::collections::HashMap::new();
    for alloc in allocations {
        allocation_map.entry(alloc.payment_id).or_insert(alloc);
    }

    let mut sales_with_details = Vec::new();
    for ((sale, items), payments) in sales
        .into_iter()
        .zip(all_items.into_iter())
        .zip(all_payments.into_iter())
    {
        let payment = payments.into_iter().next();
        let payment_allocation = payment.as_ref().and_then(|p| allocation_map.remove(&p.id));

        let sale_detail = SaleDetailResponse::from(sale);
        let items_detail = items
            .into_iter()
            .map(SaleItemDetailResponse::from)
            .collect::<Vec<_>>();
        let payment_detail = payment.map(SalePaymentDetailResponse::from);
        let allocation_detail = payment_allocation.map(SalePaymentAllocationDetailResponse::from);
        let sale_with_details = SaleAddDetailResponse::from((
            sale_detail,
            items_detail,
            payment_detail,
            allocation_detail,
        ));
        sales_with_details.push(sale_with_details);
    }
    Ok(Json(ApiResponse::success(
        sales_with_details,
        "Sales retrieved successfully".to_string(),
        total_items as i32,
    )))
}

//Para cancelar una venta el estatus debe de ser CANCEL y se deben revertir los items de inventario
pub async fn cancel_add_sale(
    State(app_ctx): State<AppContext>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    info!("cancel_add_sale called with id: {:?}", id);

    let sale = schemas::sales::Entity::find_by_id(id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;
    match sale {
        Some(sale) => {
            if sale.status == "CANCEL" {
                return Err(ApiError::ValidationError(
                    "Sale is already cancelled".to_string(),
                ));
            }

            let txn = app_ctx
                .conn
                .begin()
                .await
                .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

            // Marcar la venta como CANCEL
            let mut sale_active = sale.clone().into_active_model();
            sale_active.status = ActiveValue::Set("CANCEL".to_string());
            sale_active
                .save(&txn)
                .await
                .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

            // Revert inventory: add back quantities from sale items to their lots
            let items = schemas::sale_items::Entity::find()
                .filter(schemas::sale_items::Column::SaleId.eq(id))
                .all(&txn)
                .await
                .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

            for item in items {
                if let Some(lot_id) = item.lot_id {
                    let lot = schemas::product_lots::Entity::find_by_id(lot_id)
                        .one(&txn)
                        .await
                        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

                    if let Some(lot) = lot {
                        let mut lot_active = lot.into_active_model();
                        let restored_qty = lot_active.qty_on_hand.clone().unwrap() + item.qty;
                        lot_active.qty_on_hand = ActiveValue::Set(restored_qty);
                        lot_active
                            .save(&txn)
                            .await
                            .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

                        info!(
                            "Inventory restored: lot_id={}, qty_returned={}, new_qty={}",
                            lot_id, item.qty, restored_qty
                        );
                    }
                }
            }

            txn.commit()
                .await
                .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

            Ok(Json(ApiResponse::success(
                (),
                "Sale cancelled successfully".to_string(),
                0,
            )))
        }
        None => Err(ApiError::ValidationError("Sale not found".to_string())),
    }
}
