use std::time::Duration;

use sea_orm::DatabaseConnection;

use crate::config::config_database::config_db_context::AppContext;

pub fn get_database_url() -> String {
    std::env::var("DATABASE_URL").expect("DATABASE_URL environment variable not set")
}

pub async fn configure_database() -> Result<DatabaseConnection, String> {
    let db_url = get_database_url();

    let mut option_conection = sea_orm::ConnectOptions::new(db_url.clone());
    option_conection
        .max_connections(100)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(false)
        .sqlx_logging_level(log::LevelFilter::Off)
        .set_schema_search_path("pharmacy"); // set default Postgres schema

    let db = sea_orm::Database::connect(&db_url)
        .await
        .map_err(|e| e.to_string())?;
    Ok(db)
}

pub async fn get_db_context() -> AppContext {
    let db = configure_database().await;

    AppContext { conn: db.unwrap() }
}

pub async fn check_db_connection(db: &DatabaseConnection) -> bool {
    db.ping().await.is_ok()
}

pub async fn close_db_connection(db: DatabaseConnection) -> bool {
    db.close().await.is_ok()
}
