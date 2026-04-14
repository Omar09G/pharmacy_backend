use axum::routing::{delete, get, patch, post};
use axum::Router;

use crate::api_module::add_sale::add_sale_service::add_sale_service::{
    cancel_add_sale, create_add_sale, get_add_sale_by_id, get_add_sales_with_details,
};
use crate::api_module::discounts::discounts_service::discounts_service::{
    create_discount, delete_discount, get_discount_by_id, get_discounts, update_discount,
};
use crate::api_module::sale_items::sale_items_service::sale_items_service::{
    create_sale_item, delete_sale_item, get_sale_item_by_id, get_sale_items, update_sale_item,
};
use crate::api_module::sale_payments::sale_payments_service::sale_payments_service::{
    create_sale_payment, delete_sale_payment, get_sale_payment_by_id, get_sale_payments,
    update_sale_payment,
};
use crate::api_module::sales::sales_service::sales_service::{
    create_sale, delete_sale, get_sale_by_id, get_sales, update_sale,
};
use crate::config::config_database::config_db_context::AppContext;

macro_rules! route {
    ($p:literal) => {
        concat!("/v1/api", $p)
    };
}

const SALE: &str = route!("/sale");
const SALE_BY_ID: &str = route!("/sale/{:id}");
const SALES_LIST: &str = route!("/sale");
const SALE_DELETE: &str = route!("/sale/{:id}");
const SALE_UPDATE: &str = route!("/sale/{:id}");

const SALE_ITEM: &str = route!("/sale_item");
const SALE_ITEM_BY_ID: &str = route!("/sale_item/{:id}");
const SALE_ITEMS_LIST: &str = route!("/sale_item");
const SALE_ITEM_DELETE: &str = route!("/sale_item/{:id}");
const SALE_ITEM_UPDATE: &str = route!("/sale_item/{:id}");

const SALE_PAYMENT: &str = route!("/sale_payment");
const SALE_PAYMENT_BY_ID: &str = route!("/sale_payment/{:id}");
const SALE_PAYMENTS_LIST: &str = route!("/sale_payment");
const SALE_PAYMENT_DELETE: &str = route!("/sale_payment/{:id}");
const SALE_PAYMENT_UPDATE: &str = route!("/sale_payment/{:id}");

const DISCOUNT: &str = route!("/discount");
const DISCOUNT_BY_ID: &str = route!("/discount/{:id}");
const DISCOUNTS_LIST: &str = route!("/discount");
const DISCOUNT_DELETE: &str = route!("/discount/{:id}");
const DISCOUNT_UPDATE: &str = route!("/discount/{:id}");

const ADD_SALE: &str = route!("/add_sale");
const ADD_SALE_BY_ID: &str = route!("/add_sale/{:id}");

pub fn routes() -> Router<AppContext> {
    Router::new()
        // Sale routes
        .route(SALE, post(create_sale))
        .route(SALE_BY_ID, get(get_sale_by_id))
        .route(SALES_LIST, get(get_sales))
        .route(SALE_DELETE, delete(delete_sale))
        .route(SALE_UPDATE, patch(update_sale))
        // Sale Item routes
        .route(SALE_ITEM, post(create_sale_item))
        .route(SALE_ITEM_BY_ID, get(get_sale_item_by_id))
        .route(SALE_ITEMS_LIST, get(get_sale_items))
        .route(SALE_ITEM_DELETE, delete(delete_sale_item))
        .route(SALE_ITEM_UPDATE, patch(update_sale_item))
        // Sale Payment routes
        .route(SALE_PAYMENT, post(create_sale_payment))
        .route(SALE_PAYMENT_BY_ID, get(get_sale_payment_by_id))
        .route(SALE_PAYMENTS_LIST, get(get_sale_payments))
        .route(SALE_PAYMENT_DELETE, delete(delete_sale_payment))
        .route(SALE_PAYMENT_UPDATE, patch(update_sale_payment))
        // Discount routes
        .route(DISCOUNT, post(create_discount))
        .route(DISCOUNT_BY_ID, get(get_discount_by_id))
        .route(DISCOUNTS_LIST, get(get_discounts))
        .route(DISCOUNT_DELETE, delete(delete_discount))
        .route(DISCOUNT_UPDATE, patch(update_discount))
        // Add Sale (composite) routes
        .route(ADD_SALE, post(create_add_sale))
        .route(ADD_SALE_BY_ID, get(get_add_sale_by_id))
        .route(ADD_SALE, get(get_add_sales_with_details))
        .route(ADD_SALE_BY_ID, patch(cancel_add_sale))
}
