use axum::{extract::FromRequestParts, http::request::Parts};

use crate::{common::error::app_error::AppError, domain::authorization::entity::PermissionContext};

#[derive(Debug, Clone)]
pub struct CurrentUser(pub PermissionContext);

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

        Ok(CurrentUser(context))
    }
}
