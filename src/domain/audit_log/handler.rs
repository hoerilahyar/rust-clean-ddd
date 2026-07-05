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
        audit_log::dto::{AuditLogListResponse, AuditLogQueryRequest, AuditLogResponse},
        permission::entity::PermissionCode,
    },
};

#[utoipa::path(
    get,
    path = "/api/v1/audit-logs",
    tag = "Audit Log",
    params(AuditLogQueryRequest),
    responses(
        (status = 200, description = "List audit logs", body = AuditLogListResponse),
        (status = 400, description = "Validation error"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn list_audit_logs(
    current_user: CurrentUser,
    State(state): State<AppState>,
    Query(query): Query<AuditLogQueryRequest>,
) -> Result<(StatusCode, Json<ApiResponse<AuditLogListResponse>>), AppError> {
    current_user.require(PermissionCode::AuditLogRead)?;

    query
        .validate()
        .map_err(|e| AppError::Validation(vec![("query".into(), e.to_string())]))?;

    let response = state
        .services
        .audit_logs
        .list(query)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success(response, "Audit logs retrieved")),
    ))
}

#[utoipa::path(
    get,
    path = "/api/v1/audit-logs/{id}",
    tag = "Audit Log",
    params(
        ("id" = u64, Path, description = "Audit log id")
    ),
    responses(
        (status = 200, description = "Audit log detail", body = AuditLogResponse),
        (status = 404, description = "Audit log not found")
    )
)]
pub async fn get_audit_log(
    current_user: CurrentUser,
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<(StatusCode, Json<ApiResponse<AuditLogResponse>>), AppError> {
    current_user.require(PermissionCode::AuditLogRead)?;

    let response = state
        .services
        .audit_logs
        .get_by_id(id)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success(response, "Audit log retrieved")),
    ))
}
