use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct AssignUserRoleRequest {
    #[validate(length(min = 1))]
    pub role_ids: Vec<u64>,
}
