use sea_orm::DatabaseConnection;

use crate::config::config_database::config_db_context::AppContext;

pub fn get_database_url() -> String {
    std::env::var("DATABASE_URL").expect("DATABASE_URL environment variable not set")
}

pub async fn configure_database() -> Result<DatabaseConnection, String> {
    let db_url = get_database_url();
    let db = sea_orm::Database::connect(&db_url)
        .await
        .map_err(|e| e.to_string())?;
    Ok(db)
}

pub async fn get_db_context() -> AppContext {
    let db = configure_database().await;

    AppContext { conn: db.unwrap() }
}
