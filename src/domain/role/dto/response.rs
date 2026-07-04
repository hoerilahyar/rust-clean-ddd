use chrono::{DateTime, Utc};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct RoleResponse {
    pub id: u64,

    pub code: String,

    pub name: String,

    pub description: Option<String>,

    pub is_active: bool,

    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct RoleListResponse {
    pub items: Vec<RoleResponse>,

    pub page: u64,

    pub page_size: u64,

    pub total: u64,
}
