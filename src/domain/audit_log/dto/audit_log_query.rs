use chrono::{DateTime, Utc};
use serde::Deserialize;
use utoipa::IntoParams;
use validator::Validate;

use crate::domain::audit_log::validator::{validate_date_range, validate_status};

fn default_page() -> u32 {
    1
}
fn default_page_size() -> u32 {
    20
}

#[derive(Debug, Deserialize, Validate, IntoParams)]
#[validate(schema(function = "validate_date_range"))]
#[into_params(parameter_in = Query)]
pub struct AuditLogQueryRequest {
    pub actor_id: Option<u64>,
    pub action: Option<String>,
    pub entity_type: Option<String>,

    #[validate(custom(function = "validate_status"))]
    pub status: Option<String>,

    pub date_from: Option<DateTime<Utc>>,
    pub date_to: Option<DateTime<Utc>>,

    #[validate(range(min = 1, message = "page must be >= 1"))]
    #[serde(default = "default_page")]
    pub page: u32,

    #[validate(range(min = 1, max = 100, message = "page_size must be between 1 and 100"))]
    #[serde(default = "default_page_size")]
    pub page_size: u32,
}
