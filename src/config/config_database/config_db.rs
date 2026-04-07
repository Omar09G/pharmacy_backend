use std::time::Duration;

use log::info;
use sea_orm::DatabaseConnection;

use crate::config::config_database::config_db_context::AppContext;

pub fn get_database_url() -> String {
    std::env::var("DATABASE_URL").expect("DATABASE_URL environment variable not set")
}

pub async fn configure_database() -> Result<DatabaseConnection, String> {
    let db_url = get_database_url();

    let mut option_conection = sea_orm::ConnectOptions::new(db_url.clone());
    option_conection
        .max_connections(10)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(false)
        .sqlx_logging_level(log::LevelFilter::Off)
        .set_schema_search_path("pharmacy") // set default Postgres schema
        .min_connections(1);

    let db = sea_orm::Database::connect(option_conection)
        .await
        .map_err(|e| e.to_string())?;

    Ok(db)
}

pub async fn get_db_context() -> AppContext {
    let db = configure_database().await;
    match &db {
        Ok(_) => info!("Database connection established successfully"),
        Err(e) => info!("Failed to establish database connection: {}", e),
    }
    AppContext { conn: db.unwrap() }
}

pub async fn check_db_connection(db: &DatabaseConnection) -> bool {
    info!("Checking database connection...");
    db.ping().await.is_ok()
}

pub async fn close_db_connection(db: DatabaseConnection) -> bool {
    info!("Closing database connection...");
    db.close().await.is_ok()
}
