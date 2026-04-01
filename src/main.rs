pub mod api_module;
pub mod api_utils;
pub mod config;
pub mod controller;

use axum::serve;
use flexi_logger::{Duplicate, Logger};
use log::{error, info};
use std::net::SocketAddr;

use crate::{
    config::config_database::config_db::get_db_context,
    controller::api_controller::get_config_router,
};
#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    use flexi_logger::FileSpec;
    Logger::try_with_str("info")
        .unwrap()
        .log_to_file(
            FileSpec::default()
                .directory("/tmp/log/pharmacy_backend")
                .basename("app")
                .suffix("log"),
        )
        .duplicate_to_stdout(Duplicate::All)
        .start()
        .unwrap();

    info!("Starting Pharmacy Backend API...");

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

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap_or_else(|e| {
            panic!("Failed to bind to address {}: {}", addr, e);
        });

    let app = get_config_router(&ctx_bd);

    serve(listener, app.unwrap()).await.unwrap_or_else(|e| {
        panic!("Failed to serve on address {}", e);
    });
    info!("Server stopped");
}
