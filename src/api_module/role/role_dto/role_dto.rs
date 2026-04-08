use sea_orm::ActiveValue;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct RoleRequest {
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct RoleIdResponse {
    pub id: i64,
}

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct RoleDetailResponse {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTimeWithTimeZone,
}
impl From<RoleRequest> for schemas::roles::ActiveModel {
    fn from(request: RoleRequest) -> Self {
        Self {
            id: ActiveValue::NotSet,
            name: ActiveValue::Set(request.name),
            description: ActiveValue::Set(request.description),
            created_at: ActiveValue::Set(request.created_at),
        }
    }
}

impl From<schemas::roles::Model> for RoleDetailResponse {
    fn from(model: schemas::roles::Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
            description: model.description,
            created_at: model.created_at,
        }
    }
}

impl From<schemas::roles::ActiveModel> for RoleDetailResponse {
    fn from(model: schemas::roles::ActiveModel) -> Self {
        Self {
            id: model.id.unwrap(),
            name: model.name.unwrap(),
            description: model.description.unwrap(),
            created_at: model.created_at.unwrap(),
        }
    }
}

impl From<schemas::roles::Model> for RoleIdResponse {
    fn from(model: schemas::roles::Model) -> Self {
        Self { id: model.id }
    }
}

impl From<schemas::roles::ActiveModel> for RoleIdResponse {
    fn from(model: schemas::roles::ActiveModel) -> Self {
        Self {
            id: model.id.unwrap(),
        }
    }
}
