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
    domain::user_setting::dto::{
        ListUserSettingRequest, SetUserSettingActiveRequest, UpsertUserSettingRequest,
        UserSettingListResponse, UserSettingResponse,
    },
};

#[utoipa::path(
    put,
    path = "/api/v1/user-settings",
    tag = "User Setting",
    request_body = UpsertUserSettingRequest,
    responses(
        (status = 200, description = "Setting saved successfully", body = ApiResponse<UserSettingResponse>),
        (status = 400, description = "Validation error"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn upsert(
    current_user: CurrentUser,
    State(state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<std::net::SocketAddr>,
    headers: HeaderMap,
    ValidatedJson(request): ValidatedJson<UpsertUserSettingRequest>,
) -> Result<(StatusCode, Json<ApiResponse<UserSettingResponse>>), AppError> {
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
        .user_setting
        .upsert(current_user.user_id(), request, ip_address, user_agent)
        .await
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success(response, "Setting saved successfully")),
    ))
}

#[utoipa::path(
    patch,
    path = "/api/v1/user-settings/{key}/active",
    tag = "User Setting",
    params(
        ("key" = String, Path, description = "Setting key")
    ),
    request_body = SetUserSettingActiveRequest,
    responses(
        (status = 200, description = "Setting status updated successfully"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Setting not found")
    )
)]
pub async fn set_active(
    current_user: CurrentUser,
    State(state): State<AppState>,
    Path(key): Path<String>,
    ConnectInfo(addr): ConnectInfo<std::net::SocketAddr>,
    headers: HeaderMap,
    ValidatedJson(request): ValidatedJson<SetUserSettingActiveRequest>,
) -> Result<(StatusCode, Json<ApiResponse<()>>), AppError> {
    let ip_address = Some(addr.ip().to_string());
    let user_agent = headers
        .get(axum::http::header::USER_AGENT)
        .and_then(|v| v.to_str().ok())
        .map(str::to_owned);

    state
        .services
        .user_setting
        .set_active(
            current_user.user_id(),
            &key,
            request.is_active,
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

#[utoipa::path(
    delete,
    path = "/api/v1/user-settings/{key}",
    tag = "User Setting",
    params(
        ("key" = String, Path, description = "Setting key")
    ),
    responses(
        (status = 200, description = "Setting deleted successfully"),
        (status = 401, description = "Unauthorized"),
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
    let ip_address = Some(addr.ip().to_string());
    let user_agent = headers
        .get(axum::http::header::USER_AGENT)
        .and_then(|v| v.to_str().ok())
        .map(str::to_owned);

    state
        .services
        .user_setting
        .delete(current_user.user_id(), &key, ip_address, user_agent)
        .await
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::<()>::message("Setting deleted successfully")),
    ))
}

#[utoipa::path(
    get,
    path = "/api/v1/user-settings/{key}",
    tag = "User Setting",
    params(
        ("key" = String, Path, description = "Setting key")
    ),
    responses(
        (status = 200, description = "Setting retrieved successfully", body = ApiResponse<UserSettingResponse>),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Setting not found")
    )
)]
pub async fn find_by_key(
    current_user: CurrentUser,
    State(state): State<AppState>,
    Path(key): Path<String>,
) -> Result<(StatusCode, Json<ApiResponse<UserSettingResponse>>), AppError> {
    let response = state
        .services
        .user_setting
        .find_by_key(current_user.user_id(), &key)
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
    path = "/api/v1/user-settings",
    tag = "User Setting",
    params(ListUserSettingRequest),
    responses(
        (status = 200, description = "Settings retrieved successfully", body = ApiResponse<UserSettingListResponse>),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn list(
    current_user: CurrentUser,
    State(state): State<AppState>,
    Query(request): Query<ListUserSettingRequest>,
) -> Result<(StatusCode, Json<ApiResponse<UserSettingListResponse>>), AppError> {
    let response = state
        .services
        .user_setting
        .list(current_user.user_id(), request)
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
