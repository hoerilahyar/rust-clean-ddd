use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct PermissionResponse {
    pub id: u64,

    pub code: String,

    pub name: String,

    pub resource: String,

    pub action: String,

    pub description: Option<String>,

    pub is_active: bool,

    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct PermissionListResponse {
    pub items: Vec<PermissionResponse>,

    pub page: u64,

    pub page_size: u64,

    pub total: u64,
}
