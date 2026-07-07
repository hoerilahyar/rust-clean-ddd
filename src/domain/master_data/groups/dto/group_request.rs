use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

use crate::common::extractor::RequiredFields;

#[derive(Debug, Clone, Deserialize, Validate, ToSchema)]
pub struct CreateMasterDataGroupRequest {
    #[validate(length(
        min = 1,
        max = 50,
        message = "Code must be between 1 and 50 characters"
    ))]
    pub code: String,

    #[validate(length(
        min = 1,
        max = 150,
        message = "Name must be between 1 and 150 characters"
    ))]
    pub name: String,

    #[validate(length(max = 255, message = "Description must not exceed 255 characters"))]
    pub description: Option<String>,

    pub is_hierarchical: Option<bool>,
}

impl RequiredFields for CreateMasterDataGroupRequest {
    fn required_fields() -> &'static [&'static str] {
        &["code", "name"]
    }
}

#[derive(Debug, Clone, Deserialize, Validate, ToSchema)]
pub struct UpdateMasterDataGroupRequest {
    #[validate(length(
        min = 1,
        max = 150,
        message = "Name must be between 1 and 150 characters"
    ))]
    pub name: Option<String>,

    #[validate(length(max = 255, message = "Description must not exceed 255 characters"))]
    pub description: Option<String>,

    pub is_hierarchical: Option<bool>,

    pub is_active: Option<bool>,
}

impl RequiredFields for UpdateMasterDataGroupRequest {}
