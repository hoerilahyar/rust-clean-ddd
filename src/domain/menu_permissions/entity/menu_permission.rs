use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MenuPermission {
    pub menu_id: u64,

    pub permission_id: u64,
}
