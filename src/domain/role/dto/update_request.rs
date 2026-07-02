use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateRoleRequest {
    #[validate(length(min = 3, max = 100))]
    pub name: String,

    #[validate(length(max = 255))]
    pub description: Option<String>,

    pub is_active: bool,
}
