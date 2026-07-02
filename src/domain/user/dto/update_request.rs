use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateUserRequest {
    #[validate(length(min = 3, max = 100))]
    pub fullname: String,

    #[validate(email)]
    pub email: String,

    pub is_active: bool,
}
