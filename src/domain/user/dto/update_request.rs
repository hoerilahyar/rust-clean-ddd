use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

use crate::common::extractor::RequiredFields;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateUserRequest {
    pub fullname: Option<String>,

    pub email: Option<String>,

    pub password: Option<String>,

    pub is_active: Option<bool>,
}

impl RequiredFields for UpdateUserRequest {
    fn required_fields() -> &'static [&'static str] {
        &[]
    }
}
