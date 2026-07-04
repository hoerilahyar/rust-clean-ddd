use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct LoginRequest {
    #[validate(length(min = 1))]
    pub identity: String,

    #[validate(length(min = 1))]
    pub password: String,

    pub device_id: String,
}
