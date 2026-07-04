use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use validator::Validate;

use crate::{
    bootstrap::state::AppState,
    common::{
        error::app_error::AppError, extractor::CurrentUser, response::api_response::ApiResponse,
    },
    domain::{
        permission::entity::PermissionCode,
        role_permission::dto::{AssignRolePermissionRequest, RolePermissionResponse},
    },
};

#[utoipa::path(
    post,
    path = "/api/v1/roles/{role_id}/permissions",
    tag = "Role Permission",
    params(
        ("role_id" = u64, Path, description = "Role ID")
    ),
    request_body = AssignRolePermissionRequest,
    responses(
        (
            status = 200,
            description = "Permissions assigned successfully"
        ),
        (
            status = 400,
            description = "Validation error"
        ),
        (
            status = 401,
            description = "Unauthorized"
        ),
        (
            status = 404,
            description = "Role not found"
        )
    )
)]
pub async fn assign(
    current_user: CurrentUser,
    State(state): State<AppState>,
    Path(role_id): Path<u64>,
    Json(request): Json<AssignRolePermissionRequest>,
) -> Result<(StatusCode, Json<ApiResponse<()>>), AppError> {
    current_user.require(PermissionCode::RolePermissionAssign)?;

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

#[utoipa::path(
    delete,
    path = "/api/v1/roles/{role_id}/permissions/{permission_id}",
    tag = "Role Permission",
    params(
        ("role_id" = u64, Path, description = "Role ID"),
        ("permission_id" = u64, Path, description = "Permission ID")
    ),
    responses(
        (
            status = 200,
            description = "Permission revoked successfully"
        ),
        (
            status = 401,
            description = "Unauthorized"
        ),
        (
            status = 404,
            description = "Role or Permission not found"
        )
    )
)]
pub async fn revoke(
    current_user: CurrentUser,
    State(state): State<AppState>,
    Path((role_id, permission_id)): Path<(u64, u64)>,
) -> Result<(StatusCode, Json<ApiResponse<()>>), AppError> {
    current_user.require(PermissionCode::RolePermissionRevoke)?;

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

#[utoipa::path(
    get,
    path = "/api/v1/roles/{role_id}/permissions",
    tag = "Role Permission",
    params(
        ("role_id" = u64, Path, description = "Role ID")
    ),
    responses(
        (
            status = 200,
            description = "Permissions retrieved successfully",
            body = ApiResponse<Vec<RolePermissionResponse>>
        ),
        (
            status = 401,
            description = "Unauthorized"
        ),
        (
            status = 404,
            description = "Role not found"
        )
    )
)]
pub async fn list(
    current_user: CurrentUser,
    State(state): State<AppState>,
    Path(role_id): Path<u64>,
) -> Result<(StatusCode, Json<ApiResponse<Vec<RolePermissionResponse>>>), AppError> {
    current_user.require(PermissionCode::RolePermissionRead)?;

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
