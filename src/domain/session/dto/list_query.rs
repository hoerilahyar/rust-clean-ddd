use serde::Deserialize;
use utoipa::IntoParams;

#[derive(Debug, Clone, Deserialize, IntoParams)]
pub struct ListSessionQuery {
    /// Optional device_id of the caller's own session, used to flag `is_current` in the response.
    pub device_id: Option<String>,
}
