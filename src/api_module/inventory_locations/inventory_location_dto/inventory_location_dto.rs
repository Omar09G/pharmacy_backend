use sea_orm::ActiveValue;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct InventoryLocationDto {
    pub id: i64,
    pub name: String,
    pub r#type: Option<String>,
    pub description: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct InventoryLocationRequest {
    pub id: i64,
    #[validate(length(min = 1, message = "Name must not be empty"))]
    pub name: String,
    pub r#type: Option<String>,
    pub description: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct InventoryLocationResponse {
    pub id: i64,
    #[validate(length(min = 1, message = "Name must not be empty"))]
    pub name: String,
    pub r#type: Option<String>,
    pub description: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct InventoryLocationIdResponse {
    pub id: i64,
}

impl From<InventoryLocationRequest> for schemas::inventory_locations::ActiveModel {
    fn from(request: InventoryLocationRequest) -> Self {
        Self {
            id: ActiveValue::NotSet,
            name: ActiveValue::Set(request.name),
            r#type: ActiveValue::Set(request.r#type),
            description: ActiveValue::Set(request.description),
        }
    }
}

impl From<schemas::inventory_locations::Model> for InventoryLocationResponse {
    fn from(model: schemas::inventory_locations::Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
            r#type: model.r#type,
            description: model.description,
        }
    }
}

impl From<schemas::inventory_locations::ActiveModel> for InventoryLocationResponse {
    fn from(model: schemas::inventory_locations::ActiveModel) -> Self {
        Self {
            id: model.id.unwrap(),
            name: model.name.unwrap(),
            r#type: model.r#type.unwrap(),
            description: model.description.unwrap(),
        }
    }
}

impl From<schemas::inventory_locations::Model> for InventoryLocationIdResponse {
    fn from(model: schemas::inventory_locations::Model) -> Self {
        Self { id: model.id }
    }
}

impl From<schemas::inventory_locations::ActiveModel> for InventoryLocationIdResponse {
    fn from(model: schemas::inventory_locations::ActiveModel) -> Self {
        Self {
            id: model.id.unwrap(),
        }
    }
}
