use sea_orm::ActiveValue;
use serde::{Deserialize, Serialize};
use validator::Validate;
#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct RolePermissionsDto {
    pub id: i64,
    pub role_id: i64,
    pub permission_id: i64,
}

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct RolePermissionsRequest {
    pub id: i64,
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

impl From<RolePermissionsRequest> for schemas::role_permissions::ActiveModel {
    fn from(request: RolePermissionsRequest) -> Self {
        Self {
            role_id: ActiveValue::Set(request.role_id),
            permission_id: ActiveValue::Set(request.permission_id),
        }
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
