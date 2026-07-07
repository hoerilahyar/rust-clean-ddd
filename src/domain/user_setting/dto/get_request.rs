use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct GetUserSettingRequest {
    #[validate(length(min = 1, message = "Setting key is required"))]
    pub setting_key: String,
}
