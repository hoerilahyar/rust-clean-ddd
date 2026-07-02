use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ListPermissionRequest {
    pub page: Option<u64>,

    pub page_size: Option<u64>,

    pub search: Option<String>,

    pub resource: Option<String>,

    pub sort_by: Option<String>,

    pub sort_type: Option<String>,
}
