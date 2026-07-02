use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct AssignRolePermissionRequest {
    #[validate(length(min = 1))]
    pub permission_ids: Vec<u64>,
}
