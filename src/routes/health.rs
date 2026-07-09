use axum::{Json, Router, extract::State, http::StatusCode, routing::get};
use chrono::Utc;

use crate::bootstrap::state::AppState;

#[derive(serde::Serialize)]
pub struct HealthResponse {
    status: String,
    timestamp: String,
    database: String,
}

pub async fn health_check(
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<HealthResponse>), (StatusCode, String)> {
    // Check database connection
    match state.infra.db.acquire().await {
        Ok(_) => {
            let response = HealthResponse {
                status: "healthy".to_string(),
                timestamp: Utc::now().to_rfc3339(),
                database: "ok".to_string(),
            };
            Ok((StatusCode::OK, Json(response)))
        }
        Err(e) => {
            tracing::error!("Database health check failed: {}", e);
            Err((
                StatusCode::SERVICE_UNAVAILABLE,
                format!("Database connection failed: {}", e),
            ))
        }
    }
}

pub fn router() -> Router<AppState> {
    Router::new().route("/", get(health_check))
}
