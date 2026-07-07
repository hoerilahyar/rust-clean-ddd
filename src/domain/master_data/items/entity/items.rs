use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::prelude::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MasterDataItem {
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
