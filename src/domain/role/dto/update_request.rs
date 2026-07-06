use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

use crate::common::extractor::RequiredFields;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateRoleRequest {
    pub code: Option<String>,

    #[validate(length(min = 3, max = 100))]
    pub name: String,

    #[validate(length(max = 255))]
    pub description: Option<String>,

    pub is_active: Option<bool>,
}

impl RequiredFields for UpdateRoleRequest {
    fn required_fields() -> &'static [&'static str] {
        &["name"]
    }
}
