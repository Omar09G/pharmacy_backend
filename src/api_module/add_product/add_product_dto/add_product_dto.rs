use crate::api_module::{
    product_barcodes::product_barcodes_dto::product_barcodes_dto::{
        ProductBarcodeDetailResponse, ProductBarcodeRequest,
    },
    product_lots::{ProductLotDetailResponse, ProductLotRequest},
    product_prices::{ProductPriceDetailResponse, ProductPriceRequest},
    products::products_dto::products_dto::{ProductDetailResponse, ProductRequest},
};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ProductAddRequest {
    pub id: i64,
    pub sku: Option<String>,
    pub name: String,
    pub barcode: String,
    pub barcode_type: Option<String>,
    pub description: Option<String>,
    pub lot_number: Option<String>,
    pub qty_on_hand: Decimal,
    pub expiry_date: Option<Date>,
    pub purchase_id: Option<i64>,
    pub price_type: String,
    pub price: Decimal,
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

/// Pasar de `&ProductAddRequest` a `ProductRequest` (usa referencia para no mover el payload)
impl From<&ProductAddRequest> for ProductRequest {
    fn from(request: &ProductAddRequest) -> Self {
        Self {
            id: request.id,
            sku: request.sku.clone(),
            name: request.name.clone(),
            description: request.description.clone(),
            brand: request.brand.clone(),
            category_id: request.category_id,
            unit_id: request.unit_id,
            is_sellable: request.is_sellable,
            track_batches: request.track_batches,
            tax_profile_id: request.tax_profile_id,
            default_cost: request.default_cost.clone(),
            purchase_price: request.purchase_price.clone(),
            wholesale_price: request.wholesale_price.clone(),
            sale_price: request.sale_price.clone(),
            default_price: request.default_price.clone(),
            created_at: request.created_at.clone(),
            updated_at: request.updated_at.clone(),
            deleted_at: request.deleted_at.clone(),
        }
    }
}
// Pasar de `( &ProductAddRequest, product_id )` a `ProductBarcodeRequest`
impl From<(&ProductAddRequest, i64)> for ProductBarcodeRequest {
    fn from((request, product_id): (&ProductAddRequest, i64)) -> Self {
        Self {
            id: request.id,
            product_id,
            barcode: request.barcode.clone(),
            barcode_type: request.barcode_type.clone(),
            created_at: request.created_at.clone(),
        }
    }
}
// Pasar de `( &ProductAddRequest, product_id )` a `ProductLotRequest`
impl From<(&ProductAddRequest, i64)> for ProductLotRequest {
    fn from((request, product_id): (&ProductAddRequest, i64)) -> Self {
        Self {
            id: request.id,
            product_id,
            lot_number: request.lot_number.clone(),
            qty_on_hand: request.qty_on_hand.clone(),
            expiry_date: request.expiry_date.clone(),
            purchase_id: request.purchase_id,
            created_at: request.created_at.clone(),
        }
    }
}
// Pasar de `( &ProductAddRequest, product_id )` a `ProductPriceRequest`
impl From<(&ProductAddRequest, i64)> for ProductPriceRequest {
    fn from((request, product_id): (&ProductAddRequest, i64)) -> Self {
        Self {
            id: request.id,
            product_id,
            price_type: request.price_type.clone(),
            price: request.price.clone(),
            created_at: request.created_at.clone(),
            starts_at: Some(request.created_at.clone()),
            ends_at: None,
        }
    }
}

//Generar Response para cada una de las entidades (Product, ProductBarcode, ProductLot, ProductPrice) a partir de `ProductAddRequest`
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProductAddResponse {
    pub product_id: i64,
    pub barcode_id: i64,
    pub lot_id: i64,
    pub price_id: i64,
}

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ProductAddResponseDetail {
    pub id: i64,
    pub sku: Option<String>,
    pub name: String,
    pub barcode: String,
    pub description: Option<String>,
    pub qty_on_hand: Decimal,
    pub price: Decimal,
    pub tax_profile_id: Option<i64>,
    pub purchase_price: Option<Decimal>,
    pub wholesale_price: Option<Decimal>,
    pub sale_price: Option<Decimal>,
    pub default_price: Option<Decimal>,
}

impl
    From<(
        ProductDetailResponse,
        ProductBarcodeDetailResponse,
        ProductLotDetailResponse,
        ProductPriceDetailResponse,
    )> for ProductAddResponseDetail
{
    fn from(
        (product_detail, barcode_detail, lot_detail, price_detail): (
            ProductDetailResponse,
            ProductBarcodeDetailResponse,
            ProductLotDetailResponse,
            ProductPriceDetailResponse,
        ),
    ) -> Self {
        Self {
            id: product_detail.id,
            sku: product_detail.sku,
            name: product_detail.name,
            barcode: barcode_detail.barcode,
            description: product_detail.description,
            qty_on_hand: lot_detail.qty_on_hand,
            price: price_detail.price,
            tax_profile_id: product_detail.tax_profile_id,
            purchase_price: product_detail.purchase_price,
            wholesale_price: product_detail.wholesale_price,
            sale_price: product_detail.sale_price,
            default_price: product_detail.default_price,
        }
    }
}
