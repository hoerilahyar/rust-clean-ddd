use crate::bootstrap::state::AppState;
use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};

pub async fn validate_api_key(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let api_key = request
        .headers()
        .get("x-api-key")
        .and_then(|v| v.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    match state.services.api_key.verify(api_key).await {
        Ok(Some(api_key_entity)) => {
            tracing::debug!("API key validated");

            request.extensions_mut().insert(api_key_entity.clone());

            state.services.api_key.record_usage(api_key_entity.id).await;

            Ok(next.run(request).await)
        }
        Ok(None) => {
            tracing::warn!("Invalid API key");
            Err(StatusCode::UNAUTHORIZED)
        }
        Err(err) => {
            tracing::error!("Verify API key failed: {:?}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
