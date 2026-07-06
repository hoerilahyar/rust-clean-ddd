use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

use crate::common::extractor::RequiredFields;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateMenuRequest {
    pub parent_id: Option<u64>,

    #[validate(length(min = 3, max = 50))]
    pub name: String,

    #[validate(length(min = 3, max = 100))]
    pub icon: Option<String>,

    #[validate(length(min = 3, max = 50))]
    pub path: String,

    pub sort_order: Option<i32>,

    pub is_active: Option<bool>,
}

impl RequiredFields for CreateMenuRequest {
    fn required_fields() -> &'static [&'static str] {
        &["name", "path"]
    }
}
