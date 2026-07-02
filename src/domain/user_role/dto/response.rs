use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct UserRoleResponse {
    pub role_id: u64,

    pub code: String,

    pub name: String,
}
