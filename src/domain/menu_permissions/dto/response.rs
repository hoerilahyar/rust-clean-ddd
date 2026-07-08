use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct MenuPermissionResponse {
    pub menu_id: u64,

    pub permission_id: u64,
}
