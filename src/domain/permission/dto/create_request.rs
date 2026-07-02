use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct CreatePermissionRequest {
    #[validate(length(min = 3, max = 50))]
    pub code: String,

    #[validate(length(min = 3, max = 100))]
    pub name: String,

    #[validate(length(min = 2, max = 50))]
    pub resource: String,

    #[validate(length(min = 2, max = 50))]
    pub action: String,

    #[validate(length(max = 255))]
    pub description: Option<String>,

    pub is_active: bool,
}
