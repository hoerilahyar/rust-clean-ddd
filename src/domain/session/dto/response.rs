use chrono::{DateTime, Utc};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct SessionResponse {
    pub id: u64,
    pub device_id: String,
    pub ip_address: Option<String>,
    pub created_at: DateTime<Utc>,
    pub expired_at: DateTime<Utc>,
    pub is_current: bool,
}
