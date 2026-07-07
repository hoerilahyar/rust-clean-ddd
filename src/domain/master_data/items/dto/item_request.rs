use serde::Deserialize;
use serde_json::Value;
use utoipa::ToSchema;
use validator::Validate;

use crate::common::extractor::RequiredFields;

#[derive(Debug, Clone, Deserialize, Validate, ToSchema)]
pub struct CreateMasterDataItemRequest {
    pub parent_id: Option<u64>,

    #[validate(length(
        min = 1,
        max = 100,
        message = "Code must be between 1 and 100 characters"
    ))]
    pub code: String,

    #[validate(length(
        min = 1,
        max = 255,
        message = "Name must be between 1 and 255 characters"
    ))]
    pub name: String,

    pub metadata: Option<Value>,

    pub sort_order: Option<i32>,

    pub is_active: Option<bool>,
}

impl RequiredFields for CreateMasterDataItemRequest {
    fn required_fields() -> &'static [&'static str] {
        &["code", "name"]
    }
}

#[derive(Debug, Clone, Deserialize, Validate, ToSchema)]
pub struct UpdateMasterDataItemRequest {
    pub parent_id: Option<u64>,

    #[validate(length(
        min = 1,
        max = 255,
        message = "Name must be between 1 and 255 characters"
    ))]
    pub name: Option<String>,

    pub metadata: Option<Value>,

    pub sort_order: Option<i32>,

    pub is_active: Option<bool>,
}

impl RequiredFields for UpdateMasterDataItemRequest {}
