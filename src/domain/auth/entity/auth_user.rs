use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct AuthUser {
    pub id: u64,
    pub username: String,
    pub email: String,
    pub password: String,
    pub fullname: String,
    pub phone: Option<String>,
    pub avatar: Option<String>,
    pub is_active: bool,
    pub last_login_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}
