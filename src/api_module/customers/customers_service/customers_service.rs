use axum::{
    Json,
    extract::{Path, Query, State},
};

use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, IntoActiveModel, ModelTrait,
    PaginatorTrait, QueryFilter, QueryOrder,
};
use validator::Validate;

use crate::api_module::customers::customers_dto::customers_dto::{
    CustomerDetailResponse, CustomerIdResponse, CustomerRequest,
};
use crate::{
    api_utils::{
        api_error::ApiError,
        api_response::{ApiResponse, PaginationParams},
        api_utils_fun::{to_page_index, to_page_limit},
    },
    config::config_database::config_db_context::AppContext,
};

pub async fn create_customer(
    State(app_ctx): State<AppContext>,
    Json(payload): Json<CustomerRequest>,
) -> Result<Json<ApiResponse<CustomerIdResponse>>, ApiError> {
    payload.validate().map_err(ApiError::Validation)?;

    let customer_create = schemas::customers::ActiveModel::try_from(payload)
        .map_err(|e| ApiError::Unexpected(Box::new(std::io::Error::other(e))))?;

    if customer_create.name.is_not_set() {
        return Err(ApiError::Validation(validator::ValidationErrors::new()));
    }

    let new_customer = customer_create
        .save(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    if new_customer.id.is_not_set() {
        return Err(ApiError::ValidationError(
            "Failed to create customer".to_string(),
        ));
    }

    Ok(Json(ApiResponse::success(
        CustomerIdResponse::from(new_customer),
        "Customer created successfully".to_string(),
        1,
    )))
}

pub async fn get_customer_by_id(
    State(app_ctx): State<AppContext>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<CustomerDetailResponse>>, ApiError> {
    let customer = schemas::customers::Entity::find_by_id(id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    match customer {
        Some(customer) => Ok(Json(ApiResponse::success(
            CustomerDetailResponse::from(customer),
            "Customer retrieved successfully".to_string(),
            1,
        ))),
        None => Err(ApiError::ValidationError("Customer not found".to_string())),
    }
}

pub async fn get_customers(
    State(app_ctx): State<AppContext>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<ApiResponse<Vec<CustomerDetailResponse>>>, ApiError> {
    let page_index = to_page_index(pagination.page);
    let page_limit = to_page_limit(pagination.limit);

    let mut select = schemas::customers::Entity::find();

    // Apply optional filters
    if let Some(name_filter) = pagination.name.clone()
        && !name_filter.is_empty()
    {
        select = select.filter(schemas::customers::Column::Name.eq(name_filter));
    }

    if let Some(doc_filter) = pagination.document_id.clone()
        && !doc_filter.is_empty()
    {
        select = select.filter(schemas::customers::Column::DocumentId.eq(doc_filter));
    }

    if let Some(email_filter) = pagination.email.clone()
        && !email_filter.is_empty()
    {
        select = select.filter(schemas::customers::Column::Email.eq(email_filter));
    }

    if let Some(phone_filter) = pagination.phone.clone()
        && !phone_filter.is_empty()
    {
        select = select.filter(schemas::customers::Column::Phone.eq(phone_filter));
    }

    if let Some(status_filter) = pagination.status.clone()
        && !status_filter.is_empty()
    {
        select = select.filter(schemas::customers::Column::Status.eq(status_filter));
    }

    let paginator = select
        .order_by_asc(schemas::customers::Column::Id)
        .paginate(&app_ctx.conn, page_limit);

    let total_items = paginator
        .num_items()
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    let customers = paginator
        .fetch_page(page_index)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    Ok(Json(ApiResponse::success(
        customers
            .into_iter()
            .map(CustomerDetailResponse::from)
            .collect(),
        "Customers retrieved successfully".to_string(),
        total_items as i32,
    )))
}

pub async fn delete_customer(
    State(app_ctx): State<AppContext>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    let customer = schemas::customers::Entity::find_by_id(id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    match customer {
        Some(customer) => {
            customer
                .delete(&app_ctx.conn)
                .await
                .map_err(|e| ApiError::Unexpected(Box::new(e)))?;
            Ok(Json(ApiResponse::success(
                (),
                "Customer deleted successfully".to_string(),
                0,
            )))
        }
        None => Err(ApiError::ValidationError("Customer not found".to_string())),
    }
}

pub async fn get_customers_by_name(
    State(app_ctx): State<AppContext>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<ApiResponse<Vec<CustomerDetailResponse>>>, ApiError> {
    let page_index = to_page_index(pagination.page);
    let page_limit = to_page_limit(pagination.limit);
    let name_filter = pagination.name.clone().unwrap_or_default();

    if name_filter.is_empty() {
        return Err(ApiError::ValidationError(
            "Name filter cannot be empty".to_string(),
        ));
    }

    let paginator = schemas::customers::Entity::find()
        .filter(schemas::customers::Column::Name.eq(name_filter))
        .order_by_asc(schemas::customers::Column::Id)
        .paginate(&app_ctx.conn, page_limit);

    let total_items = paginator
        .num_items()
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    let customers = paginator
        .fetch_page(page_index)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    Ok(Json(ApiResponse::success(
        customers
            .into_iter()
            .map(CustomerDetailResponse::from)
            .collect(),
        "Customers retrieved successfully".to_string(),
        total_items as i32,
    )))
}

pub async fn update_customer(
    State(app_ctx): State<AppContext>,
    Path(id): Path<i64>,
    Json(payload): Json<CustomerRequest>,
) -> Result<Json<ApiResponse<CustomerIdResponse>>, ApiError> {
    payload.validate().map_err(ApiError::Validation)?;

    let customer = schemas::customers::Entity::find_by_id(id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    match customer {
        Some(customer) => {
            let mut customer_active_model = customer.into_active_model();

            // name and status are required in the request DTO, assign directly
            customer_active_model.name = ActiveValue::Set(payload.name.clone());

            // Optional fields: wrap in Some(...) when setting into ActiveValue::Set
            if let Some(document_id) = payload.document_id {
                customer_active_model.document_id = ActiveValue::Set(Some(document_id));
            }
            if let Some(phone) = payload.phone {
                customer_active_model.phone = ActiveValue::Set(Some(phone));
            }
            if let Some(email) = payload.email {
                customer_active_model.email = ActiveValue::Set(Some(email));
            }
            if let Some(billing_address) = payload.billing_address {
                customer_active_model.billing_address = ActiveValue::Set(Some(billing_address));
            }
            if let Some(credit_limit) = payload.credit_limit {
                customer_active_model.credit_limit = ActiveValue::Set(Some(credit_limit));
            }
            if let Some(terms_days) = payload.terms_days {
                customer_active_model.terms_days = ActiveValue::Set(Some(terms_days));
            }

            // status is required in the request DTO
            customer_active_model.status = ActiveValue::Set(payload.status.clone());

            let updated_customer = customer_active_model
                .save(&app_ctx.conn)
                .await
                .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

            Ok(Json(ApiResponse::success(
                CustomerIdResponse::from(updated_customer),
                "Customer updated successfully".to_string(),
                0,
            )))
        }
        None => Err(ApiError::ValidationError("Customer not found".to_string())),
    }
}
