use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

use crate::common::extractor::RequiredFields;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct AssignUserRoleRequest {
    #[validate(length(min = 1))]
    pub role_ids: Vec<u64>,
}

impl RequiredFields for AssignUserRoleRequest {
    fn required_fields() -> &'static [&'static str] {
        &["role_ids"]
    }
}
