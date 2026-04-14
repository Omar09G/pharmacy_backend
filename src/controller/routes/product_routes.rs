use axum::routing::{delete, get, patch, post};
use axum::Router;

use crate::api_module::add_product::add_product_service::add_product_service::{
    add_product, get_product_by_bar_code, get_products_with_details,
};
use crate::api_module::categories::categories_service::categories_service::{
    create_category, delete_category, get_categories, get_category_by_id, update_category,
};
use crate::api_module::product_barcodes::product_barcodes_service::product_barcodes_service::{
    create_product_barcode, delete_product_barcode, get_product_barcode_by_id,
    get_product_barcodes, get_product_barcodes_by_barcode, update_product_barcode,
};
use crate::api_module::product_lots::product_lots_service::product_lots_service::{
    create_product_lot, delete_product_lot, get_product_lot_by_id, get_product_lots,
    update_product_lot,
};
use crate::api_module::product_lots::{adjust_product_lot, get_product_lot_by_barcode};
use crate::api_module::product_prices::product_prices_service::product_prices_service::{
    create_product_price, delete_product_price, get_product_price_by_id, get_product_prices,
    update_product_price,
};
use crate::api_module::products::products_service::products_service::{
    create_product, delete_product, get_product_by_id, get_products, get_products_by_name,
    update_product,
};
use crate::config::config_database::config_db_context::AppContext;

macro_rules! route {
    ($p:literal) => {
        concat!("/v1/api", $p)
    };
}

const PRODUCT: &str = route!("/product");
const PRODUCT_BY_ID: &str = route!("/product/{:id}");
const PRODUCTS_LIST: &str = route!("/product");
const PRODUCT_DELETE: &str = route!("/product/{:id}");
const PRODUCT_UPDATE: &str = route!("/product/{:id}");
const PRODUCT_BY_NAME: &str = route!("/product/name");

const CATEGORY: &str = route!("/category");
const CATEGORY_BY_ID: &str = route!("/category/{:id}");
const CATEGORY_LIST: &str = route!("/category");
const CATEGORY_DELETE: &str = route!("/category/{:id}");
const CATEGORY_UPDATE: &str = route!("/category/{:id}");

const PRODUCT_BARCODE: &str = route!("/product_barcode");
const PRODUCT_BARCODE_BY_ID: &str = route!("/product_barcode/{:id}");
const PRODUCT_BARCODE_DELETE: &str = route!("/product_barcode/{:id}");
const PRODUCT_BARCODE_BY_BARCODE: &str = route!("/product_barcode/barcode");
const PRODUCT_BARCODE_UPDATE: &str = route!("/product_barcode/{:id}");

const PRODUCT_PRICE: &str = route!("/product_price");
const PRODUCT_PRICE_BY_ID: &str = route!("/product_price/{:id}");
const PRODUCT_PRICE_DELETE: &str = route!("/product_price/{:id}");

const PRODUCT_LOT: &str = route!("/product_lot");
const PRODUCT_LOT_BY_ID: &str = route!("/product_lot/{:id}");
const PRODUCT_LOT_DELETE: &str = route!("/product_lot/{:id}");
const PRODUCT_LOT_UPDATE: &str = route!("/product_lot/{:id}");
const PRODUCT_LOT_ADJUST: &str = route!("/product_lot/adjust/{:id}");
const PRODUCT_LOT_BY_BARCODE: &str = route!("/product_lot/barcode/{:barcode}");

const ADD_PRODUCT: &str = route!("/add_product");
const ADD_PRODUCT_BY_DETAIL: &str = route!("/add_product");
const ADD_PRODUCT_BY_BARCODE: &str = route!("/add_product/{:barcode}");

pub fn routes() -> Router<AppContext> {
    Router::new()
        // Add Product (composite) routes
        .route(ADD_PRODUCT, post(add_product))
        .route(ADD_PRODUCT_BY_DETAIL, get(get_products_with_details))
        .route(ADD_PRODUCT_BY_BARCODE, get(get_product_by_bar_code))
        // Product routes
        .route(PRODUCT, post(create_product))
        .route(PRODUCT_BY_ID, get(get_product_by_id))
        .route(PRODUCTS_LIST, get(get_products))
        .route(PRODUCT_DELETE, delete(delete_product))
        .route(PRODUCT_UPDATE, patch(update_product))
        .route(PRODUCT_BY_NAME, get(get_products_by_name))
        // Category routes
        .route(CATEGORY, post(create_category))
        .route(CATEGORY_BY_ID, get(get_category_by_id))
        .route(CATEGORY_LIST, get(get_categories))
        .route(CATEGORY_DELETE, delete(delete_category))
        .route(CATEGORY_UPDATE, patch(update_category))
        // Product Barcode routes
        .route(PRODUCT_BARCODE, post(create_product_barcode))
        .route(PRODUCT_BARCODE_BY_ID, get(get_product_barcode_by_id))
        .route(PRODUCT_BARCODE_DELETE, delete(delete_product_barcode))
        .route(PRODUCT_BARCODE, get(get_product_barcodes))
        .route(PRODUCT_BARCODE_UPDATE, patch(update_product_barcode))
        .route(PRODUCT_BARCODE_BY_BARCODE, get(get_product_barcodes_by_barcode))
        // Product Price routes
        .route(PRODUCT_PRICE, post(create_product_price))
        .route(PRODUCT_PRICE_BY_ID, get(get_product_price_by_id))
        .route(PRODUCT_PRICE_DELETE, delete(delete_product_price))
        .route(PRODUCT_PRICE, get(get_product_prices))
        .route(PRODUCT_PRICE_DELETE, patch(update_product_price))
        // Product Lot routes
        .route(PRODUCT_LOT, post(create_product_lot))
        .route(PRODUCT_LOT_BY_ID, get(get_product_lot_by_id))
        .route(PRODUCT_LOT_DELETE, delete(delete_product_lot))
        .route(PRODUCT_LOT, get(get_product_lots))
        .route(PRODUCT_LOT_UPDATE, patch(update_product_lot))
        .route(PRODUCT_LOT_ADJUST, patch(adjust_product_lot))
        .route(PRODUCT_LOT_BY_BARCODE, get(get_product_lot_by_barcode))
}
