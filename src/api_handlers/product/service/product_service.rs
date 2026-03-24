use crate::{
    api_handlers::product::dto::product_dto::{
        ProductRequestCount, ProductRequestDTO, ProductRequestPrice, ProductResponse, TotalProducts,
    },
    api_utils::{
        api_error::ApiError,
        api_response::{ApiResponse, PaginationParams, PaginationParamsProductName},
    },
    config::config_database::config_db_context::AppContext,
};
use axum::{
    Json,
    extract::{Path, Query, State},
};
use log::info;

use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, DbBackend, EntityTrait, FromQueryResult,
    IntoActiveModel, ModelTrait, PaginatorTrait, QueryFilter, QueryOrder, Statement,
};
use validator::Validate;

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
            1,
        ))),
        Err(_) => Err(ApiError::NotFound),
    }
}

pub async fn get_product_by_cod_bar(
    State(app_context): State<AppContext>,
    Query(codigo_bar): Query<PaginationParamsProductName>,
) -> Result<Json<ApiResponse<Vec<ProductResponse>>>, ApiError> {
    info!("Fetching product with COD BAR: {}", codigo_bar.product_name);

    let product = schemas::product::Entity::find()
        .filter(schemas::product::Column::ProductCodeBar.eq(codigo_bar.product_name.clone()))
        .all(&app_context.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    let total_products = TotalProducts::find_by_statement(Statement::from_sql_and_values(
        DbBackend::Postgres,
        r#"SELECT count(product_id) as total FROM product where product_code_bar = $1"#,
        [codigo_bar.product_name.clone().into()],
    ))
    .one(&app_context.conn)
    .await
    .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    let product_response: Vec<ProductResponse> = product.into_iter().map(Into::into).collect();

    Ok(Json(ApiResponse::success(
        product_response,
        "Product retrieved successfully".to_string(),
        total_products.map(|t| t.total).unwrap_or(0) as i32,
    )))
}

pub async fn get_all_product(
    State(app_context): State<AppContext>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<ApiResponse<Vec<ProductResponse>>>, ApiError> {
    info!("Fetching product all");

    let product_list = schemas::product::Entity::find()
        .order_by_asc(schemas::product::Column::ProductId)
        .paginate(&app_context.conn, pagination.limit)
        .fetch_page(pagination.page)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    let total_products = TotalProducts::find_by_statement(Statement::from_sql_and_values(
        DbBackend::Postgres,
        r#"SELECT count(product_id) as total FROM product"#,
        [],
    ))
    .one(&app_context.conn)
    .await
    .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    let max_total_product = total_products.map(|t| t.total).unwrap_or(0);

    Ok(Json(ApiResponse::success(
        product_list.into_iter().map(Into::into).collect(),
        "Product retrieved successfully".to_string(),
        max_total_product as i32,
    )))
}

pub async fn create_new_product(
    State(app_context): State<AppContext>,
    Json(payload): Json<ProductRequestDTO>,
) -> Result<Json<ApiResponse<ProductResponse>>, ApiError> {
    info!("fn create new product ");

    payload.validate().map_err(ApiError::Validation)?;

    let new_user = schemas::product::ActiveModel {
        product_id: ActiveValue::NotSet,
        product_name: ActiveValue::Set(payload.product_name),
        product_catalog: ActiveValue::Set(payload.product_catalog),
        product_count: ActiveValue::Set(payload.product_count),
        product_code_bar: ActiveValue::Set(payload.product_code_bar),
        product_price: ActiveValue::Set(payload.product_price),
        product_desc: ActiveValue::Set(payload.product_desc),
        product_lote: ActiveValue::Set(payload.product_lote),
        product_date: ActiveValue::NotSet,
        product_lastmdate: ActiveValue::NotSet,
    }
    .save(&app_context.conn)
    .await?;

    Ok(Json(ApiResponse::success(
        new_user.into(),
        "message".to_string(),
        1,
    )))
}

pub async fn delete_product(
    State(app_context): State<AppContext>,
    Path(product_id): Path<i64>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    info!("fn delete product by id: {} ", product_id);

    let product = schemas::product::Entity::find_by_id(product_id)
        .one(&app_context.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?
        .ok_or_else(|| ApiError::NotFound)?;

    product
        .delete(&app_context.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    Ok(Json(ApiResponse::success(
        (),
        "Product delete correct".to_string(),
        1,
    )))
}

pub async fn update_product(
    State(app_context): State<AppContext>,
    Path(product_id): Path<i64>,
    Json(payload): Json<ProductRequestDTO>,
) -> Result<Json<ApiResponse<ProductResponse>>, ApiError> {
    info!("fn update_product by id: {} ", product_id);

    let product = schemas::product::Entity::find_by_id(product_id)
        .one(&app_context.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?
        .ok_or_else(|| ApiError::NotFound)?;

    let mut update_product = product.into_active_model();

    update_product.product_count = ActiveValue::Set(payload.product_count);
    update_product.product_name = ActiveValue::Set(payload.product_name);
    update_product.product_desc = ActiveValue::Set(payload.product_desc);

    let new_update_product = update_product.save(&app_context.conn).await?;

    Ok(Json(ApiResponse::success(
        new_update_product.into(),
        "Update Correcto".to_string(),
        1,
    )))
}

pub async fn update_product_price(
    State(app_context): State<AppContext>,
    Path(product_id): Path<i64>,
    Json(payload): Json<ProductRequestPrice>,
) -> Result<Json<ApiResponse<ProductResponse>>, ApiError> {
    info!("fn update_product_price by id: {} ", product_id);

    let product = schemas::product::Entity::find_by_id(product_id)
        .one(&app_context.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?
        .ok_or_else(|| ApiError::NotFound)?;

    let mut update_product = product.into_active_model();

    update_product.product_price = ActiveValue::Set(payload.product_price);

    let new_update_product = update_product.save(&app_context.conn).await?;

    Ok(Json(ApiResponse::success(
        new_update_product.into(),
        "Update Correcto".to_string(),
        1,
    )))
}

pub async fn update_product_count(
    State(app_context): State<AppContext>,
    Path(product_id): Path<i64>,
    Json(payload): Json<ProductRequestCount>,
) -> Result<Json<ApiResponse<ProductResponse>>, ApiError> {
    info!("fn update_product_price by id: {} ", product_id);

    let product = schemas::product::Entity::find_by_id(product_id)
        .one(&app_context.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?
        .ok_or_else(|| ApiError::NotFound)?;

    let mut update_product = product.into_active_model();

    update_product.product_count = ActiveValue::Set(payload.product_count);

    let new_update_product = update_product.save(&app_context.conn).await?;

    Ok(Json(ApiResponse::success(
        new_update_product.into(),
        "Update Correcto".to_string(),
        1,
    )))
}

pub async fn update_product_count_details(
    State(app_context): State<AppContext>,
    Path(product_id): Path<i64>,
    Json(payload): Json<ProductRequestCount>,
) -> Result<Json<ApiResponse<ProductResponse>>, ApiError> {
    info!("fn update_product_price by id: {} ", product_id);

    let product = schemas::product::Entity::find_by_id(product_id)
        .one(&app_context.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?
        .ok_or_else(|| ApiError::NotFound)?;

    //Validar si es Venta o Update para actulizar el numero de items
    let product_count_actual = if payload.product_code_bar == "0000000" {
        product.product_count + payload.product_count
    } else {
        payload.product_count
    };

    let mut update_product = product.into_active_model();

    update_product.product_count = ActiveValue::Set(product_count_actual);

    let new_update_product = update_product.save(&app_context.conn).await?;

    Ok(Json(ApiResponse::success(
        new_update_product.into(),
        "Update Correcto".to_string(),
        1,
    )))
}

/* Get Report de Lista de Usarios  */
pub async fn get_list_products(
    State(app_ctx): State<AppContext>,
    Path(product_name): Path<String>,
) -> Result<Json<ApiResponse<Vec<ProductResponse>>>, ApiError> {
    let report: Vec<ProductResponse> =
        ProductResponse::find_by_statement(Statement::from_sql_and_values(
            DbBackend::Postgres,
            r#"select product_id, product_name, product_count, product_code_bar, product_price, product_lastmdate from pharmacy.product where lower(product_name) like lower('$1%') "#,
            [
                    product_name.into(),
            ],
        ))
        .all(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    //SI esta vacio activar ApiError Data no encontrado

    let api_response = ApiResponse::new(
        report,
        1,
        "Report generated successfully".to_string(),
        "success".to_string(),
        200,
        chrono::Utc::now().to_rfc3339(),
    );

    Ok(Json(api_response))
}

pub async fn get_product_by_name_details(
    State(app_context): State<AppContext>,
    Query(pagination): Query<PaginationParamsProductName>,
) -> Result<Json<ApiResponse<Vec<ProductResponse>>>, ApiError> {
    info!(
        "Fetching product with product_name: {}",
        pagination.product_name
    );

    if pagination.product_name.len() == 0 {
        return Err(ApiError::NotFound);
    }

    // Use the paginator to fetch the requested page and obtain a consistent total
    // count for pagination (num_items) using the same filter.
    let paginator = schemas::product::Entity::find()
        .filter(schemas::product::Column::ProductName.starts_with(pagination.product_name.clone()))
        .order_by_asc(schemas::product::Column::ProductId)
        .paginate(&app_context.conn, pagination.limit);

    let product = paginator
        .fetch_page(pagination.page)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    let max_total_product = paginator
        .num_items()
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    if product.is_empty() {
        return Err(ApiError::NotFound);
    }

    Ok(Json(ApiResponse::success(
        product.into_iter().map(Into::into).collect(),
        "Product retrieved successfully".to_string(),
        max_total_product as i32,
    )))
}
