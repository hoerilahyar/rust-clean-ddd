use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrentUser {
    pub id: u64,
    pub username: String,
    pub fullname: String,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
}
