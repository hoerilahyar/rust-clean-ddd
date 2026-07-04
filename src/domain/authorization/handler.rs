use axum::{Json, extract::State, http::StatusCode};

use crate::{
    bootstrap::state::AppState,
    common::{error::app_error::AppError, response::api_response::ApiResponse},
    domain::authorization::dto::CurrentUser,
};

#[utoipa::path(
    get,
    path = "/api/v1/authorize/me",
    tag = "Authorization",
    responses(
        (
            status = 200,
            description = "Current user retrieved successfully",
            body = ApiResponse<CurrentUser>
        ),
        (
            status = 401,
            description = "Unauthorized"
        )
    )
)]
pub async fn me(
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<ApiResponse<CurrentUser>>), AppError> {
    let user = state
        .services
        .authorization
        .current_user(1)
        .await
        .map_err(|e| AppError::Unauthorized(e.to_string()))?;

    Ok((StatusCode::OK, Json(ApiResponse::success(user, "Success"))))
}

// pub async fn me(
//     current: CurrentUser,
// ) -> Result<(StatusCode, Json<ApiResponse<CurrentUser>>), AppError> {
//     Ok((
//         StatusCode::OK,
//         Json(ApiResponse::success(current, "Success")),
//     ))
// }
