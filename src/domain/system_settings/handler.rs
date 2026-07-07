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
        system_settings::dto::{
            ListSystemSettingRequest, SetSystemSettingActiveRequest, SystemSettingListResponse,
            SystemSettingResponse, UpsertSystemSettingRequest,
        },
    },
};

#[utoipa::path(
    put,
    path = "/api/v1/system-settings",
    tag = "System Setting",
    request_body = UpsertSystemSettingRequest,
    responses(
        (status = 200, description = "Setting saved successfully", body = ApiResponse<SystemSettingResponse>),
        (status = 400, description = "Validation error"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden")
    )
)]

pub async fn upsert(
    current_user: CurrentUser,
    State(state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<std::net::SocketAddr>,
    headers: HeaderMap,
    ValidatedJson(request): ValidatedJson<UpsertSystemSettingRequest>,
) -> Result<(StatusCode, Json<ApiResponse<SystemSettingResponse>>), AppError> {
    current_user.require(PermissionCode::SystemSettingUpdate)?;

    request
        .validate()
        .map_err(|e| AppError::Validation(vec![("request".to_string(), e.to_string())]))?;

    let ip_address = Some(addr.ip().to_string());
    let user_agent = headers
        .get(axum::http::header::USER_AGENT)
        .and_then(|v| v.to_str().ok())
        .map(str::to_owned);

    let response = state
        .services
        .system_setting
        .upsert(
            request,
            Some(current_user.user_id()),
            ip_address,
            user_agent,
        )
        .await
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success(response, "Setting saved successfully")),
    ))
}

#[utoipa::path(
    patch,
    path = "/api/v1/system-settings/{key}/active",
    tag = "System Setting",
    params(
        ("is_active" = bool, Query, description = "Active status")
    ),
    request_body = SetSystemSettingActiveRequest,
    responses(
        (status = 200, description = "Setting status updated successfully"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Setting not found")
    )
)]
pub async fn set_active(
    current_user: CurrentUser,
    State(state): State<AppState>,
    Path(key): Path<String>,
    ConnectInfo(addr): ConnectInfo<std::net::SocketAddr>,
    headers: HeaderMap,
    ValidatedJson(request): ValidatedJson<SetSystemSettingActiveRequest>,
) -> Result<(StatusCode, Json<ApiResponse<()>>), AppError> {
    current_user.require(PermissionCode::SystemSettingUpdate)?;

    let ip_address = Some(addr.ip().to_string());
    let user_agent = headers
        .get(axum::http::header::USER_AGENT)
        .and_then(|v| v.to_str().ok())
        .map(str::to_owned);

    state
        .services
        .system_setting
        .set_active(
            &key,
            request.is_active,
            Some(current_user.user_id()),
            ip_address,
            user_agent,
        )
        .await
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::<()>::message(
            "Setting status updated successfully",
        )),
    ))
}

#[derive(Debug, serde::Deserialize)]
pub struct SetActiveQuery {
    pub is_active: bool,
}

#[utoipa::path(
    delete,
    path = "/api/v1/system-settings/{key}",
    tag = "System Setting",
    params(
        ("key" = String, Path, description = "Setting key")
    ),
    responses(
        (status = 200, description = "Setting deleted successfully"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Setting not found")
    )
)]
pub async fn delete(
    current_user: CurrentUser,
    State(state): State<AppState>,
    Path(key): Path<String>,
    ConnectInfo(addr): ConnectInfo<std::net::SocketAddr>,
    headers: HeaderMap,
) -> Result<(StatusCode, Json<ApiResponse<()>>), AppError> {
    current_user.require(PermissionCode::SystemSettingDelete)?;

    let ip_address = Some(addr.ip().to_string());
    let user_agent = headers
        .get(axum::http::header::USER_AGENT)
        .and_then(|v| v.to_str().ok())
        .map(str::to_owned);

    state
        .services
        .system_setting
        .delete(&key, Some(current_user.user_id()), ip_address, user_agent)
        .await
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::<()>::message("Setting deleted successfully")),
    ))
}

#[utoipa::path(
    get,
    path = "/api/v1/system-settings/{key}",
    tag = "System Setting",
    params(
        ("key" = String, Path, description = "Setting key")
    ),
    responses(
        (status = 200, description = "Setting retrieved successfully", body = ApiResponse<SystemSettingResponse>),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Setting not found")
    )
)]
pub async fn find_by_key(
    current_user: CurrentUser,
    State(state): State<AppState>,
    Path(key): Path<String>,
) -> Result<(StatusCode, Json<ApiResponse<SystemSettingResponse>>), AppError> {
    current_user.require(PermissionCode::SystemSettingRead)?;

    let response = state
        .services
        .system_setting
        .find_by_key(&key)
        .await
        .map_err(|e| AppError::NotFound(e.to_string()))?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success(
            response,
            "Setting retrieved successfully",
        )),
    ))
}

#[utoipa::path(
    get,
    path = "/api/v1/system-settings",
    tag = "System Setting",
    params(ListSystemSettingRequest),
    responses(
        (status = 200, description = "Settings retrieved successfully", body = ApiResponse<SystemSettingListResponse>),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden")
    )
)]
pub async fn list(
    current_user: CurrentUser,
    State(state): State<AppState>,
    Query(request): Query<ListSystemSettingRequest>,
) -> Result<(StatusCode, Json<ApiResponse<SystemSettingListResponse>>), AppError> {
    current_user.require(PermissionCode::SystemSettingRead)?;

    let response = state
        .services
        .system_setting
        .list(request)
        .await
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success(
            response,
            "Settings retrieved successfully",
        )),
    ))
}
