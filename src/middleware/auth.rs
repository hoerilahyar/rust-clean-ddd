use axum::{
    extract::{Request, State},
    http::{HeaderMap, header::AUTHORIZATION},
    middleware::Next,
    response::Response,
};

use crate::{
    bootstrap::state::AppState, common::error::app_error::AppError,
    infrastructure::security::AccessClaims,
};

fn bearer_token(headers: &HeaderMap) -> Option<&str> {
    let value = headers.get(AUTHORIZATION)?.to_str().ok()?;
    value.strip_prefix("Bearer ")
}

pub async fn authenticate(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let token = bearer_token(request.headers())
        .ok_or_else(|| AppError::Unauthorized("Missing bearer token".into()))?;

    let claims: AccessClaims = state
        .infra
        .jwt
        .verify_access_token(token)
        .map_err(|_| AppError::Unauthorized("Invalid access token".into()))?;

    let user_id = claims.sub;

    let context = state
        .services
        .authorization
        .permission_context(user_id)
        .await
        .map_err(|_| AppError::Unauthorized("Unauthorized".into()))?;

    request.extensions_mut().insert(context);

    Ok(next.run(request).await)
}
