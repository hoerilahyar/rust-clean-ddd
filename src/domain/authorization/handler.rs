use axum::{Json, extract::State, http::StatusCode};

use crate::{
    bootstrap::state::AppState,
    common::{
        error::app_error::AppError, extractor::CurrentUser, response::api_response::ApiResponse,
    },
    domain::{
        authorization::dto::CurrentUser as DtoCurrentUser, permission::entity::PermissionCode,
    },
};

#[utoipa::path(
    get,
    path = "/api/v1/iam/authorize/me",
    tag = "Authorization",
    responses(
        (
            status = 200,
            description = "Current user retrieved successfully",
            body = ApiResponse<DtoCurrentUser>
        ),
        (
            status = 401,
            description = "Unauthorized"
        )
    )
)]
pub async fn me(
    current_user: CurrentUser,
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<ApiResponse<DtoCurrentUser>>), AppError> {
    current_user.require(PermissionCode::AuthorizeMe)?;

    let user = state
        .services
        .authorization
        .current_user(current_user.user_id())
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
