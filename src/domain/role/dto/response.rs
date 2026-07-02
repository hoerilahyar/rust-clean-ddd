use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct RoleResponse {
    pub id: u64,

    pub code: String,

    pub name: String,

    pub description: Option<String>,

    pub is_active: bool,

    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct RoleListResponse {
    pub items: Vec<RoleResponse>,

    pub page: u64,

    pub page_size: u64,

    pub total: u64,
}
