use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct MenuPermissionResponse {
    pub menu_id: u64,

    pub permission_id: u64,
}
