use sea_orm::ActiveValue;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ProductPriceRequest {
    pub id: i64,
    pub product_id: i64,
    pub price_type: String,
    pub price: Decimal,
    pub starts_at: Option<DateTimeWithTimeZone>,
    pub ends_at: Option<DateTimeWithTimeZone>,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ProductPriceIdResponse {
    pub id: i64,
}

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ProductPriceDetailResponse {
    pub id: i64,
    pub product_id: i64,
    pub price_type: String,
    pub price: Decimal,
    pub starts_at: Option<DateTimeWithTimeZone>,
    pub ends_at: Option<DateTimeWithTimeZone>,
    pub created_at: DateTimeWithTimeZone,
}

impl From<ProductPriceRequest> for schemas::product_prices::ActiveModel {
    fn from(request: ProductPriceRequest) -> Self {
        Self {
            id: ActiveValue::NotSet,
            product_id: ActiveValue::Set(request.product_id),
            price_type: ActiveValue::Set(request.price_type),
            price: ActiveValue::Set(request.price),
            starts_at: ActiveValue::Set(request.starts_at),
            ends_at: ActiveValue::Set(request.ends_at),
            created_at: ActiveValue::Set(request.created_at),
        }
    }
}

impl From<schemas::product_prices::Model> for ProductPriceDetailResponse {
    fn from(model: schemas::product_prices::Model) -> Self {
        Self {
            id: model.id,
            product_id: model.product_id,
            price_type: model.price_type,
            price: model.price,
            starts_at: model.starts_at,
            ends_at: model.ends_at,
            created_at: model.created_at,
        }
    }
}

impl From<schemas::product_prices::ActiveModel> for ProductPriceDetailResponse {
    fn from(model: schemas::product_prices::ActiveModel) -> Self {
        Self {
            id: model.id.unwrap(),
            product_id: model.product_id.unwrap(),
            price_type: model.price_type.unwrap(),
            price: model.price.unwrap(),
            starts_at: model.starts_at.unwrap(),
            ends_at: model.ends_at.unwrap(),
            created_at: model.created_at.unwrap(),
        }
    }
}

impl From<schemas::product_prices::Model> for ProductPriceIdResponse {
    fn from(model: schemas::product_prices::Model) -> Self {
        Self { id: model.id }
    }
}

impl From<schemas::product_prices::ActiveModel> for ProductPriceIdResponse {
    fn from(model: schemas::product_prices::ActiveModel) -> Self {
        Self {
            id: model.id.unwrap(),
        }
    }
}
