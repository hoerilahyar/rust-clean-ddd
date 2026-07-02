use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct RefreshToken {
    pub id: u64,
    pub user_id: u64,
    pub device_id: String,
    pub ip_address: Option<String>,
    pub token: String,
    pub expired_at: DateTime<Utc>,
    pub revoked_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}
