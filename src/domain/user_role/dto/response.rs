use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct UserRoleResponse {
    pub role_id: u64,

    pub code: String,

    pub name: String,
}
