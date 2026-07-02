use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use validator::Validate;

use crate::{
    bootstrap::state::AppState,
    common::{error::app_error::AppError, response::api_response::ApiResponse},
    domain::role_permission::dto::{AssignRolePermissionRequest, RolePermissionResponse},
};

pub async fn assign(
    State(state): State<AppState>,
    Path(role_id): Path<u64>,
    Json(request): Json<AssignRolePermissionRequest>,
) -> Result<(StatusCode, Json<ApiResponse<()>>), AppError> {
    request
        .validate()
        .map_err(|e| AppError::Validation(vec![("request".into(), e.to_string())]))?;

    state
        .services
        .role_permission
        .assign(role_id, request)
        .await
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::<()>::message(
            "Permissions assigned successfully",
        )),
    ))
}

pub async fn revoke(
    State(state): State<AppState>,
    Path((role_id, permission_id)): Path<(u64, u64)>,
) -> Result<(StatusCode, Json<ApiResponse<()>>), AppError> {
    state
        .services
        .role_permission
        .revoke(role_id, permission_id)
        .await
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::<()>::message(
            "Permission revoked successfully",
        )),
    ))
}

pub async fn list(
    State(state): State<AppState>,
    Path(role_id): Path<u64>,
) -> Result<(StatusCode, Json<ApiResponse<Vec<RolePermissionResponse>>>), AppError> {
    let response = state
        .services
        .role_permission
        .list(role_id)
        .await
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success(
            response,
            "Permissions retrieved successfully",
        )),
    ))
}
