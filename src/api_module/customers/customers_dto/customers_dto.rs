use sea_orm::ActiveValue;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CustomerDto {
    pub id: i64,
    pub name: String,
    pub document_id: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub billing_address: Option<String>,
    pub credit_limit: Option<Decimal>,
    pub terms_days: Option<i32>,
    pub status: String,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CustomerRequest {
    pub id: i64,
    pub name: String,
    pub document_id: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub billing_address: Option<String>,
    pub credit_limit: Option<Decimal>,
    pub terms_days: Option<i32>,
    pub status: String,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CustomerIdResponse {
    pub id: i64,
}

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CustomerDetailResponse {
    pub id: i64,
    pub name: String,
    pub document_id: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub billing_address: Option<String>,
    pub credit_limit: Option<Decimal>,
    pub terms_days: Option<i32>,
    pub status: String,
    pub created_at: DateTimeWithTimeZone,
}

impl From<CustomerRequest> for schemas::customers::ActiveModel {
    fn from(request: CustomerRequest) -> Self {
        Self {
            id: ActiveValue::NotSet,
            name: ActiveValue::Set(request.name),
            document_id: ActiveValue::Set(request.document_id),
            phone: ActiveValue::Set(request.phone),
            email: ActiveValue::Set(request.email),
            billing_address: ActiveValue::Set(request.billing_address),
            credit_limit: ActiveValue::Set(request.credit_limit),
            terms_days: ActiveValue::Set(request.terms_days),
            status: ActiveValue::Set(request.status),
            created_at: ActiveValue::Set(request.created_at),
        }
    }
}

impl From<schemas::customers::Model> for CustomerDetailResponse {
    fn from(model: schemas::customers::Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
            document_id: model.document_id,
            phone: model.phone,
            email: model.email,
            billing_address: model.billing_address,
            credit_limit: model.credit_limit,
            terms_days: model.terms_days,
            status: model.status,
            created_at: model.created_at,
        }
    }
}

impl From<schemas::customers::ActiveModel> for CustomerDetailResponse {
    fn from(model: schemas::customers::ActiveModel) -> Self {
        Self {
            id: model.id.unwrap(),
            name: model.name.unwrap(),
            document_id: model.document_id.unwrap(),
            phone: model.phone.unwrap(),
            email: model.email.unwrap(),
            billing_address: model.billing_address.unwrap(),
            credit_limit: model.credit_limit.unwrap(),
            terms_days: model.terms_days.unwrap(),
            status: model.status.unwrap(),
            created_at: model.created_at.unwrap(),
        }
    }
}

impl From<schemas::customers::Model> for CustomerIdResponse {
    fn from(model: schemas::customers::Model) -> Self {
        Self { id: model.id }
    }
}

impl From<schemas::customers::ActiveModel> for CustomerIdResponse {
    fn from(model: schemas::customers::ActiveModel) -> Self {
        Self {
            id: model.id.unwrap(),
        }
    }
}
