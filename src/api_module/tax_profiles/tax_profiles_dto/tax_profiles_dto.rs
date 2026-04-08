use sea_orm::ActiveValue;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TaxProfileRequest {
    #[validate(length(min = 1, message = "Name must not be empty"))]
    pub name: String,
    pub rate: Decimal,
    pub is_inclusive: bool,
    #[validate(length(max = 255, message = "Description must be at most 255 characters"))]
    pub description: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TaxProfileResponse {
    pub id: i64,
    pub name: String,
    pub rate: Decimal,
    pub is_inclusive: bool,
    pub description: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct TaxProfileIdResponse {
    pub id: i64,
}

impl TryFrom<TaxProfileRequest> for schemas::tax_profiles::ActiveModel {
    type Error = String;

    fn try_from(request: TaxProfileRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            id: ActiveValue::NotSet,
            name: ActiveValue::Set(request.name),
            rate: ActiveValue::Set(request.rate),
            is_inclusive: ActiveValue::Set(request.is_inclusive),
            description: ActiveValue::Set(request.description),
        })
    }
}

impl From<schemas::tax_profiles::Model> for TaxProfileResponse {
    fn from(model: schemas::tax_profiles::Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
            rate: model.rate,
            is_inclusive: model.is_inclusive,
            description: model.description,
        }
    }
}

impl From<schemas::tax_profiles::ActiveModel> for TaxProfileResponse {
    fn from(model: schemas::tax_profiles::ActiveModel) -> Self {
        Self {
            id: model.id.unwrap(),
            name: model.name.unwrap(),
            rate: model.rate.unwrap(),
            is_inclusive: model.is_inclusive.unwrap(),
            description: model.description.unwrap(),
        }
    }
}

impl From<schemas::tax_profiles::ActiveModel> for TaxProfileIdResponse {
    fn from(model: schemas::tax_profiles::ActiveModel) -> Self {
        Self {
            id: model.id.unwrap(),
        }
    }
}
