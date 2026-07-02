use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(length(min = 1))]
    pub identity: String,

    #[validate(length(min = 1))]
    pub password: String,

    pub device_id: String,
}
