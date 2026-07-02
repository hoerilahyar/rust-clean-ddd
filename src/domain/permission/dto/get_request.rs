use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GetPermissionRequest {
    pub id: u64,
}
