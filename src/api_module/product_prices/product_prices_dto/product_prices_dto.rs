use sea_orm::ActiveValue;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::api_utils::api_utils_fun::{
    get_current_timestamp_at_zone_mexico, get_current_timestamp_now,
};

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

impl TryFrom<ProductPriceRequest> for schemas::product_prices::ActiveModel {
    type Error = String;

    fn try_from(request: ProductPriceRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            id: ActiveValue::NotSet,
            product_id: ActiveValue::Set(request.product_id),
            price_type: ActiveValue::Set(request.price_type),
            price: ActiveValue::Set(request.price),
            starts_at: ActiveValue::Set(Some(get_current_timestamp_now())),
            ends_at: ActiveValue::NotSet,
            created_at: ActiveValue::Set(get_current_timestamp_now()),
        })
    }
}

impl From<schemas::product_prices::Model> for ProductPriceDetailResponse {
    fn from(model: schemas::product_prices::Model) -> Self {
        Self {
            id: model.id,
            product_id: model.product_id,
            price_type: model.price_type,
            price: model.price,
            starts_at: model.starts_at.map(get_current_timestamp_at_zone_mexico),
            ends_at: model.ends_at.map(get_current_timestamp_at_zone_mexico),
            created_at: get_current_timestamp_at_zone_mexico(model.created_at),
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
            starts_at: model
                .starts_at
                .unwrap()
                .map(get_current_timestamp_at_zone_mexico),
            ends_at: model
                .ends_at
                .unwrap()
                .map(get_current_timestamp_at_zone_mexico),
            created_at: get_current_timestamp_at_zone_mexico(model.created_at.unwrap()),
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
