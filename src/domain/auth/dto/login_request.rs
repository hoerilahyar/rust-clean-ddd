use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

use crate::common::extractor::RequiredFields;

#[derive(Debug, Clone, Deserialize, Validate, ToSchema)]
pub struct LoginRequest {
    #[validate(length(min = 1))]
    pub identity: String,

    #[validate(length(min = 1))]
    pub password: String,

    pub device_id: String,
}

impl RequiredFields for LoginRequest {
    fn required_fields() -> &'static [&'static str] {
        &["identity", "password", "device_id"]
    }
}
