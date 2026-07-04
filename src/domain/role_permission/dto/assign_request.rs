use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct AssignRolePermissionRequest {
    #[validate(length(min = 1))]
    pub permission_ids: Vec<u64>,
}
