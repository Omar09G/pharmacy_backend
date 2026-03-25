use axum::Extension;
use axum::routing::{delete, patch, post, put};
use axum::{Router, middleware::from_fn, routing::get};
use log::info;

use crate::api_handlers::sales::sales_handler::{
    create_sale_handler, get_sales_by_date_ini_fin_handler, get_sales_by_id_handler,
    get_sum_sales_by_date_ini_fin_handler,
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

pub fn get_config_router(app_ctx: &AppContext) -> Result<Router, String> {
    info!("Configuring API routes...");
    let router = Router::new()
        .route("/v1/api/user/{user_id}", get(get_user_handler))
        .route("/v1/api/user", get(get_all_users_handler))
        .route("/v1/api/user", put(create_user_handler))
        .route(
            "/v1/api/user/pass/{user_id}",
            patch(update_pass_user_handler),
        )
        .route("/v1/api/login", post(get_login))
        .route("/v1/api/auth/profile", get(get_profile))
        .route(
            "/v1/api/report/user/active/{tipo_user}",
            get(get_report_list_user_active),
        )
        .route("/v1/api/product/{product_id}", get(get_product_by_id))
        .route("/v1/api/product", get(get_all_product))
        .route("/v1/api/product", put(create_new_product))
        .route("/v1/api/product/{product_id}", delete(delete_product))
        .route("/v1/api/product/{product_id}", patch(update_product))
        .route("/v1/api/product/code", get(get_product_by_cod_bar))
        .route("/v1/api/product/details", get(get_product_by_name_details))
        .route("/v1/api/sale", put(create_sale_handler))
        .route("/v1/api/sale/{sale_id}", get(get_sales_by_id_handler))
        .route(
            "/v1/api/sale/detail",
            get(get_sales_by_date_ini_fin_handler),
        )
        .route(
            "/v1/api/sale/detail/total",
            get(get_sum_sales_by_date_ini_fin_handler),
        )
        .with_state(app_ctx.clone())
        .layer(Extension(app_ctx.clone()))
        // CORS middleware must be the outermost layer so it runs before auth
        .layer(from_fn(auth_middleware))
        .layer(from_fn(cors_middleware));

    Ok(router)
}
