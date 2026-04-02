use axum::{
    Json,
    extract::{Path, Query, State},
};

use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, IntoActiveModel, ModelTrait,
    PaginatorTrait, QueryFilter, QueryOrder,
};
use validator::Validate;

use crate::api_module::suppliers::suppliers_dto::suppliers_dto::{
    SupplierDetailResponse, SupplierIdResponse, SupplierRequest,
};
use crate::{
    api_utils::{
        api_error::ApiError,
        api_response::{ApiResponse, PaginationParams},
        api_utils_fun::{to_page_index, to_page_limit},
    },
    config::config_database::config_db_context::AppContext,
};

pub async fn create_supplier(
    State(app_ctx): State<AppContext>,
    Json(payload): Json<SupplierRequest>,
) -> Result<Json<ApiResponse<SupplierIdResponse>>, ApiError> {
    payload.validate().map_err(ApiError::Validation)?;

    let supplier_create = schemas::suppliers::ActiveModel::from(payload);

    if supplier_create.name.is_not_set() {
        return Err(ApiError::Validation(validator::ValidationErrors::new()));
    }

    let new_supplier = supplier_create
        .save(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    if new_supplier.id.is_not_set() {
        return Err(ApiError::ValidationError(
            "Failed to create supplier".to_string(),
        ));
    }

    Ok(Json(ApiResponse::success(
        SupplierIdResponse::from(new_supplier),
        "Supplier created successfully".to_string(),
        1,
    )))
}

pub async fn get_supplier_by_id(
    State(app_ctx): State<AppContext>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<SupplierDetailResponse>>, ApiError> {
    let supplier = schemas::suppliers::Entity::find_by_id(id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    match supplier {
        Some(supplier) => Ok(Json(ApiResponse::success(
            SupplierDetailResponse::from(supplier),
            "Supplier retrieved successfully".to_string(),
            1,
        ))),
        None => Err(ApiError::ValidationError("Supplier not found".to_string())),
    }
}

pub async fn get_suppliers(
    State(app_ctx): State<AppContext>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<ApiResponse<Vec<SupplierDetailResponse>>>, ApiError> {
    let page_index = to_page_index(pagination.page);
    let page_limit = to_page_limit(pagination.limit);

    let mut select = schemas::suppliers::Entity::find();

    // Apply optional filters
    if let Some(name_filter) = pagination.name.clone() {
        if !name_filter.is_empty() {
            select = select.filter(schemas::suppliers::Column::Name.eq(name_filter));
        }
    }

    if let Some(tax_id_filter) = pagination.username.clone() {
        // reuse username field to allow tax_id search if provided
        if !tax_id_filter.is_empty() {
            select = select.filter(schemas::suppliers::Column::TaxId.eq(tax_id_filter));
        }
    }

    let paginator = select
        .order_by_asc(schemas::suppliers::Column::Id)
        .paginate(&app_ctx.conn, page_limit);

    let total_items = paginator
        .num_items()
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    let suppliers = paginator
        .fetch_page(page_index)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    Ok(Json(ApiResponse::success(
        suppliers
            .into_iter()
            .map(SupplierDetailResponse::from)
            .collect(),
        "Suppliers retrieved successfully".to_string(),
        total_items as i32,
    )))
}

pub async fn delete_supplier(
    State(app_ctx): State<AppContext>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    let supplier = schemas::suppliers::Entity::find_by_id(id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    match supplier {
        Some(supplier) => {
            supplier
                .delete(&app_ctx.conn)
                .await
                .map_err(|e| ApiError::Unexpected(Box::new(e)))?;
            Ok(Json(ApiResponse::success(
                (),
                "Supplier deleted successfully".to_string(),
                0,
            )))
        }
        None => Err(ApiError::ValidationError("Supplier not found".to_string())),
    }
}

pub async fn get_suppliers_by_name(
    State(app_ctx): State<AppContext>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<ApiResponse<Vec<SupplierDetailResponse>>>, ApiError> {
    let page_index = to_page_index(pagination.page);
    let page_limit = to_page_limit(pagination.limit);
    let name_filter = pagination.name.clone().unwrap_or_default();

    if name_filter.is_empty() {
        return Err(ApiError::ValidationError(
            "Name filter cannot be empty".to_string(),
        ));
    }

    let paginator = schemas::suppliers::Entity::find()
        .filter(schemas::suppliers::Column::Name.eq(name_filter))
        .order_by_asc(schemas::suppliers::Column::Id)
        .paginate(&app_ctx.conn, page_limit);

    let total_items = paginator
        .num_items()
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    let suppliers = paginator
        .fetch_page(page_index)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    Ok(Json(ApiResponse::success(
        suppliers
            .into_iter()
            .map(SupplierDetailResponse::from)
            .collect(),
        "Suppliers retrieved successfully".to_string(),
        total_items as i32,
    )))
}

pub async fn update_supplier(
    State(app_ctx): State<AppContext>,
    Path(id): Path<i64>,
    Json(payload): Json<SupplierRequest>,
) -> Result<Json<ApiResponse<SupplierIdResponse>>, ApiError> {
    payload.validate().map_err(ApiError::Validation)?;

    let supplier = schemas::suppliers::Entity::find_by_id(id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    match supplier {
        Some(supplier) => {
            let mut supplier_active_model = supplier.into_active_model();

            supplier_active_model.name = ActiveValue::Set(payload.name);
            supplier_active_model.tax_id = ActiveValue::Set(payload.tax_id);
            supplier_active_model.contact_person = ActiveValue::Set(payload.contact_person);
            supplier_active_model.phone = ActiveValue::Set(payload.phone);
            supplier_active_model.email = ActiveValue::Set(payload.email);
            supplier_active_model.address = ActiveValue::Set(payload.address);
            supplier_active_model.notes = ActiveValue::Set(payload.notes);

            let updated_supplier = supplier_active_model
                .save(&app_ctx.conn)
                .await
                .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

            Ok(Json(ApiResponse::success(
                SupplierIdResponse::from(updated_supplier),
                "Supplier updated successfully".to_string(),
                0,
            )))
        }
        None => Err(ApiError::ValidationError("Supplier not found".to_string())),
    }
}
