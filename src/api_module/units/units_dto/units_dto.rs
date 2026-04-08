use sea_orm::ActiveValue;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UnitResponse {
    pub id: i64,
    pub code: String,
    pub name: String,
    pub precision: i32,
}

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UnitIdResponse {
    pub id: i64,
}

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UnitRequest {
    #[validate(length(min = 1, message = "Code must not be empty"))]
    pub code: String,
    #[validate(length(min = 1, message = "Name must not be empty"))]
    pub name: String,
    #[validate(range(min = 0, message = "Precision must be non-negative"))]
    pub precision: i32,
}

impl TryFrom<UnitRequest> for schemas::units::ActiveModel {
    type Error = String;

    fn try_from(request: UnitRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            id: ActiveValue::NotSet,
            code: ActiveValue::Set(request.code),
            name: ActiveValue::Set(request.name),
            precision: ActiveValue::Set(request.precision),
        })
    }
}

impl From<schemas::units::Model> for UnitResponse {
    fn from(model: schemas::units::Model) -> Self {
        Self {
            id: model.id,
            code: model.code,
            name: model.name,
            precision: model.precision,
        }
    }
}

impl From<schemas::units::ActiveModel> for UnitResponse {
    fn from(model: schemas::units::ActiveModel) -> Self {
        Self {
            id: model.id.unwrap(),
            code: model.code.unwrap(),
            name: model.name.unwrap(),
            precision: model.precision.unwrap(),
        }
    }
}

impl From<schemas::units::ActiveModel> for UnitIdResponse {
    fn from(model: schemas::units::ActiveModel) -> Self {
        Self {
            id: model.id.unwrap(),
        }
    }
}
