use sea_orm::ActiveValue;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PaymentMethodDto {
    pub id: i64,
    pub name: String,
    method_type: Option<String>,
    pub active: bool,
}

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PaymentMethodResponse {
    pub id: i64,
    pub name: String,
    method_type: Option<String>,
    pub active: bool,
}

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PaymentMethodIdResponse {
    pub id: i64,
}

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PaymentMethodRequest {
    #[validate(length(min = 1, message = "Name must not be empty"))]
    pub name: String,
    pub method_type: Option<String>,
    pub active: bool,
}

impl From<PaymentMethodRequest> for schemas::payment_methods::ActiveModel {
    fn from(request: PaymentMethodRequest) -> Self {
        Self {
            name: ActiveValue::Set(request.name),
            method_type: ActiveValue::Set(request.method_type),
            active: ActiveValue::Set(request.active),
            id: ActiveValue::NotSet,
        }
    }
}

impl From<schemas::payment_methods::Model> for PaymentMethodResponse {
    fn from(model: schemas::payment_methods::Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
            method_type: model.method_type,
            active: model.active,
        }
    }
}

impl From<schemas::payment_methods::ActiveModel> for PaymentMethodResponse {
    fn from(model: schemas::payment_methods::ActiveModel) -> Self {
        Self {
            id: model.id.unwrap(),
            name: model.name.unwrap(),
            method_type: model.method_type.unwrap(),
            active: model.active.unwrap(),
        }
    }
}

impl From<schemas::payment_methods::Model> for PaymentMethodIdResponse {
    fn from(model: schemas::payment_methods::Model) -> Self {
        Self { id: model.id }
    }
}

impl From<schemas::payment_methods::ActiveModel> for PaymentMethodIdResponse {
    fn from(model: schemas::payment_methods::ActiveModel) -> Self {
        Self {
            id: model.id.unwrap(),
        }
    }
}
