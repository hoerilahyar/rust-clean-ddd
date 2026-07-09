use chrono::{DateTime, Utc};
use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

use crate::common::extractor::RequiredFields;

#[derive(Debug, Clone, Deserialize, Validate, ToSchema)]
pub struct CreateApiKeyRequest {
    #[validate(length(
        min = 1,
        max = 150,
        message = "Name must be between 1 and 150 characters"
    ))]
    pub name: String,

    #[validate(length(min = 1, message = "At least one permission is required"))]
    pub permissions: Vec<String>,

    pub expires_at: Option<DateTime<Utc>>,
}

impl RequiredFields for CreateApiKeyRequest {
    fn required_fields() -> &'static [&'static str] {
        &["name", "permissions"]
    }
}
