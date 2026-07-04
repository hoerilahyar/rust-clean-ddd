use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct CurrentUser {
    pub id: u64,
    pub username: String,
    pub fullname: String,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
}
