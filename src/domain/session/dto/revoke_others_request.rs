use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

use crate::common::extractor::RequiredFields;

#[derive(Debug, Clone, Deserialize, Validate, ToSchema)]
pub struct RevokeOtherSessionsRequest {
    #[validate(length(min = 1, message = "device_id is required"))]
    pub device_id: String,
}

impl RequiredFields for RevokeOtherSessionsRequest {
    fn required_fields() -> &'static [&'static str] {
        &["device_id"]
    }
}
