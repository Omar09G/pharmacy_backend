use sea_orm::ActiveValue;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::api_utils::api_utils_fun::get_current_timestamp_now;

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ProductBarcodeDto {
    pub product_id: i64,
    pub barcode: String,
    pub barcode_type: Option<String>,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ProductBarcodeRequest {
    pub product_id: i64,
    pub barcode: String,
    pub barcode_type: Option<String>,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ProductBarcodeIdResponse {
    pub id: i64,
}

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ProductBarcodeDetailResponse {
    pub id: i64,
    pub product_id: i64,
    pub barcode: String,
    pub barcode_type: Option<String>,
    pub created_at: DateTimeWithTimeZone,
}

impl From<ProductBarcodeRequest> for schemas::product_barcodes::ActiveModel {
    fn from(request: ProductBarcodeRequest) -> Self {
        Self {
            id: ActiveValue::NotSet,
            product_id: ActiveValue::Set(request.product_id),
            barcode: ActiveValue::Set(request.barcode),
            barcode_type: ActiveValue::Set(request.barcode_type),
            created_at: ActiveValue::Set(get_current_timestamp_now()),
        }
    }
}

impl From<schemas::product_barcodes::Model> for ProductBarcodeDetailResponse {
    fn from(model: schemas::product_barcodes::Model) -> Self {
        Self {
            id: model.id,
            product_id: model.product_id,
            barcode: model.barcode,
            barcode_type: model.barcode_type,
            created_at: model.created_at,
        }
    }
}

impl From<schemas::product_barcodes::ActiveModel> for ProductBarcodeDetailResponse {
    fn from(model: schemas::product_barcodes::ActiveModel) -> Self {
        Self {
            id: model.id.unwrap(),
            product_id: model.product_id.unwrap(),
            barcode: model.barcode.unwrap(),
            barcode_type: model.barcode_type.unwrap(),
            created_at: model.created_at.unwrap(),
        }
    }
}

impl From<schemas::product_barcodes::Model> for ProductBarcodeIdResponse {
    fn from(model: schemas::product_barcodes::Model) -> Self {
        Self { id: model.id }
    }
}

impl From<schemas::product_barcodes::ActiveModel> for ProductBarcodeIdResponse {
    fn from(model: schemas::product_barcodes::ActiveModel) -> Self {
        Self {
            id: model.id.unwrap(),
        }
    }
}
