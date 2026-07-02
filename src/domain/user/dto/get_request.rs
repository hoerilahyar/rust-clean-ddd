use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GetUserRequest {
    pub id: u64,
}
