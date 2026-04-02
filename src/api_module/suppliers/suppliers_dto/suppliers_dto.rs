use sea_orm::ActiveValue;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::api_utils::api_utils_fun::{
    get_current_timestamp_at_zone_mexico, get_current_timestamp_now,
};

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SupplierDto {
    pub id: i64,
    pub name: String,
    pub tax_id: Option<String>,
    pub contact_person: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
    pub notes: Option<String>,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SupplierRequest {
    pub id: i64,
    pub name: String,
    pub tax_id: Option<String>,
    pub contact_person: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
    pub notes: Option<String>,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SupplierIdResponse {
    pub id: i64,
}

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct SupplierDetailResponse {
    pub id: i64,
    pub name: String,
    pub tax_id: Option<String>,
    pub contact_person: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
    pub notes: Option<String>,
    pub created_at: DateTimeWithTimeZone,
}

impl From<SupplierRequest> for schemas::suppliers::ActiveModel {
    fn from(request: SupplierRequest) -> Self {
        Self {
            id: ActiveValue::NotSet,
            name: ActiveValue::Set(request.name),
            tax_id: ActiveValue::Set(request.tax_id),
            contact_person: ActiveValue::Set(request.contact_person),
            phone: ActiveValue::Set(request.phone),
            email: ActiveValue::Set(request.email),
            address: ActiveValue::Set(request.address),
            notes: ActiveValue::Set(request.notes),
            created_at: ActiveValue::Set(get_current_timestamp_now()),
        }
    }
}

impl From<schemas::suppliers::Model> for SupplierDetailResponse {
    fn from(model: schemas::suppliers::Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
            tax_id: model.tax_id,
            contact_person: model.contact_person,
            phone: model.phone,
            email: model.email,
            address: model.address,
            notes: model.notes,
            created_at: get_current_timestamp_at_zone_mexico(model.created_at),
        }
    }
}

impl From<schemas::suppliers::ActiveModel> for SupplierDetailResponse {
    fn from(model: schemas::suppliers::ActiveModel) -> Self {
        Self {
            id: model.id.unwrap(),
            name: model.name.unwrap(),
            tax_id: model.tax_id.unwrap(),
            contact_person: model.contact_person.unwrap(),
            phone: model.phone.unwrap(),
            email: model.email.unwrap(),
            address: model.address.unwrap(),
            notes: model.notes.unwrap(),
            created_at: get_current_timestamp_at_zone_mexico(model.created_at.unwrap()),
        }
    }
}

impl From<schemas::suppliers::Model> for SupplierIdResponse {
    fn from(model: schemas::suppliers::Model) -> Self {
        Self { id: model.id }
    }
}

impl From<schemas::suppliers::ActiveModel> for SupplierIdResponse {
    fn from(model: schemas::suppliers::ActiveModel) -> Self {
        Self {
            id: model.id.unwrap(),
        }
    }
}
