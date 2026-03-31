use axum::routing::{get, post};
use axum::{Router, middleware::from_fn};
use log::info;

use crate::api_handlers::login::service::login_service::{get_login, get_profile};
use crate::config::config_database::config_db_context::AppContext;
use crate::config::config_middleware::auth_jwt::auth_middleware;
use crate::config::config_middleware::cors::cors_middleware;

// API route constants
// Base API prefix and helper macro to compose routes at compile time.

macro_rules! route {
    ($p:literal) => {
        concat!("/v1/api", $p)
    };
}

const LOGIN: &str = route!("/auth/login");
const PROFILE: &str = route!("/auth/profile");

pub fn get_config_router(app_ctx: &AppContext) -> Result<Router, String> {
    info!("Configuring API routes...");
    let router = Router::new()
        .route(LOGIN, post(get_login))
        .route(PROFILE, get(get_profile))
        .with_state(app_ctx.clone())
        // CORS middleware must be the outermost layer so it runs before auth
        .layer(from_fn(auth_middleware))
        .layer(from_fn(cors_middleware));

    Ok(router)
}
