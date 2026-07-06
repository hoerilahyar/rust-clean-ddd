use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Menu {
    pub id: u64,
    pub parent_id: Option<u64>,
    pub name: String,
    pub icon: Option<String>,
    pub path: String,
    pub sort_order: i32,
    pub is_active: bool,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
