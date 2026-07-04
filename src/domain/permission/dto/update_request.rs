use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct UpdatePermissionRequest {
    #[validate(length(min = 3, max = 100))]
    pub name: String,

    #[validate(length(max = 255))]
    pub description: Option<String>,

    pub is_active: bool,
}
