use sea_orm::ActiveValue;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::api_utils::api_utils_fun::get_current_timestamp_now;

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DiscountRequest {
    pub id: i64,
    pub code: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub discount_type: String,
    pub value: Decimal,
    pub applies_to: String,
    pub product_id: Option<i64>,
    pub category_id: Option<i64>,
    pub customer_id: Option<i64>,
    pub min_qty: Option<Decimal>,
    pub max_uses: Option<i64>,
    pub priority: Option<i32>,
    pub start_at: Option<DateTimeWithTimeZone>,
    pub end_at: Option<DateTimeWithTimeZone>,
    pub active: bool,
    pub created_at: DateTimeWithTimeZone,
    pub created_by: Option<i64>,
}

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DiscountIdResponse {
    pub id: i64,
}

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct DiscountDetailResponse {
    pub id: i64,
    pub code: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub discount_type: String,
    pub value: Decimal,
    pub applies_to: String,
    pub product_id: Option<i64>,
    pub category_id: Option<i64>,
    pub customer_id: Option<i64>,
    pub min_qty: Option<Decimal>,
    pub max_uses: Option<i64>,
    pub priority: Option<i32>,
    pub start_at: Option<DateTimeWithTimeZone>,
    pub end_at: Option<DateTimeWithTimeZone>,
    pub active: bool,
    pub created_at: DateTimeWithTimeZone,
    pub created_by: Option<i64>,
}

impl TryFrom<DiscountRequest> for schemas::discounts::ActiveModel {
    type Error = String;

    fn try_from(request: DiscountRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            id: ActiveValue::NotSet,
            code: ActiveValue::Set(request.code),
            name: ActiveValue::Set(request.name),
            description: ActiveValue::Set(request.description),
            discount_type: ActiveValue::Set(request.discount_type),
            value: ActiveValue::Set(request.value),
            applies_to: ActiveValue::Set(request.applies_to),
            product_id: ActiveValue::Set(request.product_id),
            category_id: ActiveValue::Set(request.category_id),
            customer_id: ActiveValue::Set(request.customer_id),
            min_qty: ActiveValue::Set(request.min_qty),
            max_uses: ActiveValue::Set(request.max_uses),
            priority: ActiveValue::Set(request.priority),
            start_at: ActiveValue::Set(Some(get_current_timestamp_now())),
            end_at: ActiveValue::Set(Some(get_current_timestamp_now())),
            active: ActiveValue::Set(request.active),
            created_at: ActiveValue::Set(get_current_timestamp_now()),
            created_by: ActiveValue::Set(request.created_by),
        })
    }
}

impl From<schemas::discounts::Model> for DiscountDetailResponse {
    fn from(model: schemas::discounts::Model) -> Self {
        Self {
            id: model.id,
            code: model.code,
            name: model.name,
            description: model.description,
            discount_type: model.discount_type,
            value: model.value,
            applies_to: model.applies_to,
            product_id: model.product_id,
            category_id: model.category_id,
            customer_id: model.customer_id,
            min_qty: model.min_qty,
            max_uses: model.max_uses,
            priority: model.priority,
            start_at: model.start_at,
            end_at: model.end_at,
            active: model.active,
            created_at: model.created_at,
            created_by: model.created_by,
        }
    }
}

impl From<schemas::discounts::ActiveModel> for DiscountDetailResponse {
    fn from(model: schemas::discounts::ActiveModel) -> Self {
        Self {
            id: model.id.unwrap(),
            code: model.code.unwrap(),
            name: model.name.unwrap(),
            description: model.description.unwrap(),
            discount_type: model.discount_type.unwrap(),
            value: model.value.unwrap(),
            applies_to: model.applies_to.unwrap(),
            product_id: model.product_id.unwrap(),
            category_id: model.category_id.unwrap(),
            customer_id: model.customer_id.unwrap(),
            min_qty: model.min_qty.unwrap(),
            max_uses: model.max_uses.unwrap(),
            priority: model.priority.unwrap(),
            start_at: model.start_at.unwrap(),
            end_at: model.end_at.unwrap(),
            active: model.active.unwrap(),
            created_at: model.created_at.unwrap(),
            created_by: model.created_by.unwrap(),
        }
    }
}

impl From<schemas::discounts::Model> for DiscountIdResponse {
    fn from(model: schemas::discounts::Model) -> Self {
        Self { id: model.id }
    }
}

impl From<schemas::discounts::ActiveModel> for DiscountIdResponse {
    fn from(model: schemas::discounts::ActiveModel) -> Self {
        Self {
            id: model.id.unwrap(),
        }
    }
}
