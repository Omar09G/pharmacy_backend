use sea_orm::{ActiveValue, entity::prelude::*};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    api_utils::api_utils_fun::{get_current_timestamp_at_zone_mexico, get_current_timestamp_now},
    config::config_pass::config_password::generate_hash,
};

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UserDto {
    pub id: i64,
    pub username: String,
    pub password_hash: String,
    pub full_name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub status: String,
    pub created_at: DateTimeWithTimeZone,
    pub created_by: Option<i64>,
    pub updated_at: Option<DateTimeWithTimeZone>,
    pub updated_by: Option<i64>,
    pub deleted_at: Option<DateTimeWithTimeZone>,
}

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UserRequestDto {
    pub id: i64,
    #[validate(
        length(
            min = 3,
            max = 15,
            message = "Username must be at least 3 characters long"
        ),
        custom(function = "crate::api_utils::api_utils_fun::validate_special_chars")
    )]
    pub username: String,
    #[validate(length(min = 6, message = "Password must be at least 6 characters long"))]
    pub password_hash: String,
    #[validate(length(
        min = 3,
        max = 50,
        message = "Full name must be between 3 and 50 characters long"
    ))]
    pub full_name: Option<String>,
    #[validate(email(message = "Email must be a valid email address"))]
    pub email: Option<String>,
    pub phone: Option<String>,
    #[validate(length(
        min = 3,
        max = 20,
        message = "Status must be between 3 and 20 characters long"
    ))]
    pub status: String,
    pub created_at: DateTimeWithTimeZone,
    pub created_by: Option<i64>,
    pub updated_at: Option<DateTimeWithTimeZone>,
    pub updated_by: Option<i64>,
    pub deleted_at: Option<DateTimeWithTimeZone>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UserResponse {
    pub id: i64,
    pub username: String,
    pub full_name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub status: String,
    pub created_at: DateTimeWithTimeZone,
    pub created_by: Option<i64>,
    pub updated_at: Option<DateTimeWithTimeZone>,
    pub updated_by: Option<i64>,
    pub deleted_at: Option<DateTimeWithTimeZone>,
}

impl From<UserRequestDto> for schemas::users::ActiveModel {
    fn from(dto: UserRequestDto) -> Self {
        Self {
            id: ActiveValue::NotSet,
            username: ActiveValue::Set(dto.username),
            password_hash: ActiveValue::Set(generate_hash(&dto.password_hash).unwrap()),
            full_name: ActiveValue::Set(dto.full_name),
            email: ActiveValue::Set(dto.email),
            phone: ActiveValue::Set(dto.phone),
            status: ActiveValue::Set(dto.status),
            created_at: ActiveValue::Set(get_current_timestamp_now()),
            created_by: ActiveValue::NotSet,
            updated_at: ActiveValue::NotSet,
            updated_by: ActiveValue::NotSet,
            deleted_at: ActiveValue::NotSet,
        }
    }
}

impl From<schemas::users::Model> for UserResponse {
    fn from(model: schemas::users::Model) -> Self {
        Self {
            id: model.id,
            username: model.username,
            full_name: model.full_name,
            email: model.email,
            phone: model.phone,
            status: model.status,
            created_at: get_current_timestamp_at_zone_mexico(model.created_at),
            created_by: model.created_by,
            updated_at: model
                .updated_at
                .map(|dt| get_current_timestamp_at_zone_mexico(dt)),
            updated_by: model.updated_by,
            deleted_at: model
                .deleted_at
                .map(|dt| get_current_timestamp_at_zone_mexico(dt)),
        }
    }
}

impl From<schemas::users::ActiveModel> for UserResponse {
    fn from(model: schemas::users::ActiveModel) -> Self {
        Self {
            id: model.id.unwrap(),
            username: model.username.unwrap(),
            full_name: model.full_name.unwrap(),
            email: model.email.unwrap(),
            phone: model.phone.unwrap(),
            status: model.status.unwrap(),
            created_at: get_current_timestamp_at_zone_mexico(model.created_at.unwrap()),
            created_by: model.created_by.unwrap(),
            updated_at: model
                .updated_at
                .unwrap()
                .map(|dt| get_current_timestamp_at_zone_mexico(dt)),
            updated_by: model.updated_by.unwrap(),
            deleted_at: model
                .deleted_at
                .unwrap()
                .map(|dt| get_current_timestamp_at_zone_mexico(dt)),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UserChangePasswordRequest {
    #[validate(length(min = 6, message = "username must be at least 6 characters long"))]
    pub username: String,
    #[validate(length(min = 6, message = "Password must be at least 6 characters long"))]
    pub password: String,
    pub updated_by: i64,
}

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UserChangeStatusRequest {
    #[validate(length(min = 6, message = "username must be at least 6 characters long"))]
    pub username: String,
    #[validate(length(
        min = 3,
        max = 20,
        message = "Status must be between 3 and 20 characters long"
    ))]
    pub status: String,
    pub updated_by: i64,
}

#[derive(Deserialize, Serialize, Debug, Validate)]
#[serde(rename_all = "camelCase")]
pub struct UserUpdateRequestDto {
    pub id: i64,
    #[validate(
        length(
            min = 3,
            max = 15,
            message = "Username must be at least 3 characters long"
        ),
        custom(function = "crate::api_utils::api_utils_fun::validate_special_chars")
    )]
    pub full_name: Option<String>,
    #[validate(email(message = "Email must be a valid email address"))]
    pub email: Option<String>,
    pub phone: Option<String>,
    pub updated_by: i64,
}
