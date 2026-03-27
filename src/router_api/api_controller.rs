use axum::routing::{delete, patch, post, put};
use axum::{Router, middleware::from_fn, routing::get};
use log::info;

use crate::api_handlers::client::client_handler::{
    create_client_handler, delete_client_handler, get_all_clients_handler,
    get_client_by_id_handler, update_client_handler,
};
use crate::api_handlers::sales::sales_handler::{
    cancel_sale_handler, create_sale_handler, delete_sale_handler, get_sales_by_date_handler,
    get_sales_by_date_ini_fin_handler, get_sales_by_id_handler, get_sales_by_username_handler,
    get_sales_detail_by_date_ini_fin_handler, get_sales_detail_by_id_handler,
    get_sum_sales_by_date_handler, get_sum_sales_by_date_ini_fin_handler,
    get_sum_sales_by_date_ini_fin_username_handler, get_sum_sales_by_username_handler,
};
use crate::config::config_middleware::cors::cors_middleware;

use crate::api_handlers::login::login_handler::{get_login, get_profile};
use crate::api_handlers::product::service::product_service::{
    create_new_product, delete_product, get_all_product, get_product_by_cod_bar, get_product_by_id,
    get_product_by_name_details, update_product,
};
use crate::api_handlers::report::report_handler::get_report_list_user_active;
use crate::api_handlers::user::user_handler::{
    create_user_handler, get_all_users_handler, get_user_handler, update_pass_user_handler,
};
use crate::config::config_database::config_db_context::AppContext;
use crate::config::config_middleware::auth_jwt::auth_middleware;

// API route constants
// Base API prefix and helper macro to compose routes at compile time.

macro_rules! route {
    ($p:literal) => {
        concat!("/v1/api", $p)
    };
}

const USER_ID: &str = route!("/user/{user_id}");
const USER: &str = route!("/user");
const USER_PASS: &str = route!("/user/pass/{user_id}");
const LOGIN: &str = route!("/login");
const PROFILE: &str = route!("/auth/profile");
const REPORT_ACTIVE: &str = route!("/report/user/active/{tipo_user}");
const PRODUCT_ID: &str = route!("/product/{product_id}");
const PRODUCT: &str = route!("/product");
const PRODUCT_CODE: &str = route!("/product/code");
const PRODUCT_DETAILS: &str = route!("/product/details");
const SALE: &str = route!("/sale");
const SALE_ID: &str = route!("/sale/{sale_id}");
const SALE_CANCEL: &str = route!("/sale/cancel/{sale_id}");
const SALE_DATE: &str = route!("/sale/date");
const SALE_USER: &str = route!("/sale/user/");
// search/paginated sales between dates
const SALE_SEARCH: &str = route!("/sale/search");
// detailed parent-only list between dates (no child rows)
const SALE_DETAILS_RANGE: &str = route!("/sale/details");
// sale child details by parent sale id
const SALE_DETAIL_BY_ID: &str = route!("/sale/detail/{sale_id}");
const SALE_SUM_DATE: &str = route!("/sale/sum/{date}");
const SALE_SUM_RANGE: &str = route!("/sale/sum"); // expects query params date range
const SALE_SUM_USER: &str = route!("/sale/sum/user/{username}");
const SALE_SUM_USER_RANGE: &str = route!("/sale/sum/user"); // expects query params and optional username

const CLIENT: &str = route!("/client");
const CLIENT_ID: &str = route!("/client/{client_id}");

pub fn get_config_router(app_ctx: &AppContext) -> Result<Router, String> {
    info!("Configuring API routes...");
    let router = Router::new()
        .route(USER_ID, get(get_user_handler))
        .route(USER, get(get_all_users_handler))
        .route(USER, put(create_user_handler))
        .route(USER_PASS, patch(update_pass_user_handler))
        .route(LOGIN, post(get_login))
        .route(PROFILE, get(get_profile))
        .route(REPORT_ACTIVE, get(get_report_list_user_active))
        .route(PRODUCT_ID, get(get_product_by_id))
        .route(PRODUCT, get(get_all_product))
        .route(PRODUCT, put(create_new_product))
        .route(PRODUCT_ID, delete(delete_product))
        .route(PRODUCT_ID, patch(update_product))
        .route(PRODUCT_CODE, get(get_product_by_cod_bar))
        .route(PRODUCT_DETAILS, get(get_product_by_name_details))
        // Sales endpoints
        .route(SALE, put(create_sale_handler))
        // single sale operations: get and delete
        .route(
            SALE_ID,
            get(get_sales_by_id_handler).delete(delete_sale_handler),
        )
        // cancel sale (status update)
        .route(SALE_CANCEL, patch(cancel_sale_handler))
        // fetch sales by exact date or by username
        .route(SALE_DATE, get(get_sales_by_date_handler))
        .route(SALE_USER, get(get_sales_by_username_handler))
        // sale detail / search endpoints
        .route(SALE_SEARCH, get(get_sales_by_date_ini_fin_handler))
        .route(SALE_DETAIL_BY_ID, get(get_sales_detail_by_id_handler))
        .route(
            SALE_DETAILS_RANGE,
            get(get_sales_detail_by_date_ini_fin_handler),
        )
        // sum/aggregation endpoints
        .route(SALE_SUM_DATE, get(get_sum_sales_by_date_handler))
        .route(SALE_SUM_RANGE, get(get_sum_sales_by_date_ini_fin_handler))
        .route(SALE_SUM_USER, get(get_sum_sales_by_username_handler))
        .route(
            SALE_SUM_USER_RANGE,
            get(get_sum_sales_by_date_ini_fin_username_handler),
        )
        .route(CLIENT, get(get_all_clients_handler))
        .route(CLIENT, put(create_client_handler))
        .route(CLIENT_ID, get(get_client_by_id_handler))
        .route(CLIENT_ID, delete(delete_client_handler))
        .route(CLIENT_ID, patch(update_client_handler))
        .with_state(app_ctx.clone())
        // CORS middleware must be the outermost layer so it runs before auth
        .layer(from_fn(auth_middleware))
        .layer(from_fn(cors_middleware));

    Ok(router)
}
