pub mod api_module;
pub mod api_utils;
pub mod config;
pub mod controller;

use axum::serve;
use flexi_logger::{Duplicate, Logger};
use log::{error, info};
use std::net::SocketAddr;

use crate::{
    config::{
        config_database::config_db::{close_db_connection, get_db_context},
        config_jwt::validate_jwt::init_jwt_keys_if_needed,
    },
    controller::api_controller::get_config_router,
};
#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    use flexi_logger::FileSpec;
    let logger = Logger::try_with_str("info").unwrap_or_else(|e| {
        eprintln!("Logger configuration failed: {}", e);
        std::process::exit(1);
    });

    logger
        .log_to_file(
            FileSpec::default()
                .directory("/tmp/log/pharmacy_backend")
                .basename("app")
                .suffix("log"),
        )
        .duplicate_to_stdout(Duplicate::All)
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
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .unwrap_or_else(|_| {
            error!("Invalid PORT environment variable value");
            std::process::exit(1);
        });

    let ctx_bd = get_db_context().await;

    let addr: SocketAddr = SocketAddr::from(([0, 0, 0, 0], port));

    info!("Starting server on {}", addr);

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

    // Graceful shutdown: wait for SIGINT (Ctrl+C) or SIGTERM (Docker stop)
    let shutdown_signal = async {
        let ctrl_c = tokio::signal::ctrl_c();
        #[cfg(unix)]
        let terminate = async {
            tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
                .expect("failed to install SIGTERM handler")
                .recv()
                .await;
        };
        #[cfg(not(unix))]
        let terminate = std::future::pending::<()>();

        tokio::select! {
            _ = ctrl_c => { info!("Received SIGINT, shutting down..."); }
            _ = terminate => { info!("Received SIGTERM, shutting down..."); }
        }
    };

    if let Err(e) = serve(listener, app)
        .with_graceful_shutdown(shutdown_signal)
        .await
    {
        error!("Server error: {}", e);
        std::process::exit(1);
    }
    close_db_connection(ctx_bd.conn).await;
    info!("Server stopped gracefully");
}
