use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use validator::Validate;

use crate::{
    bootstrap::state::AppState,
    common::{error::app_error::AppError, response::api_response::ApiResponse},
    domain::role::dto::{
        CreateRoleRequest, GetRoleRequest, ListRoleRequest, RoleListResponse, RoleResponse,
        UpdateRoleRequest,
    },
};

pub async fn create(
    State(state): State<AppState>,
    Json(request): Json<CreateRoleRequest>,
) -> Result<(StatusCode, Json<ApiResponse<u64>>), AppError> {
    request
        .validate()
        .map_err(|e| AppError::Validation(vec![("request".to_string(), e.to_string())]))?;

    let id = state
        .services
        .role
        .create(request)
        .await
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse::success(id, "Role created successfully")),
    ))
}

pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<u64>,
    Json(request): Json<UpdateRoleRequest>,
) -> Result<(StatusCode, Json<ApiResponse<()>>), AppError> {
    request
        .validate()
        .map_err(|e| AppError::Validation(vec![("request".to_string(), e.to_string())]))?;

    state
        .services
        .role
        .update(id, request)
        .await
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::<()>::message("Role updated successfully")),
    ))
}

pub async fn delete(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<(StatusCode, Json<ApiResponse<()>>), AppError> {
    state
        .services
        .role
        .delete(id)
        .await
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::<()>::message("Role deleted successfully")),
    ))
}

pub async fn find_by_id(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<(StatusCode, Json<ApiResponse<RoleResponse>>), AppError> {
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

pub async fn list(
    State(state): State<AppState>,
    Query(request): Query<ListRoleRequest>,
) -> Result<(StatusCode, Json<ApiResponse<RoleListResponse>>), AppError> {
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
