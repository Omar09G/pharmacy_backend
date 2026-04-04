use sea_orm::ActiveValue;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CustomerCreditAccountRequest {
    pub id: i64,
    pub customer_id: i64,
    pub balance: Decimal,
    pub limit_amount: Option<Decimal>,
    pub last_overdue_date: Option<Date>,
}

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CustomerCreditAccountIdResponse {
    pub id: i64,
}

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct CustomerCreditAccountDetailResponse {
    pub id: i64,
    pub customer_id: i64,
    pub balance: Decimal,
    pub limit_amount: Option<Decimal>,
    pub last_overdue_date: Option<Date>,
}

impl TryFrom<CustomerCreditAccountRequest> for schemas::customer_credit_accounts::ActiveModel {
    type Error = String;

    fn try_from(request: CustomerCreditAccountRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            id: ActiveValue::NotSet,
            customer_id: ActiveValue::Set(request.customer_id),
            balance: ActiveValue::Set(request.balance),
            limit_amount: ActiveValue::Set(request.limit_amount),
            last_overdue_date: ActiveValue::Set(request.last_overdue_date),
        })
    }
}

impl From<schemas::customer_credit_accounts::Model> for CustomerCreditAccountDetailResponse {
    fn from(model: schemas::customer_credit_accounts::Model) -> Self {
        Self {
            id: model.id,
            customer_id: model.customer_id,
            balance: model.balance,
            limit_amount: model.limit_amount,
            last_overdue_date: model.last_overdue_date,
        }
    }
}

impl From<schemas::customer_credit_accounts::ActiveModel> for CustomerCreditAccountDetailResponse {
    fn from(model: schemas::customer_credit_accounts::ActiveModel) -> Self {
        Self {
            id: model.id.unwrap(),
            customer_id: model.customer_id.unwrap(),
            balance: model.balance.unwrap(),
            limit_amount: model.limit_amount.unwrap(),
            last_overdue_date: model.last_overdue_date.unwrap(),
        }
    }
}

impl From<schemas::customer_credit_accounts::Model> for CustomerCreditAccountIdResponse {
    fn from(model: schemas::customer_credit_accounts::Model) -> Self {
        Self { id: model.id }
    }
}

impl From<schemas::customer_credit_accounts::ActiveModel> for CustomerCreditAccountIdResponse {
    fn from(model: schemas::customer_credit_accounts::ActiveModel) -> Self {
        Self {
            id: model.id.unwrap(),
        }
    }
}
