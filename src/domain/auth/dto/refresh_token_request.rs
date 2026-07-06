use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

use crate::common::extractor::RequiredFields;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct RefreshTokenRequest {
    #[validate(length(min = 1))]
    pub refresh_token: String,

    pub device_id: String,
}

impl RequiredFields for RefreshTokenRequest {
    fn required_fields() -> &'static [&'static str] {
        &["refresh_token", "device_id"]
    }
}
