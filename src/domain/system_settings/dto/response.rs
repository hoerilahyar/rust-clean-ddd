use chrono::{DateTime, Utc};
use serde::Serialize;
use utoipa::ToSchema;

// ===== Response DTOs =====

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct SystemSettingResponse {
    pub id: u64,
    pub setting_key: String,
    pub setting_value: Option<String>,
    pub data_type: String,
    pub description: Option<String>,
    pub is_public: bool,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct SystemSettingListResponse {
    pub items: Vec<SystemSettingResponse>,
}
