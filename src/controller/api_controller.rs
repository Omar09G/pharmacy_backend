use axum::error_handling::HandleErrorLayer;
use axum::extract::DefaultBodyLimit;
use axum::http::StatusCode;
use axum::{Router, middleware::from_fn};
use log::info;
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;
use tower_http::timeout::TimeoutLayer;

use crate::config::config_database::config_db_context::AppContext;
use crate::config::config_middleware::auth_jwt::auth_middleware;
use crate::config::config_middleware::cache::cache_middleware;
use crate::config::config_middleware::content_type::content_type_middleware;
use crate::config::config_middleware::cors::cors_middleware;
use crate::config::config_middleware::idempotency::idempotency_middleware;
use crate::config::config_middleware::rate_limit::rate_limit_middleware;
use crate::config::config_middleware::request_id::request_id_middleware;
use crate::config::config_middleware::security_headers::security_headers_middleware;

use super::routes;

/// Upper bound of in-flight requests per replica. Requests beyond this are
/// shed with 503 instead of piling up on the DB pool (env-configurable).
fn max_concurrent_requests() -> usize {
    std::env::var("API_MAX_CONCURRENCY")
        .ok()
        .and_then(|v| v.parse().ok())
        .filter(|&v| v > 0 && v <= 10_000)
        .unwrap_or(256)
}

//Obtiene las rutas de configuracion para la API
pub fn get_config_router(app_ctx: &AppContext) -> Result<Router, String> {
    info!("Configuring API routes...");
    let router = Router::new()
        .merge(routes::auth_routes::routes())
        .merge(routes::health_routes::routes())
        .merge(routes::user_routes::routes())
        .merge(routes::rbac_routes::routes())
        .merge(routes::product_routes::routes())
        .merge(routes::inventory_routes::routes())
        .merge(routes::sales_routes::routes())
        .merge(routes::purchase_routes::routes())
        .merge(routes::finance_routes::routes())
        .merge(routes::catalog_routes::routes())
        .merge(routes::audit_routes::routes())
        .merge(routes::dashboard_routes::routes())
        .with_state(app_ctx.clone())
        .layer(from_fn(cache_middleware))
        .layer(from_fn(idempotency_middleware))
        .layer(from_fn(auth_middleware))
        .layer(from_fn(content_type_middleware))
        .layer(from_fn(rate_limit_middleware))
        .layer(from_fn(cors_middleware))
        .layer(from_fn(security_headers_middleware))
        .layer(CompressionLayer::new().gzip(true))
        .layer(DefaultBodyLimit::max(2 * 1024 * 1024))
        .layer(TimeoutLayer::with_status_code(
            axum::http::StatusCode::REQUEST_TIMEOUT,
            std::time::Duration::from_secs(30),
        )) // 30s request timeout
        // Saturation guard: shed overload with 503 before it queues into the
        // DB pool / Redis. Inner to request_id so rejections stay correlated.
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|_: axum::BoxError| async {
                    StatusCode::SERVICE_UNAVAILABLE
                }))
                .load_shed()
                .concurrency_limit(max_concurrent_requests()),
        )
        // Outermost: every call gets a correlation id even on early failures.
        .layer(from_fn(request_id_middleware));

    Ok(router)
}
