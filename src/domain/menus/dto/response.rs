use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct MenuResponse {
    pub id: u64,
    pub parent_id: Option<u64>,
    pub name: String,
    pub icon: Option<String>,
    pub path: String,
    pub sort_order: i32,
    pub is_active: bool,

    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct MenuListResponse {
    pub items: Vec<MenuResponse>,

    pub page: u64,

    pub page_size: u64,

    pub total: u64,
}
