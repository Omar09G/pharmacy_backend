use sea_orm::ActiveValue;
use serde::{Deserialize, Serialize};
use validator::Validate;
#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct RolePermissionsDto {
    pub role_id: i64,
    pub permission_id: i64,
}

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct RolePermissionsRequest {
    #[validate(range(min = 1, message = "Role ID must be a positive integer"))]
    pub role_id: i64,
    #[validate(range(min = 1, message = "Permission ID must be a positive integer"))]
    pub permission_id: i64,
}

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct RolePermissionsResponse {
    pub role_id: i64,
    pub permission_id: i64,
}

impl RolePermissionsResponse {
    pub fn new(role_id: i64, permission_id: i64) -> Self {
        Self {
            role_id,
            permission_id,
        }
    }
}

impl TryFrom<RolePermissionsRequest> for schemas::role_permissions::ActiveModel {
    type Error = String;

    fn try_from(request: RolePermissionsRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            role_id: ActiveValue::Set(request.role_id),
            permission_id: ActiveValue::Set(request.permission_id),
        })
    }
}

impl From<schemas::role_permissions::Model> for RolePermissionsResponse {
    fn from(model: schemas::role_permissions::Model) -> Self {
        Self {
            role_id: model.role_id,
            permission_id: model.permission_id,
        }
    }
}
impl From<schemas::role_permissions::ActiveModel> for RolePermissionsResponse {
    fn from(model: schemas::role_permissions::ActiveModel) -> Self {
        Self {
            role_id: model.role_id.unwrap(),
            permission_id: model.permission_id.unwrap(),
        }
    }
}
