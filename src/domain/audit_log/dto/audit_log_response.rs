use chrono::{DateTime, Utc};
use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct AuditLogResponse {
    pub id: u64,
    pub actor_id: Option<u64>,
    pub actor_email: Option<String>,
    pub action: String,
    pub entity_type: Option<String>,
    pub entity_id: Option<String>,
    pub status: String,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub metadata: Option<Value>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct AuditLogListResponse {
    pub data: Vec<AuditLogResponse>,
    pub total: i64,
    pub page: u32,
    pub page_size: u32,
}
