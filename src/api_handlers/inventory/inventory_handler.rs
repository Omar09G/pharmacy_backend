use axum::{
    Json,
    extract::{Query, State},
};
use log::{info, warn};

use sea_orm::IntoActiveModel;

use crate::{
    api_handlers::inventory::inventory_dto::{ParamsInventoryProduct, ResponseInventoryProduct},
    api_utils::{api_error::ApiError, api_response::ApiResponse},
    config::config_database::config_db_context::AppContext,
};

use sea_orm::{ActiveModelTrait, ActiveValue, EntityTrait};
/*
Nombre: update_inventory_product_by_price
Descripción: Actualiza el stok  de un producto en el inventario.
Parámetros:
- State(app_ctx): Contexto de la aplicación que contiene la conexión a la base de datos
- Query(payload): Parámetros de la consulta que incluyen el ID del producto y el nuevo precio
Retorno: Un Json con la respuesta de la API que incluye el ID del producto actualizado,
un mensaje de éxito, y el total de productos actualizados (en este caso siempre será 1)

*/
pub async fn update_inventory_product_by_count(
    State(app_ctx): State<AppContext>,
    Query(payload): Query<ParamsInventoryProduct>,
) -> Result<Json<ApiResponse<ResponseInventoryProduct>>, ApiError> {
    let product_id = payload
        .product_id
        .ok_or_else(|| ApiError::ValidationError("Product ID is required".to_string()))?;

    let product_cont = payload
        .product_count
        .ok_or_else(|| ApiError::ValidationError("Product count is required".to_string()))?;

    info!("Updating inventory for product ID: {}", product_id);
    warn!(
        "Product count: {} for product ID: {}",
        product_cont, product_id
    );

    let product = schemas::product::Entity::find_by_id(product_id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?
        .ok_or_else(|| ApiError::NotFound)?;

    let total_count = product.product_count + product_cont;

    info!(
        "Total count for product ID {} after update: {}, after adding count: {}",
        product_id, total_count, product_cont
    );

    let mut update_product = product.into_active_model();

    update_product.product_count = ActiveValue::Set(total_count);

    update_product.save(&app_ctx.conn).await?;

    Ok(Json(ApiResponse::success(
        ResponseInventoryProduct::new(product_id),
        "Producto actualizado exitosamente".to_string(),
        1,
    )))
}
/*
Nombre: update_inventory_product_by_price
Descripción: Actualiza el precio de un producto en el inventario. Requiere el ID del producto y el nuevo precio. Valida que ambos campos estén presentes y actualiza el precio en la base de datos. Devuelve una respuesta con el ID del producto actualizado y un mensaje de éxito
Parametros:
- product_id: ID del producto a actualizar (requerido)
- product_price: Nuevo precio del producto (requerido)
Respuesta:
- ResponseInventoryProduct: Contiene el ID del producto actualizado
- ApiResponse: Contiene el mensaje de éxito, el estado y el código de error (si aplica)
*/
pub async fn update_inventory_product_by_price(
    State(app_ctx): State<AppContext>,
    Query(payload): Query<ParamsInventoryProduct>,
) -> Result<Json<ApiResponse<ResponseInventoryProduct>>, ApiError> {
    let product_id = payload
        .product_id
        .ok_or_else(|| ApiError::ValidationError("Product ID is required".to_string()))?;

    let product_price = payload
        .product_price
        .ok_or_else(|| ApiError::ValidationError("Product price is required".to_string()))?;

    info!("Updating inventory for product ID: {}", product_id);
    warn!(
        "Product price: {} for product ID: {}",
        product_price, product_id
    );

    let product = schemas::product::Entity::find_by_id(product_id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?
        .ok_or_else(|| ApiError::NotFound)?;

    info!(
        "Total price for product ID {} after update: {}, after adding price: {}",
        product_id, product.product_price, product_price
    );

    let mut update_product = product.into_active_model();

    update_product.product_price = ActiveValue::Set(product_price);

    update_product.save(&app_ctx.conn).await?;

    Ok(Json(ApiResponse::success(
        ResponseInventoryProduct::new(product_id),
        "Producto actualizado exitosamente".to_string(),
        1,
    )))
}
