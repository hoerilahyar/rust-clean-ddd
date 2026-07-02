use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};

use crate::{
    bootstrap::state::AppState, common::error::app_error::AppError,
    infrastructure::security::AccessClaims,
};

pub async fn require(
    State(_state): State<AppState>,
    request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let claims = request
        .extensions()
        .get::<AccessClaims>()
        .ok_or_else(|| AppError::Unauthorized("Unauthorized".into()))?;

    // Pastikan user sudah terautentikasi
    if claims.sub == 0 {
        return Err(AppError::Unauthorized("Unauthorized".into()));
    }

    // ------------------------------------------------------------------
    // TODO:
    // state.services.permission
    //      .has_permission(claims.sub, permission)
    //      .await?;
    // ------------------------------------------------------------------

    Ok(next.run(request).await)
}
