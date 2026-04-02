use sea_orm::ActiveValue;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CategoryDto {
    pub id: i64,
    pub name: String,
    pub parent_id: Option<i64>,
    pub description: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CategoryRequest {
    pub id: i64,
    pub name: String,
    pub parent_id: Option<i64>,
    pub description: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CategoryIdResponse {
    pub id: i64,
}

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CategoryDetailResponse {
    pub id: i64,
    pub name: String,
    pub parent_id: Option<i64>,
    pub description: Option<String>,
}

impl From<CategoryRequest> for schemas::categories::ActiveModel {
    fn from(request: CategoryRequest) -> Self {
        Self {
            id: ActiveValue::NotSet,
            name: ActiveValue::Set(request.name),
            parent_id: ActiveValue::NotSet,
            description: ActiveValue::Set(request.description),
        }
    }
}

impl From<schemas::categories::Model> for CategoryDetailResponse {
    fn from(model: schemas::categories::Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
            parent_id: model.parent_id,
            description: model.description,
        }
    }
}

impl From<schemas::categories::ActiveModel> for CategoryDetailResponse {
    fn from(model: schemas::categories::ActiveModel) -> Self {
        Self {
            id: model.id.unwrap(),
            name: model.name.unwrap(),
            parent_id: model.parent_id.unwrap(),
            description: model.description.unwrap(),
        }
    }
}

impl From<schemas::categories::Model> for CategoryIdResponse {
    fn from(model: schemas::categories::Model) -> Self {
        Self { id: model.id }
    }
}

impl From<schemas::categories::ActiveModel> for CategoryIdResponse {
    fn from(model: schemas::categories::ActiveModel) -> Self {
        Self {
            id: model.id.unwrap(),
        }
    }
}
