use axum::extract::DefaultBodyLimit;
use axum::{Router, middleware::from_fn};
use log::info;
use tower_http::timeout::TimeoutLayer;

use crate::config::config_database::config_db_context::AppContext;
use crate::config::config_middleware::auth_jwt::auth_middleware;
use crate::config::config_middleware::content_type::content_type_middleware;
use crate::config::config_middleware::cors::cors_middleware;
use crate::config::config_middleware::idempotency::idempotency_middleware;
use crate::config::config_middleware::rate_limit::rate_limit_middleware;
use crate::config::config_middleware::security_headers::security_headers_middleware;

use super::routes;

pub fn get_config_router(app_ctx: &AppContext) -> Result<Router, String> {
    info!("Configuring API routes...");
    let router = Router::new()
        .merge(routes::auth_routes::routes())
        .merge(routes::user_routes::routes())
        .merge(routes::rbac_routes::routes())
        .merge(routes::product_routes::routes())
        .merge(routes::inventory_routes::routes())
        .merge(routes::sales_routes::routes())
        .merge(routes::purchase_routes::routes())
        .merge(routes::finance_routes::routes())
        .merge(routes::catalog_routes::routes())
        .merge(routes::audit_routes::routes())
        .with_state(app_ctx.clone())
        // Layer order (innermost first): idempotency -> auth -> content_type -> rate_limit -> cors -> security_headers
        .layer(from_fn(idempotency_middleware))
        .layer(from_fn(auth_middleware))
        .layer(from_fn(content_type_middleware))
        .layer(from_fn(rate_limit_middleware))
        .layer(from_fn(cors_middleware))
        .layer(from_fn(security_headers_middleware))
        .layer(DefaultBodyLimit::max(2 * 1024 * 1024)) // 2 MB max request body
        .layer(TimeoutLayer::with_status_code(
            axum::http::StatusCode::REQUEST_TIMEOUT,
            std::time::Duration::from_secs(30),
        )); // 30s request timeout

    Ok(router)
}
