use chrono::{DateTime, Utc};
use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

use crate::common::extractor::RequiredFields;

#[derive(Debug, Clone, Deserialize, Validate, ToSchema)]
pub struct UpdateApiKeyRequest {
    #[validate(length(
        min = 1,
        max = 150,
        message = "Name must be between 1 and 150 characters"
    ))]
    pub name: Option<String>,

    #[validate(length(min = 1, message = "At least one permission is required"))]
    pub permissions: Option<Vec<String>>,

    pub expires_at: Option<DateTime<Utc>>,
}

impl RequiredFields for UpdateApiKeyRequest {
    fn required_fields() -> &'static [&'static str] {
        &[]
    }
}

#[derive(Debug, Clone, Deserialize, Validate, ToSchema)]
pub struct SetApiKeyActiveRequest {
    pub is_active: bool,
}

impl RequiredFields for SetApiKeyActiveRequest {
    fn required_fields() -> &'static [&'static str] {
        &["is_active"]
    }
}
