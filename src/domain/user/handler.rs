use axum::{
    Json,
    extract::{Path, Query, State},
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
    Json(request): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<ApiResponse<u64>>), AppError> {
    current_user.require(PermissionCode::UserCreate)?;

    request
        .validate()
        .map_err(|e| AppError::Validation(vec![("request".to_string(), e.to_string())]))?;

    let id = state
        .services
        .user
        .create(request)
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
    Json(request): Json<UpdateUserRequest>,
) -> Result<(StatusCode, Json<ApiResponse<()>>), AppError> {
    current_user.require(PermissionCode::UserUpdate)?;

    request
        .validate()
        .map_err(|e| AppError::Validation(vec![("request".to_string(), e.to_string())]))?;

    state
        .services
        .user
        .update(id, request)
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
) -> Result<(StatusCode, Json<ApiResponse<()>>), AppError> {
    current_user.require(PermissionCode::UserDelete)?;

    state
        .services
        .user
        .delete(id)
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
