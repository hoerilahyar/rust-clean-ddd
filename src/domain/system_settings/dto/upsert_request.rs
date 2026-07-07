use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

use crate::common::extractor::RequiredFields;

// ===== Request DTOs =====

#[derive(Debug, Clone, Deserialize, Validate, ToSchema)]
pub struct UpsertSystemSettingRequest {
    #[validate(length(
        min = 1,
        max = 150,
        message = "Setting key must be between 1 and 150 characters"
    ))]
    pub setting_key: String,

    pub setting_value: Option<String>,

    #[validate(length(min = 1, message = "Data type is required"))]
    pub data_type: String, // "string" | "number" | "boolean" | "json"

    #[validate(length(max = 255, message = "Description must not exceed 255 characters"))]
    pub description: Option<String>,

    pub is_public: Option<bool>,
}

impl RequiredFields for UpsertSystemSettingRequest {
    fn required_fields() -> &'static [&'static str] {
        &["setting_key", "data_type"]
    }
}
