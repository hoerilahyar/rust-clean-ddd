use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Validate, IntoParams, ToSchema)]
pub struct ListApiKeyRequest {
    pub search: Option<String>,
    pub is_active: Option<bool>,
}
