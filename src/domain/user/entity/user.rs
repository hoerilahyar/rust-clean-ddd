use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: u64,

    pub username: String,

    pub fullname: String,

    pub email: String,

    #[serde(skip_serializing)]
    pub password: String,

    pub is_active: bool,

    pub last_login_at: Option<DateTime<Utc>>,

    pub created_at: DateTime<Utc>,

    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserFilter {
    pub page: u64,
    pub page_size: u64,
    pub search: Option<String>,
    pub sort_by: String,
    pub sort_type: String,
}
