use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserRoleResponse {
    pub role_id: u64,

    pub code: String,

    pub name: String,
}
