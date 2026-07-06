use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

use crate::common::extractor::RequiredFields;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct AssignRolePermissionRequest {
    #[validate(length(min = 1))]
    pub permission_ids: Vec<u64>,
}

impl RequiredFields for AssignRolePermissionRequest {
    fn required_fields() -> &'static [&'static str] {
        &["permission_ids"]
    }
}
