use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct AssignUserRoleRequest {
    #[validate(length(min = 1))]
    pub role_ids: Vec<u64>,
}
