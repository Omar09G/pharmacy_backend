use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ParamsInventoryProduct {
    pub product_id: Option<i64>,
    pub product_count: Option<i32>,
    pub product_price: Option<f32>,
    pub username: Option<String>,
}

#[derive(Serialize, Deserialize, Validate, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResponseInventoryProduct {
    pub product_id: i64,
}

impl ResponseInventoryProduct {
    pub fn new(product_id: i64) -> Self {
        Self { product_id }
    }
}
