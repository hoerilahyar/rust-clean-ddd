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

#[utoipa::path(
    post,
    path = "/api/v1/permissions",
    tag = "Permission",
    request_body = CreatePermissionRequest,
    responses(
        (
            status = 201,
            description = "Permission created successfully",
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

#[utoipa::path(
    put,
    path = "/api/v1/permissions/{id}",
    tag = "Permission",
    params(
        ("id" = u64, Path, description = "Permission ID")
    ),
    request_body = UpdatePermissionRequest,
    responses(
        (
            status = 200,
            description = "Permission updated successfully"
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
            description = "Permission not found"
        )
    )
)]
pub async fn update(
    current_user: CurrentUser,
    State(state): State<AppState>,
    Path(id): Path<u64>,
    Json(request): Json<UpdatePermissionRequest>,
) -> Result<(StatusCode, Json<ApiResponse<()>>), AppError> {
    current_user.require(PermissionCode::PermissionUpdate)?;

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

#[utoipa::path(
    delete,
    path = "/api/v1/permissions/{id}",
    tag = "Permission",
    params(
        ("id" = u64, Path, description = "Permission ID")
    ),
    responses(
        (
            status = 200,
            description = "Permission deleted successfully"
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
            description = "Permission not found"
        )
    )
)]
pub async fn delete(
    current_user: CurrentUser,
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<(StatusCode, Json<ApiResponse<()>>), AppError> {
    current_user.require(PermissionCode::PermissionDelete)?;

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

#[utoipa::path(
    get,
    path = "/api/v1/permissions/{id}",
    tag = "Permission",
    params(
        ("id" = u64, Path, description = "Permission ID")
    ),
    responses(
        (
            status = 200,
            description = "Permission retrieved successfully",
            body = ApiResponse<PermissionResponse>
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
            description = "Permission not found"
        )
    )
)]
pub async fn find_by_id(
    current_user: CurrentUser,
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<(StatusCode, Json<ApiResponse<PermissionResponse>>), AppError> {
    current_user.require(PermissionCode::PermissionRead)?;

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

#[utoipa::path(
    get,
    path = "/api/v1/permissions",
    tag = "Permission",
    params(ListPermissionRequest),
    responses(
        (
            status = 200,
            description = "Permissions retrieved successfully",
            body = ApiResponse<PermissionListResponse>
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
    Query(request): Query<ListPermissionRequest>,
) -> Result<(StatusCode, Json<ApiResponse<PermissionListResponse>>), AppError> {
    current_user.require(PermissionCode::PermissionRead)?;

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
