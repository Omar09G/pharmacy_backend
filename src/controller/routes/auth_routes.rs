use axum::Router;
use axum::routing::{get, post};

use crate::api_module::login::service::login_service::{
    get_login, get_profile, logout, refresh_token,
};
use crate::config::config_database::config_db_context::AppContext;

macro_rules! route {
    ($p:literal) => {
        concat!("/v1/api", $p)
    };
}

const LOGIN: &str = route!("/auth/login");
const PROFILE: &str = route!("/auth/profile");
const REFRESH: &str = route!("/auth/refresh");
const LOGOUT: &str = route!("/auth/logout");

pub fn routes() -> Router<AppContext> {
    Router::new()
        .route(LOGIN, post(get_login))
        .route(PROFILE, get(get_profile))
        .route(REFRESH, post(refresh_token))
        .route(LOGOUT, post(logout))
}
