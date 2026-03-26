use axum::{
    Json,
    extract::{Path, Query, State},
};
use log::info;

use sea_orm::{DatabaseTransaction, IntoActiveModel, entity::prelude::*};

use crate::{
    api_handlers::sales::sales_dto::{
        PaginationParamsSales, SalesDetailResponseDTO, SalesRequestDTO, SalesResponse,
        SalesResponseDTO, SalesResponseIdDTO,
    },
    api_utils::{api_error::ApiError, api_response::ApiResponse},
    config::config_database::config_db_context::AppContext,
};

use validator::Validate;

use sea_orm::QueryOrder;
use sea_orm::TransactionTrait;
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter,
};

// ---------------------- Helpers ----------------------
/// Name: `parse_date_str`
/// Description: Parse a `YYYY-MM-DD` string into a `sea_orm::Date`.
/// Parameters: `date_str` - string slice with date in `YYYY-MM-DD` format.
/// Outputs: `Ok(Date)` on success or `Err(ApiError::ValidationError)` on parse failure.
fn parse_date_str(date_str: &str) -> Result<Date, ApiError> {
    let naive = chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d");
    match naive {
        Ok(d) => {
            let date: Date = d;
            Ok(date)
        }
        Err(e) => Err(ApiError::ValidationError(format!("Invalid date: {}", e))),
    }
}

/// Name: `to_page_index`
/// Description: Convert client 1-based `page` to 0-based page index used by paginators.
/// Parameters: `page` - page number from client (u64).
/// Outputs: 0-based `usize` page index.
fn to_page_index(page: u64) -> u64 {
    if page == 0 {
        0
    } else {
        (page.saturating_sub(1)) as u64
    }
}

/// Name: `ventas_to_responses`
/// Description: Given a list of `sale` models, fetches their detail lines and converts
///              each sale into a `SalesResponse` DTO.
/// Parameters: `app_ctx` - application context with DB connection; `ventas` - vector of sales models.
/// Outputs: `Ok(Vec<SalesResponse>)` or `Err(ApiError)` on DB errors.
async fn ventas_to_responses(
    app_ctx: &AppContext,
    ventas: Vec<schemas::sale::Model>,
) -> Result<Vec<SalesResponse>, ApiError> {
    let mut response_dtos = Vec::new();

    for venta in ventas {
        let detalles = schemas::saledetal::Entity::find()
            .order_by_asc(schemas::saledetal::Column::Id)
            .filter(schemas::saledetal::Column::IdSale.eq(venta.id))
            .all(&app_ctx.conn)
            .await?;

        response_dtos.push(SalesResponse::from((venta, detalles)));
    }

    Ok(response_dtos)
}

/// Name: `fetch_sales_between_dates`
/// Description: Paginates sales between two dates.
/// Parameters: `app_ctx` - application context; `start`/`end` - date range; `page`/`limit` - pagination.
/// Outputs: `Ok((Vec<sale::Model>, usize))` where usize is total items, or `Err(ApiError)`.
async fn fetch_sales_between_dates(
    app_ctx: &AppContext,
    start: Date,
    end: Date,
    page: u64,
    limit: u64,
) -> Result<(Vec<schemas::sale::Model>, u64), ApiError> {
    let paginator = schemas::sale::Entity::find()
        .order_by_asc(schemas::sale::Column::Id)
        .filter(schemas::sale::Column::DateSale.between(start.clone(), end.clone()))
        .paginate(&app_ctx.conn, limit);

    let total_items = paginator
        .num_items()
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    let ventas = paginator
        .fetch_page(to_page_index(page))
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    Ok((ventas, total_items))
}

/// Name: `sum_sales_between_dates_username`
/// Description: Computes the total `total` value for sales in a date range, optionally
///              filtering by username and always excluding cancelled sales when requested.
/// Parameters: `app_ctx`, `start`, `end`, `username` (Option<String>), `exclude_cancel` (bool)
/// Outputs: `Ok(f32)` with the summed total or `Err(ApiError)`.
async fn sum_sales_between_dates_username(
    app_ctx: &AppContext,
    start: Date,
    end: Date,
    username: Option<String>,
    exclude_cancel: bool,
) -> Result<f32, ApiError> {
    let mut query = schemas::sale::Entity::find()
        .filter(schemas::sale::Column::DateSale.between(start.clone(), end.clone()))
        .order_by_asc(schemas::sale::Column::Id);

    if let Some(u) = username {
        query = query.filter(schemas::sale::Column::Username.eq(u));
    }

    if exclude_cancel {
        query = query.filter(schemas::sale::Column::Status.ne("CANCEL".to_string()));
    }

    let total: f32 = query
        .all(&app_ctx.conn)
        .await?
        .into_iter()
        .map(|venta| venta.total)
        .sum();

    Ok(total)
}

// -----------------------------------------------------

/// Name: create_sale_handler
/// Description: Create a new sale (parent and details), update product counts.
/// Parameters: `app_ctx` - app context; `payload` - `SalesRequestDTO` with sale and details.
/// Outputs: `ApiResponse<SalesResponseIdDTO>` containing created sale id.
pub async fn create_sale_handler(
    State(app_ctx): State<AppContext>,
    Json(payload): Json<SalesRequestDTO>,
) -> Result<Json<ApiResponse<SalesResponseIdDTO>>, ApiError> {
    info!("Creating a new sale with payload: {:?}", payload);

    payload.validate().map_err(ApiError::Validation)?;

    if payload.details.is_empty() {
        return Err(ApiError::ValidationError(
            "Sale must have at least one detail line".to_string(),
        ));
    }

    // Begin a DB transaction so we can rollback on any error
    let txn: DatabaseTransaction = app_ctx
        .conn
        .begin()
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    let venta_padre = schemas::sale::ActiveModel {
        id: ActiveValue::NotSet,
        date_sale: ActiveValue::Set(payload.date_sale),
        discount: ActiveValue::Set(payload.discount),
        id_sale_detl: ActiveValue::Set(payload.id_sale_detl),
        iva: ActiveValue::Set(payload.iva),
        msg: ActiveValue::Set(payload.msg.clone()),
        payment_method: ActiveValue::Set(payload.payment_method.clone()),
        payment_status: ActiveValue::Set(payload.payment_status.clone()),
        status: ActiveValue::Set(payload.status.clone()),
        sub_total: ActiveValue::Set(payload.sub_total),
        time_sale: ActiveValue::Set(payload.time_sale),
        total: ActiveValue::Set(payload.total),
        username: ActiveValue::Set(payload.username.clone()),
    };

    // Save parent using transaction
    let venta_guardada = match venta_padre.save(&txn).await {
        Ok(v) => v,
        Err(e) => {
            let _ = txn.rollback().await;
            return Err(ApiError::Unexpected(Box::new(e)));
        }
    };

    let venta_id = match venta_guardada.id {
        ActiveValue::Set(v) => v,
        ActiveValue::NotSet => 0,
        ActiveValue::Unchanged(v) => v,
    };

    info!("Sale created with ID: {}", venta_id);

    for detalle in payload.details {
        let detalle_hijo = schemas::saledetal::ActiveModel {
            id: ActiveValue::NotSet,
            date_sale: ActiveValue::Set(detalle.date_sale),
            product_code_bar: ActiveValue::Set(detalle.product_code_bar.clone()),
            product_count: ActiveValue::Set(detalle.product_count),
            product_id: ActiveValue::Set(detalle.product_id),
            product_price: ActiveValue::Set(detalle.product_price),
            time_sale: ActiveValue::Set(detalle.time_sale),
            id_sale: ActiveValue::Set(venta_id),
        };
        info!(
            "Saving sale detail for product code bar: {} with count: {}",
            detalle.product_code_bar.clone().unwrap_or_default(),
            detalle.product_count.unwrap_or_default()
        );

        if let Err(e) = detalle_hijo.save(&txn).await {
            let _ = txn.rollback().await;
            return Err(ApiError::Unexpected(Box::new(e)));
        }

        /*Actualizar contador de productos de la tabla product, por cada venta  */
        let product_id = detalle.product_id;
        if product_id != 0 {
            let producto = match schemas::product::Entity::find_by_id(product_id)
                .one(&txn)
                .await
            {
                Ok(Some(p)) => p,
                Ok(None) => {
                    let _ = txn.rollback().await;
                    return Err(ApiError::NotFound);
                }
                Err(e) => {
                    let _ = txn.rollback().await;
                    return Err(ApiError::Unexpected(Box::new(e)));
                }
            };

            let current_count: i32 = producto.product_count;
            let sold_count: i32 = detalle.product_count.unwrap_or_default() as i32;
            let new_count = current_count - sold_count;

            let mut producto_active = producto.into_active_model();

            producto_active.product_count = ActiveValue::Set(new_count);

            if let Err(e) = producto_active.save(&txn).await {
                let _ = txn.rollback().await;
                return Err(ApiError::Unexpected(Box::new(e)));
            }
        }
    }

    // Commit transaction
    if let Err(e) = txn.commit().await {
        return Err(ApiError::Unexpected(Box::new(e)));
    }

    Ok(Json(ApiResponse::new(
        SalesResponseIdDTO { id: venta_id },
        1,
        "Sale created successfully".to_string(),
        "success".to_string(),
        201,
        chrono::Utc::now().to_rfc3339(),
    )))
}

/// Name: get_sales_by_id_handler
/// Description: Fetch a sale (parent) and its details by sale ID.
/// Parameters: `app_ctx` - app context; `sale_id` - sale identifier.
/// Outputs: `ApiResponse<SalesResponse>` containing sale and its details.
pub async fn get_sales_by_id_handler(
    State(app_ctx): State<AppContext>,
    Path(sale_id): Path<i64>,
) -> Result<Json<ApiResponse<SalesResponse>>, ApiError> {
    info!("Fetching sale with ID: {}", sale_id);

    let venta = schemas::sale::Entity::find_by_id(sale_id)
        .one(&app_ctx.conn)
        .await?
        .ok_or(ApiError::NotFound)?;

    let detalles = schemas::saledetal::Entity::find()
        .filter(schemas::saledetal::Column::IdSale.eq(sale_id))
        .all(&app_ctx.conn)
        .await?;

    if detalles.is_empty() {
        return Err(ApiError::NotFound);
    }

    Ok(Json(ApiResponse::new(
        SalesResponse::from((venta, detalles)),
        1,
        "Sale fetched successfully".to_string(),
        "success".to_string(),
        200,
        chrono::Utc::now().to_rfc3339(),
    )))
}

pub async fn get_sales_by_date_handler(
    State(app_ctx): State<AppContext>,
    Query(payload): Query<PaginationParamsSales>,
) -> Result<Json<ApiResponse<Vec<SalesResponse>>>, ApiError> {
    let date_sale = payload.date_inicio.clone().unwrap_or_default();

    info!("Fetching sales for date: {}", date_sale);

    let date_date = parse_date_str(&date_sale)?;

    let paginator = schemas::sale::Entity::find()
        .filter(schemas::sale::Column::DateSale.eq(date_date.clone()))
        .order_by_asc(schemas::sale::Column::Id)
        .paginate(&app_ctx.conn, payload.limit);

    let total_items = paginator
        .num_items()
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    let ventas = paginator
        .fetch_page(to_page_index(payload.page))
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    let response_dtos = ventas_to_responses(&app_ctx, ventas).await?;

    if response_dtos.is_empty() {
        return Err(ApiError::NotFound);
    }

    Ok(Json(ApiResponse::new(
        response_dtos.clone(),
        total_items as i32,
        "Sales fetched successfully".to_string(),
        "success".to_string(),
        200,
        chrono::Utc::now().to_rfc3339(),
    )))
}

pub async fn get_sales_by_username_handler(
    State(app_ctx): State<AppContext>,
    Query(payload): Query<PaginationParamsSales>,
) -> Result<Json<ApiResponse<Vec<SalesResponse>>>, ApiError> {
    let username = payload.username.clone().unwrap_or_default();
    info!("Fetching sales for username: {}", username);

    let paginator = schemas::sale::Entity::find()
        .filter(schemas::sale::Column::Username.eq(username.clone()))
        .order_by_asc(schemas::sale::Column::Id)
        .paginate(&app_ctx.conn, payload.limit);

    let total_items = paginator
        .num_items()
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    let ventas = paginator
        .fetch_page(to_page_index(payload.page))
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    let response_dtos = ventas_to_responses(&app_ctx, ventas).await?;

    if response_dtos.is_empty() {
        return Err(ApiError::NotFound);
    }

    Ok(Json(ApiResponse::new(
        response_dtos.clone(),
        total_items as i32,
        "Sales fetched successfully".to_string(),
        "success".to_string(),
        200,
        chrono::Utc::now().to_rfc3339(),
    )))
}

pub async fn get_sales_by_date_ini_fin_handler(
    State(app_ctx): State<AppContext>,
    Query(payload): Query<PaginationParamsSales>,
) -> Result<Json<ApiResponse<Vec<SalesResponse>>>, ApiError> {
    let date_inicio_str = payload.date_inicio.clone().unwrap_or_default();
    let date_fin_str = payload.date_fin.clone().unwrap_or_default();

    info!(
        "Fetching sales between dates: {} and {}",
        date_inicio_str, date_fin_str
    );

    // parse date_inicio/date_fin (expecting YYYY-MM-DD)
    let date_inicio = parse_date_str(&date_inicio_str)?;
    let date_fin = parse_date_str(&date_fin_str)?;

    if date_inicio > date_fin {
        return Err(ApiError::ValidationError(
            "date_inicio must be <= date_fin".to_string(),
        ));
    }

    // convert to sea_orm::Date (chrono::NaiveDate) and paginate
    let (ventas, total_items) =
        fetch_sales_between_dates(&app_ctx, date_inicio, date_fin, payload.page, payload.limit)
            .await?;

    let response_dtos = ventas_to_responses(&app_ctx, ventas).await?;

    let total_i32 = total_items as i32;

    if response_dtos.is_empty() {
        return Err(ApiError::NotFound);
    }

    Ok(Json(ApiResponse::new(
        response_dtos,
        total_i32,
        "Sales fetched successfully".to_string(),
        "success".to_string(),
        200,
        chrono::Utc::now().to_rfc3339(),
    )))
}

pub async fn get_sum_sales_by_date_handler(
    State(app_ctx): State<AppContext>,
    Path(date_sale): Path<String>,
) -> Result<Json<ApiResponse<f32>>, ApiError> {
    info!("Calculating total sales for date: {}", date_sale);

    let date = parse_date_str(&date_sale)?;

    let total_sales = sum_sales_between_dates_username(&app_ctx, date, date, None, false).await?;

    if total_sales == 0.0 {
        return Err(ApiError::NotFound);
    }

    Ok(Json(ApiResponse::new(
        total_sales,
        1,
        "Total sales calculated successfully".to_string(),
        "success".to_string(),
        200,
        chrono::Utc::now().to_rfc3339(),
    )))
}

pub async fn get_sum_sales_by_date_ini_fin_handler(
    State(app_ctx): State<AppContext>,
    Query(payload): Query<PaginationParamsSales>,
) -> Result<Json<ApiResponse<f32>>, ApiError> {
    let date_inicio_str = payload.date_inicio.clone().unwrap_or_default();
    let date_fin_str = payload.date_fin.clone().unwrap_or_default();
    info!(
        "Calculating total sales between dates: {} and {}",
        date_inicio_str, date_fin_str
    );
    // parse date_inicio/date_fin (expecting YYYY-MM-DD)
    let date_inicio = parse_date_str(&date_inicio_str)?;
    let date_fin = parse_date_str(&date_fin_str)?;

    if date_inicio > date_fin {
        return Err(ApiError::ValidationError(
            "date_inicio must be <= date_fin".to_string(),
        ));
    }

    let total_sales =
        sum_sales_between_dates_username(&app_ctx, date_inicio, date_fin, None, false).await?;

    if total_sales == 0.0 {
        return Err(ApiError::NotFound);
    }

    Ok(Json(ApiResponse::new(
        total_sales,
        1,
        "Total sales calculated successfully".to_string(),
        "success".to_string(),
        200,
        chrono::Utc::now().to_rfc3339(),
    )))
}

pub async fn get_sum_sales_by_username_handler(
    State(app_ctx): State<AppContext>,
    Path(username): Path<String>,
) -> Result<Json<ApiResponse<f32>>, ApiError> {
    info!("Calculating total sales for username: {}", username);

    let total_sales = schemas::sale::Entity::find()
        .filter(schemas::sale::Column::Username.eq(username.clone()))
        .order_by_asc(schemas::sale::Column::Id)
        .all(&app_ctx.conn)
        .await?
        .into_iter()
        .map(|venta| venta.total)
        .sum();

    if total_sales == 0.0 {
        return Err(ApiError::NotFound);
    }

    Ok(Json(ApiResponse::new(
        total_sales,
        1,
        "Total sales calculated successfully".to_string(),
        "success".to_string(),
        200,
        chrono::Utc::now().to_rfc3339(),
    )))
}

pub async fn get_sum_sales_by_date_ini_fin_username_handler(
    State(app_ctx): State<AppContext>,
    Query(payload): Query<PaginationParamsSales>,
) -> Result<Json<ApiResponse<f32>>, ApiError> {
    let date_inicio_str = payload.date_inicio.clone().unwrap_or_default();
    let date_fin_str = payload.date_fin.clone().unwrap_or_default();
    let username = payload.username.clone().unwrap_or_default();
    info!(
        "Calculating total sales for username: {} between dates: {} and {}",
        username, date_inicio_str, date_fin_str
    );

    // parse date_inicio/date_fin (expecting YYYY-MM-DD)
    let date_inicio = parse_date_str(&date_inicio_str)?;
    let date_fin = parse_date_str(&date_fin_str)?;

    if date_inicio > date_fin {
        return Err(ApiError::ValidationError(
            "date_inicio must be <= date_fin".to_string(),
        ));
    }

    let total_sales =
        sum_sales_between_dates_username(&app_ctx, date_inicio, date_fin, Some(username), true)
            .await?;

    if total_sales == 0.0 {
        return Err(ApiError::NotFound);
    }

    Ok(Json(ApiResponse::new(
        total_sales,
        1,
        "Total sales calculated successfully".to_string(),
        "success".to_string(),
        200,
        chrono::Utc::now().to_rfc3339(),
    )))
}

/*FUncion para obtener solo el detalle padre de la ventas por fecha inicio y fecha fin */
pub async fn get_sales_detail_by_date_ini_fin_handler(
    State(app_ctx): State<AppContext>,
    Query(payload): Query<PaginationParamsSales>,
) -> Result<Json<ApiResponse<Vec<SalesResponseDTO>>>, ApiError> {
    let date_inicio_str = payload.date_inicio.clone().unwrap_or_default();
    let date_fin_str = payload.date_fin.clone().unwrap_or_default();
    info!(
        "Fetching sale details for between dates: {} and {}",
        date_inicio_str, date_fin_str
    );

    // parse date_inicio/date_fin (expecting YYYY-MM-DD)
    let date_inicio = parse_date_str(&date_inicio_str)?;

    let date_fin = parse_date_str(&date_fin_str)?;

    if date_inicio > date_fin {
        return Err(ApiError::ValidationError(
            "date_inicio must be <= date_fin".to_string(),
        ));
    }

    // convert to sea_orm::Date (chrono::NaiveDate)
    let date_inicio_date: Date = date_inicio;
    let date_fin_date: Date = date_fin;

    // paginator: convert client 1-based page to 0-based page index
    let page_index = to_page_index(payload.page);

    let paginator = schemas::sale::Entity::find()
        .filter(
            schemas::sale::Column::DateSale
                .between(date_inicio_date.clone(), date_fin_date.clone()),
        )
        .order_by_asc(schemas::sale::Column::Id)
        .paginate(&app_ctx.conn, payload.limit);

    let total_items = paginator
        .num_items()
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    let ventas = paginator
        .fetch_page(page_index)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    if ventas.is_empty() {
        return Err(ApiError::NotFound);
    }

    Ok(Json(ApiResponse::new(
        ventas
            .into_iter()
            .map(SalesResponseDTO::from)
            .collect::<Vec<_>>(),
        total_items as i32,
        "Sales details fetched successfully".to_string(),
        "success".to_string(),
        200,
        chrono::Utc::now().to_rfc3339(),
    )))
}

/*FUncion para obtener solo el detalle HIjo de la ventas POR ID del padre  */
pub async fn get_sales_detail_by_id_handler(
    State(app_ctx): State<AppContext>,
    Path(sale_id): Path<i64>,
) -> Result<Json<ApiResponse<Vec<SalesDetailResponseDTO>>>, ApiError> {
    info!("Fetching sale details for sale ID: {}", sale_id);

    let detalles = schemas::saledetal::Entity::find()
        .filter(schemas::saledetal::Column::IdSale.eq(sale_id))
        .order_by_asc(schemas::saledetal::Column::Id)
        .all(&app_ctx.conn)
        .await?;

    let total = detalles.len() as i32;

    if detalles.is_empty() {
        return Err(ApiError::NotFound);
    }

    Ok(Json(ApiResponse::new(
        detalles
            .into_iter()
            .map(SalesDetailResponseDTO::from)
            .collect(),
        total,
        "Sale details fetched successfully".to_string(),
        "success".to_string(),
        200,
        chrono::Utc::now().to_rfc3339(),
    )))
}

pub async fn cancel_sale_handler(
    State(app_ctx): State<AppContext>,
    Path(sale_id): Path<i64>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    info!("Cancelling sale with ID: {}", sale_id);

    let venta = schemas::sale::Entity::find_by_id(sale_id)
        .one(&app_ctx.conn)
        .await?
        .ok_or(ApiError::NotFound)?;

    let mut venta_active = venta.into_active_model();
    venta_active.status = ActiveValue::Set(Some("CANCEL".to_string()));
    venta_active.save(&app_ctx.conn).await?;

    /*Regresar el count de los productos por cada producto vendido */
    let detalles = schemas::saledetal::Entity::find()
        .filter(schemas::saledetal::Column::IdSale.eq(sale_id))
        .all(&app_ctx.conn)
        .await?;

    for detalle in detalles {
        let product_id = detalle.product_id;
        if product_id != 0 {
            let producto = schemas::product::Entity::find_by_id(product_id)
                .one(&app_ctx.conn)
                .await?
                .ok_or(ApiError::NotFound)?;

            let current_count: i32 = producto.product_count;
            let sold_count: i32 = detalle.product_count.unwrap_or_default() as i32;
            let new_count = current_count + sold_count;

            let mut producto_active = producto.into_active_model();
            producto_active.product_count = ActiveValue::Set(new_count);
            producto_active.save(&app_ctx.conn).await?;
        }
    }

    Ok(Json(ApiResponse::new(
        (),
        0,
        "Sale cancelled successfully".to_string(),
        "success".to_string(),
        200,
        chrono::Utc::now().to_rfc3339(),
    )))
}

pub async fn delete_sale_handler(
    State(app_ctx): State<AppContext>,
    Path(sale_id): Path<i64>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    info!("Deleting sale with ID: {}", sale_id);

    let venta = schemas::sale::Entity::find_by_id(sale_id)
        .one(&app_ctx.conn)
        .await?
        .ok_or(ApiError::NotFound)?;

    venta.delete(&app_ctx.conn).await?;

    /*Eliminar venta detalle por ID de la venta */
    let detalles = schemas::saledetal::Entity::find()
        .filter(schemas::saledetal::Column::IdSale.eq(sale_id))
        .all(&app_ctx.conn)
        .await?;

    for detalle in detalles {
        detalle.delete(&app_ctx.conn).await?;
    }

    Ok(Json(ApiResponse::new(
        (),
        0,
        "Sale deleted successfully".to_string(),
        "success".to_string(),
        200,
        chrono::Utc::now().to_rfc3339(),
    )))
}
