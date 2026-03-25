use axum::{
    Json,
    extract::{Path, Query, State},
};
use log::info;

use sea_orm::{IntoActiveModel, entity::prelude::*};

use crate::{
    api_handlers::sales::sales_dto::{
        PaginationParamsSales, SalesDetailResponseDTO, SalesRequestDTO, SalesResponse,
        SalesResponseDTO, SalesResponseIdDTO,
    },
    api_utils::{api_error::ApiError, api_response::ApiResponse},
    config::config_database::config_db_context::AppContext,
};

use validator::Validate;

use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter,
};

pub async fn create_sale_handler(
    State(app_ctx): State<AppContext>,
    Json(payload): Json<SalesRequestDTO>,
) -> Result<Json<ApiResponse<SalesResponseIdDTO>>, ApiError> {
    info!("Creating a new sale with payload: {:?}", payload);

    payload.validate().map_err(ApiError::Validation)?;

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

    let venta_guardada = venta_padre.save(&app_ctx.conn).await?;

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

        detalle_hijo.save(&app_ctx.conn).await?;

        /*Actualizar contador de productos de la tabla product, por cada venta  */
        let product_id = detalle.product_id;
        if product_id != 0 {
            let producto = schemas::product::Entity::find_by_id(product_id)
                .one(&app_ctx.conn)
                .await?
                .ok_or(ApiError::NotFound)?;

            let current_count: i32 = producto.product_count;
            let sold_count: i32 = detalle.product_count.unwrap_or_default() as i32;
            let new_count = current_count - sold_count;

            let mut producto_active = producto.into_active_model();

            producto_active.product_count = ActiveValue::Set(new_count);

            producto_active.save(&app_ctx.conn).await?;
        }
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
    Path(date_sale): Path<String>,
) -> Result<Json<ApiResponse<Vec<SalesResponse>>>, ApiError> {
    info!("Fetching sales for date: {}", date_sale);

    let ventas = schemas::sale::Entity::find()
        .filter(schemas::sale::Column::DateSale.eq(date_sale))
        .all(&app_ctx.conn)
        .await?;

    let mut response_dtos = Vec::new();

    for venta in ventas {
        let detalles = schemas::saledetal::Entity::find()
            .filter(schemas::saledetal::Column::IdSale.eq(venta.id))
            .all(&app_ctx.conn)
            .await?;

        let response_dto = SalesResponse::from((venta, detalles));

        response_dtos.push(response_dto);
    }

    Ok(Json(ApiResponse::new(
        response_dtos.clone(),
        response_dtos.len() as i32,
        "Sales fetched successfully".to_string(),
        "success".to_string(),
        200,
        chrono::Utc::now().to_rfc3339(),
    )))
}

pub async fn get_sales_by_username_handler(
    State(app_ctx): State<AppContext>,
    Path(username): Path<String>,
) -> Result<Json<ApiResponse<Vec<SalesResponse>>>, ApiError> {
    info!("Fetching sales for username: {}", username);

    let ventas = schemas::sale::Entity::find()
        .filter(schemas::sale::Column::Username.eq(username.clone()))
        .all(&app_ctx.conn)
        .await?;

    let mut response_dtos = Vec::new();

    for venta in ventas {
        let detalles = schemas::saledetal::Entity::find()
            .filter(schemas::saledetal::Column::IdSale.eq(venta.id))
            .all(&app_ctx.conn)
            .await?;

        let response_dto = SalesResponse::from((venta, detalles));
        response_dtos.push(response_dto);
    }

    Ok(Json(ApiResponse::new(
        response_dtos.clone(),
        response_dtos.len() as i32,
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
    info!(
        "Fetching sales between dates: {} and {}",
        payload.date_inicio, payload.date_fin
    );

    // parse date_inicio/date_fin (expecting YYYY-MM-DD)
    let date_inicio = match chrono::NaiveDate::parse_from_str(&payload.date_inicio, "%Y-%m-%d") {
        Ok(d) => d,
        Err(e) => {
            return Err(ApiError::ValidationError(format!(
                "Invalid date_inicio: {}",
                e
            )));
        }
    };

    let date_fin = match chrono::NaiveDate::parse_from_str(&payload.date_fin, "%Y-%m-%d") {
        Ok(d) => d,
        Err(e) => {
            return Err(ApiError::ValidationError(format!(
                "Invalid date_fin: {}",
                e
            )));
        }
    };

    if date_inicio > date_fin {
        return Err(ApiError::ValidationError(
            "date_inicio must be <= date_fin".to_string(),
        ));
    }

    // convert to sea_orm::Date (chrono::NaiveDate)
    let date_inicio_date: Date = date_inicio;
    let date_fin_date: Date = date_fin;

    // paginator: convert client 1-based page to 0-based page index
    let page_index = if payload.page == 0 {
        0
    } else {
        payload.page.saturating_sub(1)
    };

    let paginator = schemas::sale::Entity::find()
        .filter(
            schemas::sale::Column::DateSale
                .between(date_inicio_date.clone(), date_fin_date.clone()),
        )
        .paginate(&app_ctx.conn, payload.limit);

    let total_items = paginator
        .num_items()
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    let ventas = paginator
        .fetch_page(page_index)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    let mut response_dtos = Vec::new();

    for venta in ventas {
        let detalles = schemas::saledetal::Entity::find()
            .filter(schemas::saledetal::Column::IdSale.eq(venta.id))
            .all(&app_ctx.conn)
            .await?;

        let response_dto = SalesResponse::from((venta, detalles));

        response_dtos.push(response_dto);
    }

    let total_i32 = total_items as i32;

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

    let total_sales = schemas::sale::Entity::find()
        .filter(schemas::sale::Column::DateSale.eq(date_sale))
        .all(&app_ctx.conn)
        .await?
        .into_iter()
        .map(|venta| venta.total)
        .sum();

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
    info!(
        "Calculating total sales between dates: {} and {}",
        payload.date_inicio, payload.date_fin
    );

    // parse date_inicio/date_fin (expecting YYYY-MM-DD)
    let date_inicio = match chrono::NaiveDate::parse_from_str(&payload.date_inicio, "%Y-%m-%d") {
        Ok(d) => d,
        Err(e) => {
            return Err(ApiError::ValidationError(format!(
                "Invalid date_inicio: {}",
                e
            )));
        }
    };

    let date_fin = match chrono::NaiveDate::parse_from_str(&payload.date_fin, "%Y-%m-%d") {
        Ok(d) => d,
        Err(e) => {
            return Err(ApiError::ValidationError(format!(
                "Invalid date_fin: {}",
                e
            )));
        }
    };

    if date_inicio > date_fin {
        return Err(ApiError::ValidationError(
            "date_inicio must be <= date_fin".to_string(),
        ));
    }

    // convert to sea_orm::Date (chrono::NaiveDate)
    let date_inicio_date: Date = date_inicio;
    let date_fin_date: Date = date_fin;

    let total_sales = schemas::sale::Entity::find()
        .filter(
            schemas::sale::Column::DateSale
                .between(date_inicio_date.clone(), date_fin_date.clone()),
        )
        .all(&app_ctx.conn)
        .await?
        .into_iter()
        .map(|venta| venta.total)
        .sum();

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
        .all(&app_ctx.conn)
        .await?
        .into_iter()
        .map(|venta| venta.total)
        .sum();

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
    info!(
        "Calculating total sales for username: {} between dates: {} and {}",
        payload.username.clone().unwrap_or_default(),
        payload.date_inicio,
        payload.date_fin
    );

    // parse date_inicio/date_fin (expecting YYYY-MM-DD)
    let date_inicio = match chrono::NaiveDate::parse_from_str(&payload.date_inicio, "%Y-%m-%d") {
        Ok(d) => d,
        Err(e) => {
            return Err(ApiError::ValidationError(format!(
                "Invalid date_inicio: {}",
                e
            )));
        }
    };

    let date_fin = match chrono::NaiveDate::parse_from_str(&payload.date_fin, "%Y-%m-%d") {
        Ok(d) => d,
        Err(e) => {
            return Err(ApiError::ValidationError(format!(
                "Invalid date_fin: {}",
                e
            )));
        }
    };

    if date_inicio > date_fin {
        return Err(ApiError::ValidationError(
            "date_inicio must be <= date_fin".to_string(),
        ));
    }

    // convert to sea_orm::Date (chrono::NaiveDate)
    let date_inicio_date: Date = date_inicio;
    let date_fin_date: Date = date_fin;

    let total_sales = schemas::sale::Entity::find()
        .filter(
            schemas::sale::Column::DateSale
                .between(date_inicio_date.clone(), date_fin_date.clone()),
        )
        .filter(schemas::sale::Column::Username.eq(payload.username.clone().unwrap_or_default()))
        .filter(schemas::sale::Column::Status.ne("CANCEL".to_string()))
        .all(&app_ctx.conn)
        .await?
        .into_iter()
        .map(|venta| venta.total)
        .sum();

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
    info!(
        "Fetching sale details between dates: {} and {}",
        payload.date_inicio, payload.date_fin
    );

    // parse date_inicio/date_fin (expecting YYYY-MM-DD)
    let date_inicio = match chrono::NaiveDate::parse_from_str(&payload.date_inicio, "%Y-%m-%d") {
        Ok(d) => d,
        Err(e) => {
            return Err(ApiError::ValidationError(format!(
                "Invalid date_inicio: {}",
                e
            )));
        }
    };

    let date_fin = match chrono::NaiveDate::parse_from_str(&payload.date_fin, "%Y-%m-%d") {
        Ok(d) => d,
        Err(e) => {
            return Err(ApiError::ValidationError(format!(
                "Invalid date_fin: {}",
                e
            )));
        }
    };

    if date_inicio > date_fin {
        return Err(ApiError::ValidationError(
            "date_inicio must be <= date_fin".to_string(),
        ));
    }

    // convert to sea_orm::Date (chrono::NaiveDate)
    let date_inicio_date: Date = date_inicio;
    let date_fin_date: Date = date_fin;

    let ventas = schemas::sale::Entity::find()
        .filter(
            schemas::sale::Column::DateSale
                .between(date_inicio_date.clone(), date_fin_date.clone()),
        )
        .filter(schemas::sale::Column::Status.ne("CANCEL".to_string()))
        .all(&app_ctx.conn)
        .await?;

    let total = ventas.len() as i32;

    Ok(Json(ApiResponse::new(
        ventas
            .into_iter()
            .map(SalesResponseDTO::from)
            .collect::<Vec<_>>(),
        total,
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
        .all(&app_ctx.conn)
        .await?;

    let total = detalles.len() as i32;

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
