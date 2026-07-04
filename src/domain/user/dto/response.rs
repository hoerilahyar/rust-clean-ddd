use chrono::{DateTime, Utc};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct UserResponse {
    pub id: u64,

    pub username: String,

    pub fullname: String,

    pub email: String,

    pub is_active: bool,

    pub last_login_at: Option<DateTime<Utc>>,

    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UserListResponse {
    pub items: Vec<UserResponse>,

    pub page: u64,

    pub page_size: u64,

    pub total: u64,
}
