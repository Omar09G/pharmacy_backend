use axum::{
    Json,
    extract::{Path, State},
};
use log::info;

use crate::{
    api_handlers::sales::sales_dto::{SalesDetailDTO, SalesRequestDTO, SalesResponseIdDTO},
    api_utils::{api_error::ApiError, api_response::ApiResponse},
    config::config_database::config_db_context::AppContext,
};

use validator::Validate;

use sea_orm::{ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, QueryFilter};

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
) -> Result<Json<ApiResponse<SalesRequestDTO>>, ApiError> {
    info!("Fetching sale with ID: {}", sale_id);

    let venta = schemas::sale::Entity::find_by_id(sale_id)
        .one(&app_ctx.conn)
        .await?
        .ok_or(ApiError::NotFound)?;

    let detalles = schemas::saledetal::Entity::find()
        .filter(schemas::saledetal::Column::IdSale.eq(sale_id))
        .all(&app_ctx.conn)
        .await?;

    let response_dto = SalesRequestDTO {
        id: venta.id,
        date_sale: venta.date_sale,
        discount: venta.discount,
        id_sale_detl: venta.id_sale_detl,
        iva: venta.iva,
        msg: venta.msg.clone(),
        payment_method: venta.payment_method.clone(),
        payment_status: venta.payment_status.clone(),
        status: venta.status.clone(),
        sub_total: venta.sub_total,
        time_sale: venta.time_sale,
        total: venta.total,
        username: venta.username.clone(),
        details: detalles.into_iter().map(SalesDetailDTO::from).collect(),
    };

    Ok(Json(ApiResponse::new(
        response_dto,
        1,
        "Sale fetched successfully".to_string(),
        "success".to_string(),
        200,
        chrono::Utc::now().to_rfc3339(),
    )))
}
