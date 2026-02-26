use axum::routing::put;
use axum::{Router, routing::get};
use log::info;

use crate::api_handlers::user::user_handler::{create_user_handler, get_user_handler};
use crate::config::config_database::config_db_context::AppContext;

pub fn get_config_router(app_ctx: &AppContext) -> Result<Router, String> {
    info!("Configuring API routes...");
    let router = Router::new()
        .route("/api/user/{user_id}", get(get_user_handler))
        .route("/api/user/", put(create_user_handler))
        .with_state(app_ctx.clone());

    Ok(router)
}
