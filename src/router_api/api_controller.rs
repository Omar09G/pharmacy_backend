use axum::routing::put;
use axum::{Router, middleware::from_fn, routing::get};
use log::info;

use crate::api_handlers::login::login_handler::get_login;
use crate::api_handlers::user::user_handler::{create_user_handler, get_user_handler};
use crate::config::config_database::config_db_context::AppContext;
use crate::config::config_middleware::auth_jwt::auth_middleware;

pub fn get_config_router(app_ctx: &AppContext) -> Result<Router, String> {
    info!("Configuring API routes...");
    let router = Router::new()
        .route("/api/user/{user_id}", get(get_user_handler))
        .route("/api/user", put(create_user_handler))
        .route("/api/login", put(get_login))
        .with_state(app_ctx.clone())
        .layer(from_fn(auth_middleware));

    Ok(router)
}
