use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct MasterDataItemResponse {
    pub id: u64,
    pub group_id: u64,
    pub parent_id: Option<u64>,
    pub code: String,
    pub name: String,
    pub metadata: Option<Value>,
    pub sort_order: i32,
    pub is_active: bool,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct MasterDataItemListResponse {
    pub items: Vec<MasterDataItemResponse>,

    pub page: u64,
    pub page_size: u64,
    pub total: u64,
}
