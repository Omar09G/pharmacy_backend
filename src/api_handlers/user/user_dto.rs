use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserDTO {
    pub id: i64,
    pub country: Option<String>,
    pub firstname: Option<String>,
    pub lastname: String,
    pub password: Option<String>,
    pub role: Option<String>,
    pub username: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserResponseDTO {
    pub firstname: Option<String>,
    pub lastname: String,
    pub role: Option<String>,
    pub username: String,
}

#[derive(Deserialize, Validate, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserRequestDTO {
    pub country: Option<String>,
    pub firstname: Option<String>,
    #[validate(length(min = 3, max = 10, message = "Name must be at least 3 characters long"))]
    pub lastname: String,
    pub password: Option<String>,
    pub role: Option<String>,
    #[validate(length(
        min = 3,
        max = 15,
        message = "Username must be at least 3 characters long"
    ))]
    pub username: String,
}

impl From<schemas::user::ActiveModel> for UserResponseDTO {
    fn from(active_model: schemas::user::ActiveModel) -> Self {
        UserResponseDTO {
            firstname: active_model.firstname.unwrap(),
            lastname: active_model.lastname.unwrap(),
            role: active_model.role.unwrap(),
            username: active_model.username.unwrap(),
        }
    }
}

impl From<schemas::user::Model> for UserResponseDTO {
    fn from(model: schemas::user::Model) -> Self {
        UserResponseDTO {
            firstname: model.firstname,
            lastname: model.lastname,
            role: model.role,
            username: model.username,
        }
    }
}
