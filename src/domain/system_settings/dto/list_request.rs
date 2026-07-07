use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Validate, IntoParams, ToSchema)]
pub struct ListSystemSettingRequest {
    pub search: Option<String>,
    pub is_active: Option<bool>,
    pub is_public: Option<bool>,

    pub sort_by: Option<String>,
    pub sort_type: Option<String>,
}
