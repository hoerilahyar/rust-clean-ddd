use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MenuContext {
    pub id: u64,
    pub parent_id: Option<u64>,
    pub name: String,
    pub path: Option<String>,
    pub icon: Option<String>,
    pub sort_order: i32,
}
