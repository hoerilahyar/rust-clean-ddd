use validator::ValidationError;

use super::dto::AuditLogQueryRequest;

pub fn validate_status(status: &str) -> Result<(), ValidationError> {
    match status {
        "success" | "failed" => Ok(()),
        _ => Err(ValidationError::new("invalid_status")),
    }
}

pub fn validate_date_range(query: &AuditLogQueryRequest) -> Result<(), ValidationError> {
    if let (Some(from), Some(to)) = (query.date_from, query.date_to) {
        if from > to {
            return Err(ValidationError::new("date_from_after_date_to"));
        }
    }
    Ok(())
}
