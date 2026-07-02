use regex::Regex;

use crate::common::error::app_error::AppError;

pub fn validate_required(value: &str, field: &str) -> Result<(), AppError> {
    if value.trim().is_empty() {
        return Err(AppError::BadRequest(format!("{} is required", field)));
    }

    Ok(())
}

pub fn validate_string_length(
    value: &str,
    field: &str,
    min: usize,
    max: usize,
) -> Result<(), AppError> {
    let len = value.trim().chars().count();

    if len < min {
        return Err(AppError::BadRequest(format!(
            "{} must be at least {} characters",
            field, min
        )));
    }

    if len > max {
        return Err(AppError::BadRequest(format!(
            "{} must be at most {} characters",
            field, max
        )));
    }

    Ok(())
}

pub fn validate_email(value: &str, field: &str) -> Result<(), AppError> {
    let regex = Regex::new(r"^[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}$")
        .map_err(|e| AppError::Internal(e.to_string()))?;

    if !regex.is_match(value) {
        return Err(AppError::BadRequest(format!("{} is invalid", field)));
    }

    Ok(())
}
