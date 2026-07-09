use axum::{
    extract::{Request, State},
    http::{HeaderMap, HeaderName, header::AUTHORIZATION},
    middleware::Next,
    response::Response,
};

use crate::{
    bootstrap::state::AppState, common::error::app_error::AppError,
    domain::authorization::entity::PermissionContext, infrastructure::security::AccessClaims,
};

static API_KEY_HEADER: HeaderName = HeaderName::from_static("x-api-key");

fn bearer_token(headers: &HeaderMap) -> Option<&str> {
    let value = headers.get(AUTHORIZATION)?.to_str().ok()?;
    value.strip_prefix("Bearer ")
}

fn api_key_header(headers: &HeaderMap) -> Option<&str> {
    headers.get(&API_KEY_HEADER)?.to_str().ok()
}

/// Accepts either a `Bearer <jwt>` in `Authorization`, or a raw API key in
/// `X-API-Key`. Both resolve to the same `PermissionContext` shape, so
/// downstream handlers (and `CurrentUser::require`) don't need to know
/// which one was used.
pub async fn authenticate(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let headers = request.headers();

    if let Some(token) = bearer_token(headers) {
        let claims: AccessClaims = state
            .infra
            .jwt
            .verify_access_token(token)
            .map_err(|_| AppError::Unauthorized("Invalid access token".into()))?;

        let context = state
            .services
            .authorization
            .permission_context(claims.sub)
            .await
            .map_err(|_| AppError::Unauthorized("Unauthorized".into()))?;

        request.extensions_mut().insert(context);

        return Ok(next.run(request).await);
    }

    if let Some(raw_key) = api_key_header(headers) {
        let key = state
            .services
            .api_key
            .verify(raw_key)
            .await
            .map_err(|_| AppError::Unauthorized("Invalid API key".into()))?
            .ok_or_else(|| AppError::Unauthorized("Invalid or expired API key".into()))?;

        // Best-effort; a failure to record usage shouldn't block the request.
        let usage_service = state.services.api_key.clone();
        let key_id = key.id;
        tokio::spawn(async move { usage_service.record_usage(key_id).await });

        let context = PermissionContext {
            // 0 is not a valid user id (auto_increment starts at 1), so this
            // safely marks the request as machine, not human, in audit logs
            // and anywhere `actor_id` is recorded.
            user_id: 0,
            username: format!("api_key:{}", key.name),
            fullname: key.name.clone(),
            roles: vec![],
            permissions: key.permissions.clone(),
            menus: vec![],
        };

        request.extensions_mut().insert(context);

        return Ok(next.run(request).await);
    }

    Err(AppError::Unauthorized(
        "Missing bearer token or API key".into(),
    ))
}
