use std::time::Duration;

use log::{error, info, warn};
use sea_orm::DatabaseConnection;

use crate::config::config_database::config_db_context::AppContext;

/// Default number of connection attempts before giving up.
const DEFAULT_CONNECT_RETRIES: u32 = 5;
/// Base delay for the exponential backoff (milliseconds).
const BACKOFF_BASE_MS: u64 = 500;
/// Maximum delay cap so retries stay bounded (milliseconds).
const BACKOFF_MAX_MS: u64 = 8_000;

pub fn get_database_url() -> Result<String, String> {
    std::env::var("DATABASE_URL")
        .map_err(|_| "DATABASE_URL environment variable not set".to_string())
}

pub async fn configure_database() -> Result<DatabaseConnection, String> {
    let db_url = get_database_url()?;

    // Configurable pool sizes via environment variables (with sensible defaults)
    let max_connections: u32 = std::env::var("DATABASE_MAX_CONNECTIONS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(20);
    let min_connections: u32 = std::env::var("DATABASE_MIN_CONNECTIONS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(5);
    let connect_timeout_secs: u64 = std::env::var("DATABASE_CONNECT_TIMEOUT_SECS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(10);
    let acquire_timeout_secs: u64 = std::env::var("DATABASE_ACQUIRE_TIMEOUT_SECS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(10);

    // Enable SQLx logging only when LOG_LEVEL is debug or trace
    let log_level = std::env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string());
    let sqlx_log_level = if log_level == "debug" || log_level == "trace" {
        log::LevelFilter::Debug
    } else {
        log::LevelFilter::Off
    };

    let mut connect_options = sea_orm::ConnectOptions::new(db_url);
    connect_options
        .max_connections(max_connections)
        .min_connections(min_connections)
        .connect_timeout(Duration::from_secs(connect_timeout_secs))
        .acquire_timeout(Duration::from_secs(acquire_timeout_secs))
        .idle_timeout(Duration::from_secs(300))
        .max_lifetime(Duration::from_secs(1800))
        .sqlx_logging(sqlx_log_level != log::LevelFilter::Off)
        .sqlx_logging_level(sqlx_log_level)
        .set_schema_search_path("pharmacy");

    info!(
        "Connecting to database (max={}, min={}, connect_timeout={}s, sqlx_logging={})",
        max_connections, min_connections, connect_timeout_secs, sqlx_log_level
    );

    sea_orm::Database::connect(connect_options)
        .await
        .map_err(|e| e.to_string())
}

/// Attempts to establish the DB connection with exponential backoff.
///
/// Retries up to `DATABASE_CONNECT_RETRIES` times (default: 5) waiting
/// `BACKOFF_BASE_MS * 2^n` ms between attempts (capped at `BACKOFF_MAX_MS`),
/// so a slow-starting database container does not crash the backend.
fn backoff_delay(attempt: u32) -> Duration {
    let shift = attempt.min(20); // avoid overflow on huge attempts
    let delay_ms = BACKOFF_BASE_MS
        .saturating_mul(1u64 << shift)
        .min(BACKOFF_MAX_MS);
    Duration::from_millis(delay_ms)
}

pub async fn get_db_context() -> AppContext {
    let max_retries: u32 = std::env::var("DATABASE_CONNECT_RETRIES")
        .ok()
        .and_then(|v| v.parse().ok())
        .filter(|&r: &u32| r >= 1)
        .unwrap_or(DEFAULT_CONNECT_RETRIES);

    let mut last_err = String::new();
    for attempt in 0..max_retries {
        match configure_database().await {
            Ok(conn) => {
                if attempt > 0 {
                    info!(
                        "Database connection established after {} attempt(s)",
                        attempt + 1
                    );
                } else {
                    info!("Database connection established successfully");
                }
                return AppContext { conn };
            }
            Err(e) => {
                last_err = e;
                if attempt + 1 < max_retries {
                    let delay = backoff_delay(attempt);
                    warn!(
                        "DB connection attempt {}/{} failed: {}. Retrying in {:?}...",
                        attempt + 1,
                        max_retries,
                        last_err,
                        delay
                    );
                    tokio::time::sleep(delay).await;
                }
            }
        }
    }

    error!(
        "Failed to establish database connection after {} attempts: {}",
        max_retries, last_err
    );
    error!("Exiting application due to database connection failure");
    std::process::exit(1);
}

pub async fn check_db_connection(db: &DatabaseConnection) -> bool {
    db.ping().await.is_ok()
}

pub async fn close_db_connection(db: DatabaseConnection) -> bool {
    info!("Closing database connection...");
    db.close().await.is_ok()
}
