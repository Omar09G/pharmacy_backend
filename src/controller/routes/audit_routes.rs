use axum::routing::get;
use axum::Router;

use crate::api_module::audit_log::audit_log_service::audit_log_service::get_audit_logs;
use crate::config::config_database::config_db_context::AppContext;

macro_rules! route {
    ($p:literal) => {
        concat!("/v1/api", $p)
    };
}

const AUDIT_LOGS_LIST: &str = route!("/audit_log");

pub fn routes() -> Router<AppContext> {
    Router::new().route(AUDIT_LOGS_LIST, get(get_audit_logs))
}
