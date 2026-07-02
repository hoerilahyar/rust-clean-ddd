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
    domain::permission::{
        dto::{
            CreatePermissionRequest, GetPermissionRequest, ListPermissionRequest,
            PermissionListResponse, PermissionResponse, UpdatePermissionRequest,
        },
        entity::PermissionCode,
    },
};

pub async fn create(
    current_user: CurrentUser,
    State(state): State<AppState>,
    Json(request): Json<CreatePermissionRequest>,
) -> Result<(StatusCode, Json<ApiResponse<u64>>), AppError> {
    current_user.require(PermissionCode::PermissionCreate)?;

    request
        .validate()
        .map_err(|e| AppError::Validation(vec![("request".into(), e.to_string())]))?;

    let id = state
        .services
        .permission
        .create(request)
        .await
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse::success(id, "Permission created successfully")),
    ))
}

pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<u64>,
    Json(request): Json<UpdatePermissionRequest>,
) -> Result<(StatusCode, Json<ApiResponse<()>>), AppError> {
    request
        .validate()
        .map_err(|e| AppError::Validation(vec![("request".into(), e.to_string())]))?;

    state
        .services
        .permission
        .update(id, request)
        .await
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::<()>::message(
            "Permission updated successfully",
        )),
    ))
}

pub async fn delete(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<(StatusCode, Json<ApiResponse<()>>), AppError> {
    state
        .services
        .permission
        .delete(id)
        .await
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::<()>::message(
            "Permission deleted successfully",
        )),
    ))
}

pub async fn find_by_id(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<(StatusCode, Json<ApiResponse<PermissionResponse>>), AppError> {
    let response = state
        .services
        .permission
        .find_by_id(GetPermissionRequest { id })
        .await
        .map_err(|e| AppError::NotFound(e.to_string()))?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success(
            response,
            "Permission retrieved successfully",
        )),
    ))
}

pub async fn list(
    State(state): State<AppState>,
    Query(request): Query<ListPermissionRequest>,
) -> Result<(StatusCode, Json<ApiResponse<PermissionListResponse>>), AppError> {
    let response = state
        .services
        .permission
        .list(request)
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
