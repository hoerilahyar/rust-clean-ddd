use crate::{
    common::{
        error::app_error::AppError,
        validator::validator::{validate_email, validate_required, validate_string_length},
    },
    domain::auth::dto::{LoginRequest, RefreshTokenRequest},
};

pub struct AuthValidator;

impl AuthValidator {
    pub fn validate_login(request: &LoginRequest) -> Result<(), AppError> {
        validate_required(&request.identity, "identity")?;

        validate_required(&request.password, "password")?;

        validate_string_length(&request.identity, "identity", 3, 100)?;

        validate_string_length(&request.password, "password", 6, 255)?;

        if request.identity.contains('@') {
            validate_email(&request.identity, "identity")?;
        }

        Ok(())
    }

    pub fn validate_refresh_token(request: &RefreshTokenRequest) -> Result<(), AppError> {
        validate_required(&request.refresh_token, "refresh_token")?;

        validate_required(&request.device_id, "device_id")?;

        validate_string_length(&request.refresh_token, "refresh_token", 32, 4096)?;

        validate_string_length(&request.device_id, "device_id", 8, 255)?;

        Ok(())
    }
}
