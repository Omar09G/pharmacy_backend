use sea_orm::ActiveValue;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::api_utils::api_utils_fun::{
    get_current_timestamp_at_zone_mexico, get_current_timestamp_now,
};

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ProductLotRequest {
    pub product_id: i64,
    pub lot_number: Option<String>,
    pub qty_on_hand: Decimal,
    pub expiry_date: Option<Date>,
    pub purchase_id: Option<i64>,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ProductLotIdResponse {
    pub id: i64,
}

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ProductLotDetailResponse {
    pub id: i64,
    pub product_id: i64,
    pub lot_number: Option<String>,
    pub qty_on_hand: Decimal,
    pub expiry_date: Option<Date>,
    pub purchase_id: Option<i64>,
    pub created_at: DateTimeWithTimeZone,
}

impl TryFrom<ProductLotRequest> for schemas::product_lots::ActiveModel {
    type Error = String;

    fn try_from(request: ProductLotRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            id: ActiveValue::NotSet,
            product_id: ActiveValue::Set(request.product_id),
            lot_number: ActiveValue::Set(request.lot_number),
            qty_on_hand: ActiveValue::Set(request.qty_on_hand),
            expiry_date: ActiveValue::NotSet,
            purchase_id: ActiveValue::Set(request.purchase_id),
            created_at: ActiveValue::Set(get_current_timestamp_now()),
        })
    }
}

impl From<schemas::product_lots::Model> for ProductLotDetailResponse {
    fn from(model: schemas::product_lots::Model) -> Self {
        Self {
            id: model.id,
            product_id: model.product_id,
            lot_number: model.lot_number,
            qty_on_hand: model.qty_on_hand,
            expiry_date: model.expiry_date,
            purchase_id: model.purchase_id,
            created_at: get_current_timestamp_at_zone_mexico(model.created_at),
        }
    }
}

impl From<schemas::product_lots::ActiveModel> for ProductLotDetailResponse {
    fn from(model: schemas::product_lots::ActiveModel) -> Self {
        Self {
            id: model.id.unwrap(),
            product_id: model.product_id.unwrap(),
            lot_number: model.lot_number.unwrap(),
            qty_on_hand: model.qty_on_hand.unwrap(),
            expiry_date: model.expiry_date.unwrap(),
            purchase_id: model.purchase_id.unwrap(),
            created_at: get_current_timestamp_at_zone_mexico(model.created_at.unwrap()),
        }
    }
}

impl From<schemas::product_lots::Model> for ProductLotIdResponse {
    fn from(model: schemas::product_lots::Model) -> Self {
        Self { id: model.id }
    }
}

impl From<schemas::product_lots::ActiveModel> for ProductLotIdResponse {
    fn from(model: schemas::product_lots::ActiveModel) -> Self {
        Self {
            id: model.id.unwrap(),
        }
    }
}
