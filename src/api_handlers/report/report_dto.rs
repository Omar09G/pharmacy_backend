use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, FromQueryResult)]
pub struct ReportListUserActive {
    pub id: i64,
    pub username: String,
    pub role: String,
}

impl ReportListUserActive {
    pub fn new(id: i64, username: String, role: String) -> Self {
        Self { id, username, role }
    }
}
