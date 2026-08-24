// See lib.rs for the rationale of these crate-level allowances.
#![allow(clippy::module_inception)]
#![allow(clippy::let_underscore_future)]

pub mod api_module;
pub mod api_utils;
pub mod config;
pub mod controller;

use axum::serve;
use flexi_logger::{Duplicate, Logger};
use log::{error, info};
use std::net::SocketAddr;

use crate::{
    api_utils::api_utils_fun::{custom_format, custom_format_colored},
    config::{
        config_database::config_db::{close_db_connection, get_db_context},
        config_jwt::validate_jwt::init_jwt_keys_if_needed,
    },
    controller::api_controller::get_config_router,
};
use migration::MigratorTrait;
#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    use flexi_logger::FileSpec;

    // Read log level from environment (default: info)
    let log_level = std::env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string());
    let log_dir = std::env::var("LOG_DIR").unwrap_or_else(|_| "/app/logs".to_string());
    // Rotation keeps disk usage bounded under sustained traffic.
    let rotate_mb: u64 = std::env::var("LOG_ROTATE_MB")
        .ok()
        .and_then(|v| v.parse().ok())
        .filter(|&v| v > 0 && v <= 1024)
        .unwrap_or(50);
    let keep_files: usize = std::env::var("LOG_KEEP_FILES")
        .ok()
        .and_then(|v| v.parse().ok())
        .filter(|&v| v > 0 && v <= 365)
        .unwrap_or(7);
    // Duplicating every line to stdout doubles I/O; disable in production
    // when the platform already collects container stdout.
    let duplicate_stdout = std::env::var("LOG_STDOUT")
        .map(|v| v.to_lowercase() != "false")
        .unwrap_or(true);
    let logger = Logger::try_with_str(&log_level)
        .unwrap_or_else(|e| {
            eprintln!("Logger configuration failed: {}", e);
            std::process::exit(1);
        })
        .format_for_files(custom_format)
        .format_for_stdout(custom_format_colored)
        .log_to_file(
            FileSpec::default()
                .directory(&log_dir)
                .basename("app")
                .suffix("log"),
        )
        .rotate(
            flexi_logger::Criterion::Size(rotate_mb * 1024 * 1024),
            flexi_logger::Naming::Timestamps,
            flexi_logger::Cleanup::KeepLogFiles(keep_files),
        );
    let logger = if duplicate_stdout {
        logger.duplicate_to_stdout(Duplicate::All)
    } else {
        logger
    };
    logger
        .start()
        .unwrap_or_else(|e| {
            eprintln!("Logger start failed: {}", e);
            std::process::exit(1);
        });

    info!("Starting Pharmacy Backend API...");

    if let Err(e) = init_jwt_keys_if_needed() {
        error!("Failed to initialize JWT keys: {}", e);
    }

    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "8081".to_string())
        .parse::<u16>()
        .unwrap_or_else(|_| {
            error!("Invalid PORT environment variable value");
            std::process::exit(1);
        });

    let server_addr = std::env::var("SERVER_ADDR").unwrap_or_else(|_| "0.0.0.0".to_string());
    let ctx_bd = get_db_context().await;

    // Optional migrations at startup (ARCH-4). Disabled by default so the
    // binary never mutates an existing schema implicitly; enable with
    // RUN_MIGRATIONS=true in environments where that is desired.
    if std::env::var("RUN_MIGRATIONS")
        .map(|v| v.to_lowercase() == "true")
        .unwrap_or(false)
    {
        match migration::Migrator::up(&ctx_bd.conn, None).await {
            Ok(_) => info!("Database migrations applied successfully"),
            Err(e) => error!("Database migration failed (continuing startup): {}", e),
        }
    }

    // Initialize Redis (optional). Use REDIS_URL env or default to local redis.
    let redis_url =
        std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379/".to_string());
    match crate::config::config_redis::init_redis(&redis_url).await {
        Ok(_) => info!("Redis initialized: {}", redis_url),
        Err(e) => error!(
            "Failed to initialize Redis (continuing without Redis): {}",
            e
        ),
    }
    let addr: SocketAddr = format!("{}:{}", server_addr, port)
        .parse()
        .unwrap_or_else(|e| {
            error!("Invalid server address '{}:{}': {}", server_addr, port, e);
            std::process::exit(1);
        });

    info!("Starting server Pharmacy Backend API on {}", addr);

    let listener = match tokio::net::TcpListener::bind(addr).await {
        Ok(l) => l,
        Err(e) => {
            error!("Failed to bind to address {}: {}", addr, e);
            std::process::exit(1);
        }
    };

    let app = match get_config_router(&ctx_bd) {
        Ok(r) => r,
        Err(e) => {
            error!("Failed to configure router: {}", e);
            std::process::exit(1);
        }
    };

    let shutdown_signal = async {
        let ctrl_c = tokio::signal::ctrl_c();
        #[cfg(unix)]
        let terminate = async {
            match tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate()) {
                Ok(mut sig) => {
                    sig.recv().await;
                }
                Err(e) => {
                    // A failed handler must not panic the process; ctrl_c still works.
                    error!("failed to install SIGTERM handler: {}", e);
                    std::future::pending::<()>().await;
                }
            }
        };
        #[cfg(not(unix))]
        let terminate = std::future::pending::<()>();

        tokio::select! {
            _ = ctrl_c => { info!("Received SIGINT, shutting down..."); }
            _ = terminate => { info!("Received SIGTERM, shutting down..."); }
        }
    };

    if let Err(e) = serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown_signal)
    .await
    {
        error!("Server error: {}", e);
        std::process::exit(1);
    }
    close_db_connection(ctx_bd.conn).await;
    crate::config::config_redis::close_redis().await;
    info!("Server stopped gracefully");
}
