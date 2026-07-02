#[derive(Debug, Clone)]
pub struct PermissionFilter {
    pub page: u64,

    pub page_size: u64,

    pub search: Option<String>,

    pub resource: Option<String>,

    pub sort_by: String,

    pub sort_type: String,
}
