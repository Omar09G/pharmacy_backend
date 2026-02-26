use sea_orm::DatabaseConnection;

#[derive(Clone)]
pub struct AppContext {
    pub conn: DatabaseConnection,
}
