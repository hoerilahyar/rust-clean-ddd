use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

use crate::common::extractor::RequiredFields;

#[derive(Debug, Clone, Deserialize, Validate, ToSchema)]
pub struct SetUserSettingActiveRequest {
    pub is_active: bool,
}

impl RequiredFields for SetUserSettingActiveRequest {
    fn required_fields() -> &'static [&'static str] {
        &["is_active"]
    }
}
