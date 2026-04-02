use sea_orm::ActiveValue;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CashEntryRequest {
    pub id: i64,
    pub name: String,
    pub entry_type: String,
    pub amount: Decimal,
    pub method_id: Option<i64>,
    pub related_type: Option<String>,
    pub related_id: Option<i64>,
    pub description: Option<String>,
    pub recorded_at: DateTimeWithTimeZone,
    pub recorded_by: Option<i64>,
}

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CashEntryIdResponse {
    pub id: i64,
}

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CashEntryDetailResponse {
    pub id: i64,
    pub name: String,
    pub entry_type: String,
    pub amount: Decimal,
    pub method_id: Option<i64>,
    pub related_type: Option<String>,
    pub related_id: Option<i64>,
    pub description: Option<String>,
    pub recorded_at: DateTimeWithTimeZone,
    pub recorded_by: Option<i64>,
}

impl From<CashEntryRequest> for schemas::cash_entries::ActiveModel {
    fn from(request: CashEntryRequest) -> Self {
        Self {
            id: ActiveValue::NotSet,
            name: ActiveValue::Set(request.name),
            entry_type: ActiveValue::Set(request.entry_type),
            amount: ActiveValue::Set(request.amount),
            method_id: ActiveValue::Set(request.method_id),
            related_type: ActiveValue::Set(request.related_type),
            related_id: ActiveValue::Set(request.related_id),
            description: ActiveValue::Set(request.description),
            recorded_at: ActiveValue::Set(request.recorded_at),
            recorded_by: ActiveValue::Set(request.recorded_by),
        }
    }
}

impl From<schemas::cash_entries::Model> for CashEntryDetailResponse {
    fn from(model: schemas::cash_entries::Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
            entry_type: model.entry_type,
            amount: model.amount,
            method_id: model.method_id,
            related_type: model.related_type,
            related_id: model.related_id,
            description: model.description,
            recorded_at: model.recorded_at,
            recorded_by: model.recorded_by,
        }
    }
}

impl From<schemas::cash_entries::ActiveModel> for CashEntryDetailResponse {
    fn from(model: schemas::cash_entries::ActiveModel) -> Self {
        Self {
            id: model.id.unwrap(),
            name: model.name.unwrap(),
            entry_type: model.entry_type.unwrap(),
            amount: model.amount.unwrap(),
            method_id: model.method_id.unwrap(),
            related_type: model.related_type.unwrap(),
            related_id: model.related_id.unwrap(),
            description: model.description.unwrap(),
            recorded_at: model.recorded_at.unwrap(),
            recorded_by: model.recorded_by.unwrap(),
        }
    }
}

impl From<schemas::cash_entries::Model> for CashEntryIdResponse {
    fn from(model: schemas::cash_entries::Model) -> Self {
        Self { id: model.id }
    }
}

impl From<schemas::cash_entries::ActiveModel> for CashEntryIdResponse {
    fn from(model: schemas::cash_entries::ActiveModel) -> Self {
        Self {
            id: model.id.unwrap(),
        }
    }
}
