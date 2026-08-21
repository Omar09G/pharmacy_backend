use axum::Router;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::get;

use crate::config::config_database::config_db_context::AppContext;

macro_rules! route {
    ($p:literal) => {
        concat!("/v1/api", $p)
    };
}

const HEALTH: &str = route!("/health");

/// Health-check endpoints (no auth required).
///
/// - `GET /v1/api/health`       → liveness probe (always 200 while serving)
/// - `GET /v1/api/health/ready` → readiness probe; returns
///   - `200` when the DB is reachable (Redis optional),
///   - `503` when the DB is unreachable so load balancers / orchestrators
///     can stop routing traffic to this instance.
pub fn routes() -> Router<AppContext> {
    Router::new()
        .route(HEALTH, get(liveness))
        .route(route!("/health/ready"), get(readiness))
}

/// Liveness: the process is alive and serving requests.
async fn liveness() -> axum::Json<serde_json::Value> {
    axum::Json(serde_json::json!({
        "status": "UP",
        "service": "pharmacy-backend",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

/// Readiness: checks that downstream dependencies (DB, Redis) are reachable.
async fn readiness(State(app_ctx): State<AppContext>) -> axum::response::Response {
    let db_ok = crate::config::config_database::config_db::check_db_connection(&app_ctx.conn).await;

    // Redis is optional — if not initialized, report as "skipped"
    let redis_status = match crate::config::config_redis::get_kv("health:probe").await {
        Ok(_) => "UP",
        Err(_) => "DEGRADED",
    };

    let overall = if db_ok { "UP" } else { "DOWN" };
    let status = if db_ok {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    };

    (
        status,
        axum::Json(serde_json::json!({
            "status": overall,
            "checks": {
                "database": if db_ok { "UP" } else { "DOWN" },
                "redis": redis_status,
                "service": "UP"
            },
            "timestamp": chrono::Utc::now().to_rfc3339()
        })),
    )
        .into_response()
}
