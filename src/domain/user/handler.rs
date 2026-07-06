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
        user::dto::{
            CreateUserRequest, GetUserRequest, ListUserRequest, UpdateUserRequest,
            UserListResponse, UserResponse,
        },
    },
};

#[utoipa::path(
    post,
    path = "/api/v1/users",
    tag = "User",
    request_body = CreateUserRequest,
    responses(
        (
            status = 201,
            description = "User created successfully",
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
    ValidatedJson(request): ValidatedJson<CreateUserRequest>,
) -> Result<(StatusCode, Json<ApiResponse<u64>>), AppError> {
    current_user.require(PermissionCode::UserCreate)?;

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
        .user
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
        Json(ApiResponse::success(id, "User created successfully")),
    ))
}

#[utoipa::path(
    put,
    path = "/api/v1/users/{id}",
    tag = "User",
    params(
        ("id" = u64, Path, description = "User ID")
    ),
    request_body = UpdateUserRequest,
    responses(
        (
            status = 200,
            description = "User updated successfully"
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
            description = "User not found"
        )
    )
)]
pub async fn update(
    current_user: CurrentUser,
    State(state): State<AppState>,
    Path(id): Path<u64>,
    ConnectInfo(addr): ConnectInfo<std::net::SocketAddr>,
    headers: HeaderMap,
    ValidatedJson(request): ValidatedJson<UpdateUserRequest>,
) -> Result<(StatusCode, Json<ApiResponse<()>>), AppError> {
    current_user.require(PermissionCode::UserUpdate)?;

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
        .user
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
        Json(ApiResponse::<()>::message("User updated successfully")),
    ))
}

#[utoipa::path(
    delete,
    path = "/api/v1/users/{id}",
    tag = "User",
    params(
        ("id" = u64, Path, description = "User ID")
    ),
    responses(
        (
            status = 200,
            description = "User deleted successfully"
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
            description = "User not found"
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
    current_user.require(PermissionCode::UserDelete)?;

    let ip_address = Some(addr.ip().to_string());
    let user_agent = headers
        .get(axum::http::header::USER_AGENT)
        .and_then(|v| v.to_str().ok())
        .map(str::to_owned);

    state
        .services
        .user
        .delete(id, Some(current_user.user_id()), ip_address, user_agent)
        .await
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::<()>::message("User deleted successfully")),
    ))
}

#[utoipa::path(
    get,
    path = "/api/v1/users/{id}",
    tag = "User",
    params(
        ("id" = u64, Path, description = "User ID")
    ),
    responses(
        (
            status = 200,
            description = "User retrieved successfully",
            body = ApiResponse<UserResponse>
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
            description = "User not found"
        )
    )
)]
pub async fn find_by_id(
    current_user: CurrentUser,
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<(StatusCode, Json<ApiResponse<UserResponse>>), AppError> {
    current_user.require(PermissionCode::UserRead)?;

    let response = state
        .services
        .user
        .find_by_id(GetUserRequest { id })
        .await
        .map_err(|e| AppError::NotFound(e.to_string()))?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success(
            response,
            "User retrieved successfully",
        )),
    ))
}

#[utoipa::path(
    get,
    path = "/api/v1/users",
    tag = "User",
    params(ListUserRequest),
    responses(
        (
            status = 200,
            description = "Users retrieved successfully",
            body = ApiResponse<UserListResponse>
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
    Query(request): Query<ListUserRequest>,
) -> Result<(StatusCode, Json<ApiResponse<UserListResponse>>), AppError> {
    current_user.require(PermissionCode::UserRead)?;

    let response = state
        .services
        .user
        .list(request)
        .await
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success(
            response,
            "Users retrieved successfully",
        )),
    ))
}
