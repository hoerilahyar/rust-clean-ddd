use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Deserialize, IntoParams, ToSchema)]
pub struct ListMasterDataGroupRequest {
    pub page: Option<u64>,

    pub page_size: Option<u64>,

    pub search: Option<String>,

    pub sort_by: Option<String>,

    pub sort_type: Option<String>,
}
