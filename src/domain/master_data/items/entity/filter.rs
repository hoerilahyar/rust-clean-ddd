#[derive(Debug, Clone)]
pub struct MasterDataItemFilter {
    pub group_id: u64,

    /// Filter isi berdasarkan parent tertentu (untuk cascading dropdown).
    pub parent_id: Option<u64>,

    /// Kalau true, hanya ambil item level teratas (parent_id IS NULL).
    pub only_root: bool,

    pub search: Option<String>,

    pub is_active: Option<bool>,

    pub page: u64,

    pub page_size: u64,

    pub sort_by: String,

    pub sort_type: String,
}
