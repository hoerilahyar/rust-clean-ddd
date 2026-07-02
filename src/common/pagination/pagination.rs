use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Pagination {
    pub page: u64,

    pub page_size: u64,

    pub total_data: u64,

    pub total_page: u64,
}
