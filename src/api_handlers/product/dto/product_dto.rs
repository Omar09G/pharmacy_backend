use sea_orm::{FromQueryResult, prelude::*};
use serde::{Deserialize, Serialize};
use validator::Validate;
#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct ProductRequestDTO {
    #[validate(length(min = 1, message = "Product name cannot be empty"))]
    pub product_name: String,
    #[validate(range(min = 0, message = "Product catalog must be a non-negative integer"))]
    pub product_catalog: i32,
    #[validate(range(min = 0, message = "Product count must be a non-negative integer"))]
    pub product_count: i32,
    #[validate(length(min = 1, message = "Product code bar cannot be empty"))]
    pub product_code_bar: String,
    #[validate(range(min = 0.0, message = "Product price must be a non-negative number"))]
    pub product_price: f32,
    #[validate(length(
        max = 255,
        message = "Product description cannot exceed 255 characters"
    ))]
    pub product_desc: Option<String>,
    pub product_lote: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromQueryResult)]
#[serde(rename_all = "camelCase")]
pub struct TotalProducts {
    pub total: i64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductRequestPrice {
    pub product_id: i64,
    pub product_code_bar: String,
    pub product_price: f32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProductRequestCount {
    pub product_id: i64,
    pub product_code_bar: String,
    pub product_count: i32,
}

#[derive(Debug, Serialize, Deserialize, FromQueryResult)]
#[serde(rename_all = "camelCase")]
pub struct ProductResponse {
    pub product_id: i64,
    pub product_name: String,
    pub product_count: i32,
    pub product_code_bar: String,
    pub product_price: f32,
    pub product_lastmdate: Option<Date>,
}

impl ProductRequestDTO {
    pub fn new(
        product_name: String,
        product_catalog: i32,
        product_count: i32,
        product_code_bar: String,
        product_price: f32,
        product_desc: Option<String>,
        product_lote: Option<String>,
    ) -> Self {
        Self {
            product_name,
            product_catalog,
            product_count,
            product_code_bar,
            product_price,
            product_desc,
            product_lote,
        }
    }
}

impl ProductResponse {
    pub fn new(
        product_id: i64,
        product_name: String,
        product_count: i32,
        product_code_bar: String,
        product_price: f32,
        product_lastmdate: Option<Date>,
    ) -> Self {
        Self {
            product_id,
            product_name,
            product_count,
            product_code_bar,
            product_price,
            product_lastmdate,
        }
    }
}

impl From<schemas::product::ActiveModel> for ProductRequestDTO {
    fn from(product: schemas::product::ActiveModel) -> Self {
        ProductRequestDTO {
            product_name: product.product_name.unwrap(),
            product_catalog: product.product_catalog.unwrap(),
            product_count: product.product_count.unwrap(),
            product_code_bar: product.product_code_bar.unwrap(),
            product_price: product.product_price.unwrap(),
            product_desc: product.product_desc.unwrap(),
            product_lote: product.product_lote.unwrap(),
        }
    }
}

impl From<schemas::product::Model> for ProductRequestDTO {
    fn from(product: schemas::product::Model) -> Self {
        ProductRequestDTO {
            product_name: product.product_name,
            product_catalog: product.product_catalog,
            product_count: product.product_count,
            product_code_bar: product.product_code_bar,
            product_price: product.product_price,
            product_desc: product.product_desc,
            product_lote: product.product_lote,
        }
    }
}

impl From<schemas::product::ActiveModel> for ProductResponse {
    fn from(product: schemas::product::ActiveModel) -> Self {
        ProductResponse {
            product_id: product.product_id.unwrap(),
            product_name: product.product_name.unwrap(),
            product_count: product.product_count.unwrap(),
            product_code_bar: product.product_code_bar.unwrap(),
            product_price: product.product_price.unwrap(),
            product_lastmdate: product.product_lastmdate.unwrap(),
        }
    }
}

impl From<schemas::product::Model> for ProductResponse {
    fn from(product: schemas::product::Model) -> Self {
        ProductResponse {
            product_id: product.product_id,
            product_name: product.product_name,
            product_count: product.product_count,
            product_code_bar: product.product_code_bar,
            product_price: product.product_price,
            product_lastmdate: product.product_lastmdate,
        }
    }
}
