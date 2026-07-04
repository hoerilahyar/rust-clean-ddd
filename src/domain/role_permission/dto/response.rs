use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct RolePermissionResponse {
    pub role_id: u64,

    pub permission_id: u64,
}
