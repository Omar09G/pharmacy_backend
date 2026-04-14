use axum::routing::{delete, get, patch, post};
use axum::Router;

use crate::api_module::purchase_items::purchase_items_service::purchase_items_service::{
    create_purchase_item, delete_purchase_item, get_purchase_item_by_id, get_purchase_items,
    update_purchase_item,
};
use crate::api_module::purchase_payments::purchase_payments_service::purchase_payments_service::{
    create_purchase_payment, delete_purchase_payment, get_purchase_payment_by_id,
    get_purchase_payments, update_purchase_payment,
};
use crate::api_module::purchases::purchases_service::purchases_service::{
    create_purchase, delete_purchase, get_purchase_by_id, get_purchases, update_purchase,
};
use crate::config::config_database::config_db_context::AppContext;

macro_rules! route {
    ($p:literal) => {
        concat!("/v1/api", $p)
    };
}

const PURCHASE: &str = route!("/purchase");
const PURCHASE_BY_ID: &str = route!("/purchase/{:id}");
const PURCHASES_LIST: &str = route!("/purchase");
const PURCHASE_DELETE: &str = route!("/purchase/{:id}");
const PURCHASE_UPDATE: &str = route!("/purchase/{:id}");

const PURCHASE_ITEM: &str = route!("/purchase_item");
const PURCHASE_ITEM_BY_ID: &str = route!("/purchase_item/{:id}");
const PURCHASE_ITEM_DELETE: &str = route!("/purchase_item/{:id}");
const PURCHASE_ITEM_UPDATE: &str = route!("/purchase_item/{:id}");

const PURCHASE_PAYMENT: &str = route!("/purchase_payment");
const PURCHASE_PAYMENT_BY_ID: &str = route!("/purchase_payment/{:id}");
const PURCHASE_PAYMENTS_LIST: &str = route!("/purchase_payment");
const PURCHASE_PAYMENT_DELETE: &str = route!("/purchase_payment/{:id}");
const PURCHASE_PAYMENT_UPDATE: &str = route!("/purchase_payment/{:id}");

pub fn routes() -> Router<AppContext> {
    Router::new()
        // Purchase routes
        .route(PURCHASE, post(create_purchase))
        .route(PURCHASE_BY_ID, get(get_purchase_by_id))
        .route(PURCHASES_LIST, get(get_purchases))
        .route(PURCHASE_DELETE, delete(delete_purchase))
        .route(PURCHASE_UPDATE, patch(update_purchase))
        // Purchase Item routes
        .route(PURCHASE_ITEM, post(create_purchase_item))
        .route(PURCHASE_ITEM_BY_ID, get(get_purchase_item_by_id))
        .route(PURCHASE_ITEM_DELETE, delete(delete_purchase_item))
        .route(PURCHASE_ITEM, get(get_purchase_items))
        .route(PURCHASE_ITEM_UPDATE, patch(update_purchase_item))
        // Purchase Payment routes
        .route(PURCHASE_PAYMENT, post(create_purchase_payment))
        .route(PURCHASE_PAYMENT_BY_ID, get(get_purchase_payment_by_id))
        .route(PURCHASE_PAYMENTS_LIST, get(get_purchase_payments))
        .route(PURCHASE_PAYMENT_DELETE, delete(delete_purchase_payment))
        .route(PURCHASE_PAYMENT_UPDATE, patch(update_purchase_payment))
}
