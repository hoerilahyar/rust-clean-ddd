use axum::{
    Json,
    extract::{ConnectInfo, Path, State},
    http::{HeaderMap, StatusCode},
};
use validator::Validate;

use crate::{
    bootstrap::state::AppState,
    common::{
        error::app_error::AppError, extractor::CurrentUser, response::api_response::ApiResponse,
    },
    domain::{
        permission::entity::PermissionCode,
        user_role::dto::{AssignUserRoleRequest, UserRoleResponse},
    },
};

#[utoipa::path(
    post,
    path = "/api/v1/users/{user_id}/roles",
    tag = "User Role",
    params(
        ("user_id" = u64, Path, description = "User ID")
    ),
    request_body = AssignUserRoleRequest,
    responses(
        (
            status = 200,
            description = "Roles assigned successfully"
        ),
        (
            status = 400,
            description = "Validation error"
        ),
        (
            status = 404,
            description = "User not found"
        )
    )
)]
pub async fn assign(
    current_user: CurrentUser,
    State(state): State<AppState>,
    Path(user_id): Path<u64>,
    ConnectInfo(addr): ConnectInfo<std::net::SocketAddr>,
    headers: HeaderMap,
    Json(request): Json<AssignUserRoleRequest>,
) -> Result<(StatusCode, Json<ApiResponse<()>>), AppError> {
    current_user.require(PermissionCode::UserRoleAssign)?;

    request
        .validate()
        .map_err(|e| AppError::Validation(vec![("request".into(), e.to_string())]))?;

    let ip_address = Some(addr.ip().to_string());
    let user_agent = headers
        .get(axum::http::header::USER_AGENT)
        .and_then(|v| v.to_str().ok())
        .map(str::to_owned);

    state
        .services
        .user_role
        .assign(
            user_id,
            request,
            Some(current_user.user_id()),
            ip_address,
            user_agent,
        )
        .await
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::<()>::message("Roles assigned successfully")),
    ))
}

#[utoipa::path(
    delete,
    path = "/api/v1/users/{user_id}/roles/{role_id}",
    tag = "User Role",
    params(
        ("user_id" = u64, Path, description = "User ID"),
        ("role_id" = u64, Path, description = "Role ID")
    ),
    responses(
        (
            status = 200,
            description = "Role revoked successfully"
        ),
        (
            status = 404,
            description = "User or Role not found"
        )
    )
)]
pub async fn revoke(
    current_user: CurrentUser,
    State(state): State<AppState>,
    Path((user_id, role_id)): Path<(u64, u64)>,
    ConnectInfo(addr): ConnectInfo<std::net::SocketAddr>,
    headers: HeaderMap,
) -> Result<(StatusCode, Json<ApiResponse<()>>), AppError> {
    current_user.require(PermissionCode::UserRoleRevoke)?;

    let ip_address = Some(addr.ip().to_string());
    let user_agent = headers
        .get(axum::http::header::USER_AGENT)
        .and_then(|v| v.to_str().ok())
        .map(str::to_owned);

    state
        .services
        .user_role
        .revoke(
            user_id,
            role_id,
            Some(current_user.user_id()),
            ip_address,
            user_agent,
        )
        .await
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::<()>::message("Role revoked successfully")),
    ))
}

#[utoipa::path(
    get,
    path = "/api/v1/users/{user_id}/roles",
    tag = "User Role",
    params(
        ("user_id" = u64, Path, description = "User ID")
    ),
    responses(
        (
            status = 200,
            description = "Roles retrieved successfully",
            body = ApiResponse<Vec<UserRoleResponse>>
        ),
        (
            status = 404,
            description = "User not found"
        )
    )
)]
pub async fn list(
    current_user: CurrentUser,
    State(state): State<AppState>,
    Path(user_id): Path<u64>,
) -> Result<(StatusCode, Json<ApiResponse<Vec<UserRoleResponse>>>), AppError> {
    current_user.require(PermissionCode::UserRoleRead)?;

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
