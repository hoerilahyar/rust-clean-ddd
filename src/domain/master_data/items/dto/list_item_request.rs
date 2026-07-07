use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Deserialize, IntoParams, ToSchema)]
pub struct ListMasterDataItemRequest {
    pub parent_id: Option<u64>,

    pub only_root: Option<bool>,

    pub search: Option<String>,

    pub is_active: Option<bool>,

    pub page: Option<u64>,

    pub page_size: Option<u64>,

    pub sort_by: Option<String>,

    pub sort_type: Option<String>,
}

#[derive(Debug, Deserialize, IntoParams, ToSchema)]
pub struct MasterDataOptionsQuery {
    pub parent_id: Option<u64>,

    pub only_root: Option<bool>,
}
