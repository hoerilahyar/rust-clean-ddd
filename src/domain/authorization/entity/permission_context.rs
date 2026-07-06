use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone)]
pub struct PermissionContext {
    pub user_id: u64,
    pub username: String,
    pub fullname: String,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
    pub menus: Vec<MenuContext>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MenuContext {
    pub id: u64,
    pub parent_id: Option<u64>,
    pub name: String,
    pub path: Option<String>,
    pub icon: Option<String>,
    pub sort_order: i32,
}
