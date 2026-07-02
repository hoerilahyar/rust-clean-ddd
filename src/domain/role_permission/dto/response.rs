use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct RolePermissionResponse {
    pub role_id: u64,

    pub permission_id: u64,
}
