use sea_orm::ActiveValue;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CashJournalRequest {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub opening_amount: Decimal,
    pub opened_at: DateTimeWithTimeZone,
    pub closed_at: Option<DateTimeWithTimeZone>,
    pub opened_by: Option<i64>,
    pub closed_by: Option<i64>,
    pub status: String,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CashJournalIdResponse {
    pub id: i64,
}

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CashJournalDetailResponse {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub opening_amount: Decimal,
    pub opened_at: DateTimeWithTimeZone,
    pub closed_at: Option<DateTimeWithTimeZone>,
    pub opened_by: Option<i64>,
    pub closed_by: Option<i64>,
    pub status: String,
    pub created_at: DateTimeWithTimeZone,
}

impl TryFrom<CashJournalRequest> for schemas::cash_journals::ActiveModel {
    type Error = String;

    fn try_from(request: CashJournalRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            id: ActiveValue::NotSet,
            name: ActiveValue::Set(request.name),
            description: ActiveValue::Set(request.description),
            opening_amount: ActiveValue::Set(request.opening_amount),
            opened_at: ActiveValue::Set(request.opened_at),
            closed_at: ActiveValue::Set(request.closed_at),
            opened_by: ActiveValue::Set(request.opened_by),
            closed_by: ActiveValue::Set(request.closed_by),
            status: ActiveValue::Set(request.status),
            created_at: ActiveValue::Set(request.created_at),
        })
    }
}

impl From<schemas::cash_journals::Model> for CashJournalDetailResponse {
    fn from(model: schemas::cash_journals::Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
            description: model.description,
            opening_amount: model.opening_amount,
            opened_at: model.opened_at,
            closed_at: model.closed_at,
            opened_by: model.opened_by,
            closed_by: model.closed_by,
            status: model.status,
            created_at: model.created_at,
        }
    }
}

impl From<schemas::cash_journals::ActiveModel> for CashJournalDetailResponse {
    fn from(model: schemas::cash_journals::ActiveModel) -> Self {
        Self {
            id: model.id.unwrap(),
            name: model.name.unwrap(),
            description: model.description.unwrap(),
            opening_amount: model.opening_amount.unwrap(),
            opened_at: model.opened_at.unwrap(),
            closed_at: model.closed_at.unwrap(),
            opened_by: model.opened_by.unwrap(),
            closed_by: model.closed_by.unwrap(),
            status: model.status.unwrap(),
            created_at: model.created_at.unwrap(),
        }
    }
}

impl From<schemas::cash_journals::Model> for CashJournalIdResponse {
    fn from(model: schemas::cash_journals::Model) -> Self {
        Self { id: model.id }
    }
}

impl From<schemas::cash_journals::ActiveModel> for CashJournalIdResponse {
    fn from(model: schemas::cash_journals::ActiveModel) -> Self {
        Self {
            id: model.id.unwrap(),
        }
    }
}
