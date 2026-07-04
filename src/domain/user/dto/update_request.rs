use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdateUserRequest {
    #[validate(length(min = 3, max = 100))]
    pub fullname: String,

    #[validate(email)]
    pub email: String,

    pub is_active: bool,
}
