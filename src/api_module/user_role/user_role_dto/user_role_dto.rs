use sea_orm::ActiveValue;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UserRoleDto {
    pub user_id: i64,
    pub role_id: i64,
}

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UserRoleRequest {
    #[validate(range(min = 1, message = "User ID must be provided"))]
    pub user_id: i64,
    #[validate(range(min = 1, message = "Role ID must be provided"))]
    pub role_id: i64,
}

impl From<UserRoleRequest> for schemas::user_roles::ActiveModel {
    fn from(request: UserRoleRequest) -> Self {
        Self {
            user_id: ActiveValue::Set(request.user_id),
            role_id: ActiveValue::Set(request.role_id),
        }
    }
}

impl From<schemas::user_roles::Model> for UserRoleDto {
    fn from(model: schemas::user_roles::Model) -> Self {
        Self {
            user_id: model.user_id,
            role_id: model.role_id,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UserRoleResponse {
    pub user_id: i64,
    pub role_id: i64,
}
impl From<schemas::user_roles::Model> for UserRoleResponse {
    fn from(model: schemas::user_roles::Model) -> Self {
        Self {
            user_id: model.user_id,
            role_id: model.role_id,
        }
    }
}

impl From<schemas::user_roles::ActiveModel> for UserRoleResponse {
    fn from(model: schemas::user_roles::ActiveModel) -> Self {
        Self {
            user_id: model.user_id.unwrap(),
            role_id: model.role_id.unwrap(),
        }
    }
}
