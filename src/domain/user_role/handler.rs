use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use validator::Validate;

use crate::{
    bootstrap::state::AppState,
    common::{error::app_error::AppError, response::api_response::ApiResponse},
    domain::user_role::dto::{AssignUserRoleRequest, UserRoleResponse},
};

pub async fn assign(
    State(state): State<AppState>,
    Path(user_id): Path<u64>,
    Json(request): Json<AssignUserRoleRequest>,
) -> Result<(StatusCode, Json<ApiResponse<()>>), AppError> {
    request
        .validate()
        .map_err(|e| AppError::Validation(vec![("request".into(), e.to_string())]))?;

    state
        .services
        .user_role
        .assign(user_id, request)
        .await
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::<()>::message("Roles assigned successfully")),
    ))
}

pub async fn revoke(
    State(state): State<AppState>,
    Path((user_id, role_id)): Path<(u64, u64)>,
) -> Result<(StatusCode, Json<ApiResponse<()>>), AppError> {
    state
        .services
        .user_role
        .revoke(user_id, role_id)
        .await
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::<()>::message("Role revoked successfully")),
    ))
}

pub async fn list(
    State(state): State<AppState>,
    Path(user_id): Path<u64>,
) -> Result<(StatusCode, Json<ApiResponse<Vec<UserRoleResponse>>>), AppError> {
    let response = state
        .services
        .user_role
        .list(user_id)
        .await
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success(
            response,
            "Roles retrieved successfully",
        )),
    ))
}
