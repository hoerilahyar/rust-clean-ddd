use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct MasterDataGroupResponse {
    pub id: u64,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub is_hierarchical: bool,
    pub is_active: bool,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct MasterDataGroupListResponse {
    pub items: Vec<MasterDataGroupResponse>,

    pub page: u64,
    pub page_size: u64,
    pub total: u64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct MasterDataOptionResponse {
    pub id: u64,
    pub code: String,
    pub name: String,
    pub parent_id: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct MasterDataOptionListResponse {
    pub items: Vec<MasterDataOptionResponse>,
}
