use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GetRoleRequest {
    pub id: u64,
}
