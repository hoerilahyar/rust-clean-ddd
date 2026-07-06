use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

use crate::common::extractor::RequiredFields;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct CreateUserRequest {
    #[validate(length(min = 3, max = 50))]
    pub username: String,

    #[validate(length(min = 3, max = 100))]
    pub fullname: String,

    #[validate(email)]
    pub email: String,

    #[validate(length(min = 8))]
    pub password: String,

    pub is_active: Option<bool>,
}

impl RequiredFields for CreateUserRequest {
    fn required_fields() -> &'static [&'static str] {
        &["username", "fullname", "email", "password"]
    }
}
