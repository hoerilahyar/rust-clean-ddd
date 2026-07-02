use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use validator::Validate;

use crate::{
    bootstrap::state::AppState,
    common::{error::app_error::AppError, response::api_response::ApiResponse},
    domain::user::dto::{
        CreateUserRequest, GetUserRequest, ListUserRequest, UpdateUserRequest, UserListResponse,
        UserResponse,
    },
};

pub async fn create(
    State(state): State<AppState>,
    Json(request): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<ApiResponse<u64>>), AppError> {
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

pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<u64>,
    Json(request): Json<UpdateUserRequest>,
) -> Result<(StatusCode, Json<ApiResponse<()>>), AppError> {
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

pub async fn delete(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<(StatusCode, Json<ApiResponse<()>>), AppError> {
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

pub async fn find_by_id(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<(StatusCode, Json<ApiResponse<UserResponse>>), AppError> {
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

pub async fn list(
    State(state): State<AppState>,
    Query(request): Query<ListUserRequest>,
) -> Result<(StatusCode, Json<ApiResponse<UserListResponse>>), AppError> {
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
