use sea_orm::ActiveValue;
use serde::{Deserialize, Serialize};
use validator::Validate;
#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PermissionDto {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PermissionRequest {
    #[validate(length(
        min = 3,
        max = 50,
        message = "Permission name must be between 3 and 50 characters long"
    ))]
    pub name: String,
    #[validate(length(max = 255, message = "Description must be at most 255 characters long"))]
    pub description: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PermissionResponse {
    pub id: i64,
}

impl TryFrom<PermissionRequest> for schemas::permissions::ActiveModel {
    type Error = String;

    fn try_from(request: PermissionRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            id: ActiveValue::NotSet,
            name: ActiveValue::Set(request.name),
            description: ActiveValue::Set(request.description),
        })
    }
}

impl From<schemas::permissions::Model> for PermissionResponse {
    fn from(model: schemas::permissions::Model) -> Self {
        Self { id: model.id }
    }
}

impl From<schemas::permissions::ActiveModel> for PermissionResponse {
    fn from(model: schemas::permissions::ActiveModel) -> Self {
        Self {
            id: model.id.unwrap(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PermissionDetailResponse {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
}
impl From<schemas::permissions::Model> for PermissionDetailResponse {
    fn from(model: schemas::permissions::Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
            description: model.description,
        }
    }
}

impl From<schemas::permissions::ActiveModel> for PermissionDetailResponse {
    fn from(model: schemas::permissions::ActiveModel) -> Self {
        Self {
            id: model.id.unwrap(),
            name: model.name.unwrap(),
            description: model.description.unwrap(),
        }
    }
}
