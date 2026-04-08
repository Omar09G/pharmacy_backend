use sea_orm::ActiveValue;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AuditLogRequest {
    pub entity_type: String,
    pub table_name: Option<String>,
    pub entity_id: Option<i64>,
    pub action: String,
    pub changed_by: Option<i64>,
    pub changed_at: DateTimeWithTimeZone,
    pub change_data: Option<Json>,
}

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AuditLogIdResponse {
    pub id: i64,
}

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct AuditLogDetailResponse {
    pub id: i64,
    pub entity_type: String,
    pub table_name: Option<String>,
    pub entity_id: Option<i64>,
    pub action: String,
    pub changed_by: Option<i64>,
    pub changed_at: DateTimeWithTimeZone,
    pub change_data: Option<Json>,
}

impl TryFrom<AuditLogRequest> for schemas::audit_log::ActiveModel {
    type Error = String;

    fn try_from(request: AuditLogRequest) -> Result<Self, Self::Error> {
        Ok(Self {
            id: ActiveValue::NotSet,
            entity_type: ActiveValue::Set(request.entity_type),
            table_name: ActiveValue::Set(request.table_name),
            entity_id: ActiveValue::Set(request.entity_id),
            action: ActiveValue::Set(request.action),
            changed_by: ActiveValue::Set(request.changed_by),
            changed_at: ActiveValue::Set(request.changed_at),
            change_data: ActiveValue::Set(request.change_data),
        })
    }
}

impl From<schemas::audit_log::Model> for AuditLogDetailResponse {
    fn from(model: schemas::audit_log::Model) -> Self {
        Self {
            id: model.id,
            entity_type: model.entity_type,
            table_name: model.table_name,
            entity_id: model.entity_id,
            action: model.action,
            changed_by: model.changed_by,
            changed_at: model.changed_at,
            change_data: model.change_data,
        }
    }
}

impl From<schemas::audit_log::ActiveModel> for AuditLogDetailResponse {
    fn from(model: schemas::audit_log::ActiveModel) -> Self {
        Self {
            id: model.id.unwrap(),
            entity_type: model.entity_type.unwrap(),
            table_name: model.table_name.unwrap(),
            entity_id: model.entity_id.unwrap(),
            action: model.action.unwrap(),
            changed_by: model.changed_by.unwrap(),
            changed_at: model.changed_at.unwrap(),
            change_data: model.change_data.unwrap(),
        }
    }
}

impl From<schemas::audit_log::Model> for AuditLogIdResponse {
    fn from(model: schemas::audit_log::Model) -> Self {
        Self { id: model.id }
    }
}

impl From<schemas::audit_log::ActiveModel> for AuditLogIdResponse {
    fn from(model: schemas::audit_log::ActiveModel) -> Self {
        Self {
            id: model.id.unwrap(),
        }
    }
}
