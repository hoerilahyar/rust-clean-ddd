#[derive(Debug, Clone)]
pub struct RoleFilter {
    pub page: u64,

    pub page_size: u64,

    pub search: Option<String>,

    pub sort_by: String,

    pub sort_type: String,
}
