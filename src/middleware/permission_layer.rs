use axum::{
    extract::Request,
    middleware::{self, FromFnLayer, Next},
    response::Response,
};

use crate::{
    bootstrap::state::AppState,
    common::error::app_error::AppError,
    domain::{authorization::entity::PermissionContext, permission::entity::PermissionCode},
};

pub fn layer(state: AppState, permission: PermissionCode) -> FromFnLayer<impl Clone, AppState, ()> {
    middleware::from_fn_with_state(state, move |request, next| check(permission, request, next))
}

async fn check(
    permission: PermissionCode,
    request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let context = request
        .extensions()
        .get::<PermissionContext>()
        .ok_or_else(|| AppError::Unauthorized("Unauthorized".into()))?;

    if context.permissions.iter().any(|p| p == permission.as_str()) {
        return Ok(next.run(request).await);
    }

    Err(AppError::Forbidden("Permission denied".into()))
}
