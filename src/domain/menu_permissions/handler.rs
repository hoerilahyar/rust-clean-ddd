use axum::{
    Json,
    extract::{ConnectInfo, Path, State},
    http::{HeaderMap, StatusCode},
};
use validator::Validate;

use crate::{
    bootstrap::state::AppState,
    common::{
        error::app_error::AppError,
        extractor::{CurrentUser, ValidatedJson},
        response::api_response::ApiResponse,
    },
    domain::{
        menu_permissions::dto::{AssignMenuPermissionRequest, MenuPermissionResponse},
        permission::entity::PermissionCode,
    },
};

#[utoipa::path(
    post,
    path = "/api/v1/iam/menus/{menu_id}/permissions",
    tag = "Menu Permission",
    params(
        ("menu_id" = u64, Path, description = "Menu ID")
    ),
    request_body = AssignMenuPermissionRequest,
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
            description = "Menu not found"
        )
    )
)]
pub async fn assign(
    current_user: CurrentUser,
    State(state): State<AppState>,
    Path(menu_id): Path<u64>,
    ConnectInfo(addr): ConnectInfo<std::net::SocketAddr>,
    headers: HeaderMap,
    ValidatedJson(request): ValidatedJson<AssignMenuPermissionRequest>,
) -> Result<(StatusCode, Json<ApiResponse<()>>), AppError> {
    current_user.require(PermissionCode::MenuPermissionAssign)?;

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
        .menu_permissions
        .assign(
            menu_id,
            request,
            Some(current_user.user_id()),
            ip_address,
            user_agent,
        )
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
    path = "/api/v1/iam/menus/{menu_id}/permissions/{permission_id}",
    tag = "Menu Permission",
    params(
        ("menu_id" = u64, Path, description = "Menu ID"),
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
            description = "Menu or Permission not found"
        )
    )
)]
pub async fn revoke(
    current_user: CurrentUser,
    State(state): State<AppState>,
    Path((menu_id, permission_id)): Path<(u64, u64)>,
    ConnectInfo(addr): ConnectInfo<std::net::SocketAddr>,
    headers: HeaderMap,
) -> Result<(StatusCode, Json<ApiResponse<()>>), AppError> {
    current_user.require(PermissionCode::MenuPermissionRevoke)?;

    let ip_address = Some(addr.ip().to_string());
    let user_agent = headers
        .get(axum::http::header::USER_AGENT)
        .and_then(|v| v.to_str().ok())
        .map(str::to_owned);

    state
        .services
        .menu_permissions
        .revoke(
            menu_id,
            permission_id,
            Some(current_user.user_id()),
            ip_address,
            user_agent,
        )
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
    path = "/api/v1/iam/menus/{menu_id}/permissions",
    tag = "Menu Permission",
    params(
        ("menu_id" = u64, Path, description = "Menu ID")
    ),
    responses(
        (
            status = 200,
            description = "Permissions retrieved successfully",
            body = ApiResponse<Vec<MenuPermissionResponse>>
        ),
        (
            status = 401,
            description = "Unauthorized"
        ),
        (
            status = 404,
            description = "Menu not found"
        )
    )
)]
pub async fn list(
    current_user: CurrentUser,
    State(state): State<AppState>,
    Path(menu_id): Path<u64>,
) -> Result<(StatusCode, Json<ApiResponse<Vec<MenuPermissionResponse>>>), AppError> {
    current_user.require(PermissionCode::MenuPermissionRead)?;

    let response = state
        .services
        .menu_permissions
        .list(menu_id)
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
