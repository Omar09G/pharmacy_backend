use sea_orm::ActiveValue;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::api_utils::api_utils_fun::{
    get_current_timestamp_at_zone_mexico, get_current_timestamp_now,
};

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ProductDto {
    pub id: i64,
    pub sku: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub brand: Option<String>,
    pub category_id: Option<i64>,
    pub unit_id: Option<i64>,
    pub is_sellable: bool,
    pub track_batches: bool,
    pub tax_profile_id: Option<i64>,
    pub default_cost: Option<Decimal>,
    pub purchase_price: Option<Decimal>,
    pub wholesale_price: Option<Decimal>,
    pub sale_price: Option<Decimal>,
    pub default_price: Option<Decimal>,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: Option<DateTimeWithTimeZone>,
    pub deleted_at: Option<DateTimeWithTimeZone>,
}

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ProductRequest {
    pub id: i64,
    pub sku: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub brand: Option<String>,
    pub category_id: Option<i64>,
    pub unit_id: Option<i64>,
    pub is_sellable: bool,
    pub track_batches: bool,
    pub tax_profile_id: Option<i64>,
    pub default_cost: Option<Decimal>,
    pub purchase_price: Option<Decimal>,
    pub wholesale_price: Option<Decimal>,
    pub sale_price: Option<Decimal>,
    pub default_price: Option<Decimal>,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: Option<DateTimeWithTimeZone>,
    pub deleted_at: Option<DateTimeWithTimeZone>,
}

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ProductIdResponse {
    pub id: i64,
}

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ProductDetailResponse {
    pub id: i64,
    pub sku: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub brand: Option<String>,
    pub category_id: Option<i64>,
    pub unit_id: Option<i64>,
    pub is_sellable: bool,
    pub track_batches: bool,
    pub tax_profile_id: Option<i64>,
    pub default_cost: Option<Decimal>,
    pub purchase_price: Option<Decimal>,
    pub wholesale_price: Option<Decimal>,
    pub sale_price: Option<Decimal>,
    pub default_price: Option<Decimal>,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: Option<DateTimeWithTimeZone>,
    pub deleted_at: Option<DateTimeWithTimeZone>,
}

impl From<ProductRequest> for schemas::products::ActiveModel {
    fn from(request: ProductRequest) -> Self {
        Self {
            id: ActiveValue::NotSet,
            sku: ActiveValue::Set(request.sku),
            name: ActiveValue::Set(request.name),
            description: ActiveValue::Set(request.description),
            brand: ActiveValue::Set(request.brand),
            category_id: ActiveValue::Set(request.category_id),
            unit_id: ActiveValue::Set(request.unit_id),
            is_sellable: ActiveValue::Set(request.is_sellable),
            track_batches: ActiveValue::Set(request.track_batches),
            tax_profile_id: ActiveValue::Set(request.tax_profile_id),
            default_cost: ActiveValue::Set(request.default_cost),
            purchase_price: ActiveValue::Set(request.purchase_price),
            wholesale_price: ActiveValue::Set(request.wholesale_price),
            sale_price: ActiveValue::Set(request.sale_price),
            default_price: ActiveValue::Set(request.default_price),
            created_at: ActiveValue::Set(get_current_timestamp_now()),
            updated_at: ActiveValue::NotSet,
            deleted_at: ActiveValue::NotSet,
        }
    }
}

impl From<schemas::products::Model> for ProductDetailResponse {
    fn from(model: schemas::products::Model) -> Self {
        Self {
            id: model.id,
            sku: model.sku,
            name: model.name,
            description: model.description,
            brand: model.brand,
            category_id: model.category_id,
            unit_id: model.unit_id,
            is_sellable: model.is_sellable,
            track_batches: model.track_batches,
            tax_profile_id: model.tax_profile_id,
            default_cost: model.default_cost,
            purchase_price: model.purchase_price,
            wholesale_price: model.wholesale_price,
            sale_price: model.sale_price,
            default_price: model.default_price,
            created_at: get_current_timestamp_at_zone_mexico(model.created_at),
            updated_at: model.updated_at.map(get_current_timestamp_at_zone_mexico),
            deleted_at: model.deleted_at.map(get_current_timestamp_at_zone_mexico),
        }
    }
}

impl From<schemas::products::ActiveModel> for ProductDetailResponse {
    fn from(model: schemas::products::ActiveModel) -> Self {
        Self {
            id: model.id.unwrap(),
            sku: model.sku.unwrap(),
            name: model.name.unwrap(),
            description: model.description.unwrap(),
            brand: model.brand.unwrap(),
            category_id: model.category_id.unwrap(),
            unit_id: model.unit_id.unwrap(),
            is_sellable: model.is_sellable.unwrap(),
            track_batches: model.track_batches.unwrap(),
            tax_profile_id: model.tax_profile_id.unwrap(),
            default_cost: model.default_cost.unwrap(),
            purchase_price: model.purchase_price.unwrap(),
            wholesale_price: model.wholesale_price.unwrap(),
            sale_price: model.sale_price.unwrap(),
            default_price: model.default_price.unwrap(),
            created_at: get_current_timestamp_at_zone_mexico(model.created_at.unwrap()),
            updated_at: model
                .updated_at
                .unwrap()
                .map(get_current_timestamp_at_zone_mexico),
            deleted_at: model
                .deleted_at
                .unwrap()
                .map(get_current_timestamp_at_zone_mexico),
        }
    }
}

impl From<schemas::products::Model> for ProductIdResponse {
    fn from(model: schemas::products::Model) -> Self {
        Self { id: model.id }
    }
}

impl From<schemas::products::ActiveModel> for ProductIdResponse {
    fn from(model: schemas::products::ActiveModel) -> Self {
        Self {
            id: model.id.unwrap(),
        }
    }
}
