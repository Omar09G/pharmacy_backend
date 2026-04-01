use axum::routing::{delete, get, patch, post, put};
use axum::{Router, middleware::from_fn};
use log::info;

use crate::api_module::login::service::login_service::{get_login, get_profile};
use crate::api_module::user::service::user_service::{
    change_user_password, change_user_status, create_user, delete_user, get_all_users,
    get_user_by_id, update_user,
};
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

/*Metodos USER  */
const USER: &str = route!("/user");
const USER_BY_ID: &str = route!("/user/{:id}");
const USER_CHANGE_PASSWORD: &str = route!("/user/password");
const USER_CHANGE_STATUS: &str = route!("/user/status");
const USER_LIST: &str = route!("/users");
const USER_DELETE: &str = route!("/user");
const USER_UPDATE: &str = route!("/user");

pub fn get_config_router(app_ctx: &AppContext) -> Result<Router, String> {
    info!("Configuring API routes...");
    let router = Router::new()
        .route(LOGIN, post(get_login))
        .route(PROFILE, get(get_profile))
        // User routes
        .route(USER, put(create_user))
        .route(USER_BY_ID, get(get_user_by_id))
        .route(USER_CHANGE_PASSWORD, put(change_user_password))
        .route(USER_CHANGE_STATUS, put(change_user_status))
        .route(USER_LIST, get(get_all_users))
        .route(USER_DELETE, delete(delete_user))
        .route(USER_UPDATE, patch(update_user))
        .with_state(app_ctx.clone())
        // CORS middleware must be the outermost layer so it runs before auth
        .layer(from_fn(auth_middleware))
        .layer(from_fn(cors_middleware));

    Ok(router)
}
