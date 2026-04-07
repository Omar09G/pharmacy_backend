use axum::{
    Json,
    extract::{Path, Query, State},
};

use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, IntoActiveModel, ModelTrait,
    PaginatorTrait, QueryFilter, QueryOrder,
};
use validator::Validate;

use crate::api_module::customer_credit_accounts::customer_credit_accounts_dto::customer_credit_accounts_dto::{
    CustomerCreditAccountDetailResponse, CustomerCreditAccountIdResponse, CustomerCreditAccountRequest,
};
use crate::{
    api_utils::{
        api_error::ApiError,
        api_response::{ApiResponse, PaginationParams},
        api_utils_fun::{to_page_index, to_page_limit},
    },
    config::config_database::config_db_context::AppContext,
};

pub async fn create_customer_credit_account(
    State(app_ctx): State<AppContext>,
    Json(payload): Json<CustomerCreditAccountRequest>,
) -> Result<Json<ApiResponse<CustomerCreditAccountIdResponse>>, ApiError> {
    payload.validate().map_err(ApiError::Validation)?;

    let cca_create = schemas::customer_credit_accounts::ActiveModel::try_from(payload)
        .map_err(|e| ApiError::Unexpected(Box::new(std::io::Error::other(e))))?;

    let new_cca = cca_create
        .save(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    if new_cca.id.is_not_set() {
        return Err(ApiError::ValidationError(
            "Failed to create customer credit account".to_string(),
        ));
    }

    Ok(Json(ApiResponse::success(
        CustomerCreditAccountIdResponse::from(new_cca),
        "Customer credit account created successfully".to_string(),
        1,
    )))
}

pub async fn get_customer_credit_account_by_id(
    State(app_ctx): State<AppContext>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<CustomerCreditAccountDetailResponse>>, ApiError> {
    let cca = schemas::customer_credit_accounts::Entity::find_by_id(id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    match cca {
        Some(cca) => Ok(Json(ApiResponse::success(
            CustomerCreditAccountDetailResponse::from(cca),
            "Customer credit account retrieved successfully".to_string(),
            1,
        ))),
        None => Err(ApiError::ValidationError(
            "Customer credit account not found".to_string(),
        )),
    }
}

pub async fn get_customer_credit_accounts(
    State(app_ctx): State<AppContext>,
    Query(pagination): Query<PaginationParams>,
) -> Result<Json<ApiResponse<Vec<CustomerCreditAccountDetailResponse>>>, ApiError> {
    let page_index = to_page_index(pagination.page);
    let page_limit = to_page_limit(pagination.limit);

    let mut select = schemas::customer_credit_accounts::Entity::find();

    if let Some(customer) = pagination.customer_id {
        select = select.filter(schemas::customer_credit_accounts::Column::CustomerId.eq(customer));
    }

    if let Some(id) = pagination.id {
        select = select.filter(schemas::customer_credit_accounts::Column::Id.eq(id));
    }

    let paginator = select
        .order_by_asc(schemas::customer_credit_accounts::Column::Id)
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
    

    let items = paginator
        .fetch_page(page_index)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    Ok(Json(ApiResponse::success(
        items
            .into_iter()
            .map(CustomerCreditAccountDetailResponse::from)
            .collect(),
        "Customer credit accounts retrieved successfully".to_string(),
        total_items as i32,
    )))
}

pub async fn delete_customer_credit_account(
    State(app_ctx): State<AppContext>,
    Path(id): Path<i64>,
) -> Result<Json<ApiResponse<()>>, ApiError> {
    let cca = schemas::customer_credit_accounts::Entity::find_by_id(id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    match cca {
        Some(cca) => {
            cca.delete(&app_ctx.conn)
                .await
                .map_err(|e| ApiError::Unexpected(Box::new(e)))?;
            Ok(Json(ApiResponse::success(
                (),
                "Customer credit account deleted successfully".to_string(),
                0,
            )))
        }
        None => Err(ApiError::ValidationError(
            "Customer credit account not found".to_string(),
        )),
    }
}

pub async fn update_customer_credit_account(
    State(app_ctx): State<AppContext>,
    Json(payload): Json<CustomerCreditAccountRequest>,
) -> Result<Json<ApiResponse<CustomerCreditAccountIdResponse>>, ApiError> {
    payload.validate().map_err(ApiError::Validation)?;

    let cca = schemas::customer_credit_accounts::Entity::find_by_id(payload.id)
        .one(&app_ctx.conn)
        .await
        .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

    match cca {
        Some(cca) => {
            let mut cca_active = cca.into_active_model();

            cca_active.customer_id = ActiveValue::Set(payload.customer_id);
            cca_active.balance = ActiveValue::Set(payload.balance);
            cca_active.limit_amount = ActiveValue::Set(payload.limit_amount);
            cca_active.last_overdue_date = ActiveValue::Set(payload.last_overdue_date);

            let updated = cca_active
                .save(&app_ctx.conn)
                .await
                .map_err(|e| ApiError::Unexpected(Box::new(e)))?;

            Ok(Json(ApiResponse::success(
                CustomerCreditAccountIdResponse::from(updated),
                "Customer credit account updated successfully".to_string(),
                0,
            )))
        }
        None => Err(ApiError::ValidationError(
            "Customer credit account not found".to_string(),
        )),
    }
}
