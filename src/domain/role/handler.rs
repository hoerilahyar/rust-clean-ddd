use axum::{
    Json,
    extract::{ConnectInfo, Path, Query, State},
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
        permission::entity::PermissionCode,
        role::dto::{
            CreateRoleRequest, GetRoleRequest, ListRoleRequest, RoleListResponse, RoleResponse,
            UpdateRoleRequest,
        },
    },
};

#[utoipa::path(
    post,
    path = "/api/v1/iam/roles",
    tag = "Role",
    request_body = CreateRoleRequest,
    responses(
        (
            status = 201,
            description = "Role created successfully",
            body = ApiResponse<u64>
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
            status = 403,
            description = "Forbidden"
        )
    )
)]
pub async fn create(
    current_user: CurrentUser,
    State(state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<std::net::SocketAddr>,
    headers: HeaderMap,
    ValidatedJson(request): ValidatedJson<CreateRoleRequest>,
) -> Result<(StatusCode, Json<ApiResponse<u64>>), AppError> {
    current_user.require(PermissionCode::RoleCreate)?;

    request
        .validate()
        .map_err(|e| AppError::Validation(vec![("request".to_string(), e.to_string())]))?;

    let ip_address = Some(addr.ip().to_string());
    let user_agent = headers
        .get(axum::http::header::USER_AGENT)
        .and_then(|v| v.to_str().ok())
        .map(str::to_owned);

    let id = state
        .services
        .role
        .create(
            request,
            Some(current_user.user_id()),
            ip_address,
            user_agent,
        )
        .await
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse::success(id, "Role created successfully")),
    ))
}

#[utoipa::path(
    put,
    path = "/api/v1/iam/roles/{id}",
    tag = "Role",
    params(
        ("id" = u64, Path, description = "Role ID")
    ),
    request_body = UpdateRoleRequest,
    responses(
        (
            status = 200,
            description = "Role updated successfully"
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
            status = 403,
            description = "Forbidden"
        ),
        (
            status = 404,
            description = "Role not found"
        )
    )
)]
pub async fn update(
    current_user: CurrentUser,
    State(state): State<AppState>,
    Path(id): Path<u64>,
    ConnectInfo(addr): ConnectInfo<std::net::SocketAddr>,
    headers: HeaderMap,
    ValidatedJson(request): ValidatedJson<UpdateRoleRequest>,
) -> Result<(StatusCode, Json<ApiResponse<()>>), AppError> {
    current_user.require(PermissionCode::RoleUpdate)?;

    request
        .validate()
        .map_err(|e| AppError::Validation(vec![("request".to_string(), e.to_string())]))?;

    let ip_address = Some(addr.ip().to_string());
    let user_agent = headers
        .get(axum::http::header::USER_AGENT)
        .and_then(|v| v.to_str().ok())
        .map(str::to_owned);

    state
        .services
        .role
        .update(
            id,
            request,
            Some(current_user.user_id()),
            ip_address,
            user_agent,
        )
        .await
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::<()>::message("Role updated successfully")),
    ))
}

#[utoipa::path(
    delete,
    path = "/api/v1/iam/roles/{id}",
    tag = "Role",
    params(
        ("id" = u64, Path, description = "Role ID")
    ),
    responses(
        (
            status = 200,
            description = "Role deleted successfully"
        ),
        (
            status = 401,
            description = "Unauthorized"
        ),
        (
            status = 403,
            description = "Forbidden"
        ),
        (
            status = 404,
            description = "Role not found"
        )
    )
)]
pub async fn delete(
    current_user: CurrentUser,
    State(state): State<AppState>,
    Path(id): Path<u64>,
    ConnectInfo(addr): ConnectInfo<std::net::SocketAddr>,
    headers: HeaderMap,
) -> Result<(StatusCode, Json<ApiResponse<()>>), AppError> {
    current_user.require(PermissionCode::RoleDelete)?;

    let ip_address = Some(addr.ip().to_string());
    let user_agent = headers
        .get(axum::http::header::USER_AGENT)
        .and_then(|v| v.to_str().ok())
        .map(str::to_owned);

    state
        .services
        .role
        .delete(id, Some(current_user.user_id()), ip_address, user_agent)
        .await
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::<()>::message("Role deleted successfully")),
    ))
}

#[utoipa::path(
    get,
    path = "/api/v1/iam/roles/{id}",
    tag = "Role",
    params(
        ("id" = u64, Path, description = "Role ID")
    ),
    responses(
        (
            status = 200,
            description = "Role retrieved successfully",
            body = ApiResponse<RoleResponse>
        ),
        (
            status = 401,
            description = "Unauthorized"
        ),
        (
            status = 403,
            description = "Forbidden"
        ),
        (
            status = 404,
            description = "Role not found"
        )
    )
)]
pub async fn find_by_id(
    current_user: CurrentUser,
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<(StatusCode, Json<ApiResponse<RoleResponse>>), AppError> {
    current_user.require(PermissionCode::RoleRead)?;

    let response = state
        .services
        .role
        .find_by_id(GetRoleRequest { id })
        .await
        .map_err(|e| AppError::NotFound(e.to_string()))?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success(
            response,
            "Role retrieved successfully",
        )),
    ))
}

#[utoipa::path(
    get,
    path = "/api/v1/iam/roles",
    tag = "Role",
    params(ListRoleRequest),
    responses(
        (
            status = 200,
            description = "Roles retrieved successfully",
            body = ApiResponse<RoleListResponse>
        ),
        (
            status = 401,
            description = "Unauthorized"
        ),
        (
            status = 403,
            description = "Forbidden"
        )
    )
)]
pub async fn list(
    current_user: CurrentUser,
    State(state): State<AppState>,
    Query(request): Query<ListRoleRequest>,
) -> Result<(StatusCode, Json<ApiResponse<RoleListResponse>>), AppError> {
    current_user.require(PermissionCode::RoleRead)?;

    let response = state
        .services
        .role
        .list(request)
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
