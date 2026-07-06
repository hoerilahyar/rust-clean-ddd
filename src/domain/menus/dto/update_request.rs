use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

use crate::common::extractor::RequiredFields;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateMenuRequest {
    pub parent_id: Option<u64>,

    #[validate(length(min = 3, max = 50))]
    pub name: Option<String>,

    #[validate(length(min = 3, max = 100))]
    pub icon: Option<String>,

    #[validate(length(min = 3, max = 50))]
    pub path: Option<String>,

    pub sort_order: Option<i32>,

    pub is_active: Option<bool>,
}

impl RequiredFields for UpdateMenuRequest {
    fn required_fields() -> &'static [&'static str] {
        &[]
    }
}
