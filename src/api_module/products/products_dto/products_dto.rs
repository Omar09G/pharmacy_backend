use sea_orm::ActiveValue;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    api_module::{
        product_barcodes::product_barcodes_dto::product_barcodes_dto::ProductBarcodeDetailResponse,
        product_lots::ProductLotDetailResponse, product_prices::ProductPriceDetailResponse,
    },
    api_utils::api_utils_fun::get_current_timestamp_now,
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

impl TryFrom<ProductRequest> for schemas::products::ActiveModel {
    type Error = String;

    fn try_from(request: ProductRequest) -> Result<Self, Self::Error> {
        Ok(Self {
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
        })
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
            created_at: model.created_at,
            updated_at: model.updated_at,
            deleted_at: model.deleted_at,
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
            created_at: model.created_at.unwrap(),
            updated_at: model.updated_at.unwrap(),
            deleted_at: model.deleted_at.unwrap(),
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

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ProductResponse {
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
    pub lots_detail: ProductLotDetailResponse,
    pub prices_detail: ProductPriceDetailResponse,
    pub barcodes_detail: ProductBarcodeDetailResponse,
}

impl
    From<(
        schemas::products::Model,
        schemas::product_barcodes::Model,
        schemas::product_lots::Model,
        schemas::product_prices::Model,
    )> for ProductResponse
{
    fn from(
        (product_model, barcode_model, lot_model, price_model): (
            schemas::products::Model,
            schemas::product_barcodes::Model,
            schemas::product_lots::Model,
            schemas::product_prices::Model,
        ),
    ) -> Self {
        Self {
            id: product_model.id,
            sku: product_model.sku,
            name: product_model.name,
            description: product_model.description,
            brand: product_model.brand,
            category_id: product_model.category_id,
            unit_id: product_model.unit_id,
            is_sellable: product_model.is_sellable,
            track_batches: product_model.track_batches,
            tax_profile_id: product_model.tax_profile_id,
            default_cost: product_model.default_cost,
            purchase_price: product_model.purchase_price,
            wholesale_price: product_model.wholesale_price,
            sale_price: product_model.sale_price,
            default_price: product_model.default_price,
            created_at: product_model.created_at,
            updated_at: product_model.updated_at,
            deleted_at: product_model.deleted_at,
            lots_detail: ProductLotDetailResponse::from(lot_model),
            prices_detail: ProductPriceDetailResponse::from(price_model),
            barcodes_detail: ProductBarcodeDetailResponse::from(barcode_model),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Validate, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProductLotRequestAdd {
    pub lot_number: Option<String>,
    pub qty_on_hand: Decimal,
    pub expiry_date: Option<Date>,
    pub purchase_id: Option<i64>,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Deserialize, Serialize, Debug, Validate, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProductPriceRequestAdd {
    pub price_type: String,
    pub price: Decimal,
    pub starts_at: Option<DateTimeWithTimeZone>,
    pub ends_at: Option<DateTimeWithTimeZone>,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Deserialize, Serialize, Debug, Validate, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProductBarcodeRequestAdd {
    pub barcode: String,
    pub barcode_type: Option<String>,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Deserialize, Serialize, Debug, Validate, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProductRequestDetail {
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
    pub lots_detail: ProductLotRequestAdd,
    pub prices_detail: ProductPriceRequestAdd,
    pub barcodes_detail: ProductBarcodeRequestAdd,
}

//Pasar de ProductRequestDetail a ActiveModel de cada entidad relacionada (ProductLot, ProductPrice, ProductBarcode) y luego usar esos ActiveModel para crear las relaciones con el producto al momento de crear un nuevo producto.
impl TryFrom<ProductRequestDetail> for schemas::products::ActiveModel {
    type Error = String;

    fn try_from(request: ProductRequestDetail) -> Result<Self, Self::Error> {
        Ok(Self {
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
        })
    }
}

impl ProductBarcodeRequestAdd {
    pub fn into_active_model(self, product_id: i64) -> schemas::product_barcodes::ActiveModel {
        schemas::product_barcodes::ActiveModel {
            id: ActiveValue::NotSet,
            product_id: ActiveValue::Set(product_id),
            barcode: ActiveValue::Set(self.barcode),
            barcode_type: ActiveValue::Set(self.barcode_type),
            created_at: ActiveValue::Set(get_current_timestamp_now()),
        }
    }
}

impl ProductLotRequestAdd {
    pub fn into_active_model(self, product_id: i64) -> schemas::product_lots::ActiveModel {
        schemas::product_lots::ActiveModel {
            id: ActiveValue::NotSet,
            product_id: ActiveValue::Set(product_id),
            lot_number: ActiveValue::Set(self.lot_number),
            qty_on_hand: ActiveValue::Set(self.qty_on_hand),
            expiry_date: ActiveValue::Set(self.expiry_date),
            purchase_id: ActiveValue::Set(self.purchase_id),
            created_at: ActiveValue::Set(get_current_timestamp_now()),
        }
    }
}

impl ProductPriceRequestAdd {
    pub fn into_active_model(self, product_id: i64) -> schemas::product_prices::ActiveModel {
        schemas::product_prices::ActiveModel {
            id: ActiveValue::NotSet,
            product_id: ActiveValue::Set(product_id),
            price_type: ActiveValue::Set(self.price_type),
            price: ActiveValue::Set(self.price),
            starts_at: ActiveValue::Set(self.starts_at),
            ends_at: ActiveValue::Set(self.ends_at),
            created_at: ActiveValue::Set(get_current_timestamp_now()),
        }
    }
}
