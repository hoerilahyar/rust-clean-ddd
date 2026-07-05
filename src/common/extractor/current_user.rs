use axum::{extract::FromRequestParts, http::request::Parts};

use crate::{
    common::error::app_error::AppError,
    domain::{authorization::entity::PermissionContext, permission::entity::PermissionCode},
};

#[derive(Debug, Clone)]
pub struct CurrentUser {
    pub context: PermissionContext,
}

impl CurrentUser {
    pub fn user_id(&self) -> u64 {
        self.context.user_id
    }

    pub fn require(&self, permission: PermissionCode) -> Result<(), AppError> {
        if self
            .context
            .permissions
            .iter()
            .any(|p| p == permission.as_str())
        {
            return Ok(());
        }

        Err(AppError::Forbidden("Permission denied".to_string()))
    }
}

impl<S> FromRequestParts<S> for CurrentUser
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let context = parts
            .extensions
            .get::<PermissionContext>()
            .cloned()
            .ok_or_else(|| AppError::Unauthorized("Unauthorized".into()))?;

        Ok(CurrentUser { context })
    }
}
