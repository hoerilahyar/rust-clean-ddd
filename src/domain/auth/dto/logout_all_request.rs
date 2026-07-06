use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

use crate::common::extractor::RequiredFields;

#[derive(Debug, Deserialize, Validate, Serialize, ToSchema)]
pub struct LogoutAllRequest {
    pub user_id: u64,
}

impl RequiredFields for LogoutAllRequest {
    fn required_fields() -> &'static [&'static str] {
        &["user_id"]
    }
}
