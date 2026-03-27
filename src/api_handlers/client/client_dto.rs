use sea_orm::{
    ActiveValue::{NotSet, Set},
    entity::prelude::*,
};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Validate, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClientDto {
    pub client_id: i64,
    pub client_address: Option<String>,
    pub client_city: Option<String>,
    pub client_country: Option<String>,
    pub client_date: Date,
    pub client_email: Option<String>,
    pub client_last_name: String,
    pub client_name: String,
    pub client_phone: Option<String>,
    pub client_postal_code: Option<String>,
    pub client_state: Option<String>,
    pub client_status: bool,
    pub client_type: Option<String>,
}

#[derive(Serialize, Deserialize, Validate, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClientRequest {
    pub client_id: i64,
    #[validate(length(min = 1, message = "Client address cannot be empty"))]
    pub client_address: Option<String>,
    #[validate(length(min = 1, message = "Client city cannot be empty"))]
    pub client_city: Option<String>,
    #[validate(length(min = 1, message = "Client country cannot be empty"))]
    pub client_country: Option<String>,
    pub client_date: Date,
    #[validate(email(message = "Invalid email format"))]
    pub client_email: Option<String>,
    #[validate(length(min = 1, message = "Client last name cannot be empty"))]
    pub client_last_name: String,
    #[validate(length(min = 1, message = "Client name cannot be empty"))]
    pub client_name: String,
    #[validate(length(min = 1, message = "Client phone cannot be empty"))]
    pub client_phone: Option<String>,
    #[validate(length(min = 1, message = "Client postal code cannot be empty"))]
    pub client_postal_code: Option<String>,
    #[validate(length(min = 1, message = "Client state cannot be empty"))]
    pub client_state: Option<String>,
    pub client_status: bool,
    #[validate(length(min = 1, message = "Client type cannot be empty"))]
    pub client_type: Option<String>,
}

#[derive(Serialize, Deserialize, Validate, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClientResponse {
    pub client_id: i64,
    pub client_address: Option<String>,
    pub client_city: Option<String>,
    pub client_last_name: String,
    pub client_name: String,
    pub client_phone: Option<String>,
    pub client_status: bool,
    pub client_type: Option<String>,
}

impl ClientResponse {
    pub fn new(
        client_id: i64,
        client_address: Option<String>,
        client_city: Option<String>,
        client_last_name: String,
        client_name: String,
        client_phone: Option<String>,
        client_status: bool,
        client_type: Option<String>,
    ) -> Self {
        Self {
            client_id,
            client_address,
            client_city,
            client_last_name,
            client_name,
            client_phone,
            client_status,
            client_type,
        }
    }
    pub fn from_client_dto(client_dto: ClientDto) -> Self {
        Self {
            client_id: client_dto.client_id,
            client_address: client_dto.client_address,
            client_city: client_dto.client_city,
            client_last_name: client_dto.client_last_name,
            client_name: client_dto.client_name,
            client_phone: client_dto.client_phone,
            client_status: client_dto.client_status,
            client_type: client_dto.client_type,
        }
    }
}

impl From<schemas::client::Model> for ClientResponse {
    fn from(model: schemas::client::Model) -> Self {
        Self {
            client_id: model.client_id,
            client_address: model.client_address,
            client_city: model.client_city,
            client_last_name: model.client_last_name,
            client_name: model.client_name,
            client_phone: model.client_phone,
            client_status: model.client_status,
            client_type: model.client_type,
        }
    }
}

impl From<ClientRequest> for schemas::client::ActiveModel {
    fn from(request: ClientRequest) -> Self {
        Self {
            client_id: NotSet, // Assuming client_id is auto-generated
            client_address: Set(request.client_address),
            client_city: Set(request.client_city),
            client_country: Set(request.client_country),
            client_date: Set(request.client_date),
            client_email: Set(request.client_email),
            client_last_name: Set(request.client_last_name),
            client_name: Set(request.client_name),
            client_phone: Set(request.client_phone),
            client_postal_code: Set(request.client_postal_code),
            client_state: Set(request.client_state),
            client_status: Set(request.client_status),
            client_type: Set(request.client_type),
        }
    }
}
