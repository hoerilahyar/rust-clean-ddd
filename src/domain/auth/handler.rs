use axum::{
    Json,
    extract::{ConnectInfo, State},
    http::{HeaderMap, StatusCode},
};
use validator::Validate;

use crate::{
    bootstrap::state::AppState,
    common::{
        error::app_error::AppError, extractor::ValidatedJson, response::api_response::ApiResponse,
    },
    domain::auth::dto::{
        LoginRequest, LoginResponse, LogoutAllRequest, RefreshTokenRequest, RefreshTokenResponse,
    },
};

#[utoipa::path(
    post,
    path = "/api/v1/auth/login",
    tag = "Authentication",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = LoginResponse),
        (status = 400, description = "Validation error"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn login(
    State(state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<std::net::SocketAddr>,
    headers: HeaderMap,
    ValidatedJson(request): ValidatedJson<LoginRequest>,
) -> Result<(StatusCode, Json<ApiResponse<LoginResponse>>), AppError> {
    request
        .validate()
        .map_err(|e| AppError::Validation(vec![("request".into(), e.to_string())]))?;

    let ip_address = Some(addr.ip().to_string());

    let user_agent = headers
        .get(axum::http::header::USER_AGENT)
        .and_then(|v| v.to_str().ok())
        .map(str::to_owned);

    let response = state
        .services
        .auth
        .login(request, ip_address, user_agent)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success(response, "Login successful")),
    ))
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/refresh",
    tag = "Authentication",
    request_body = RefreshTokenRequest,
    responses(
        (status = 200, description = "Refresh token successful", body = RefreshTokenResponse),
        (status = 400, description = "Validation error"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn refresh_token(
    State(state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<std::net::SocketAddr>,
    headers: HeaderMap,
    ValidatedJson(request): ValidatedJson<RefreshTokenRequest>,
) -> Result<(StatusCode, Json<ApiResponse<RefreshTokenResponse>>), AppError> {
    request
        .validate()
        .map_err(|e| AppError::Validation(vec![("request".into(), e.to_string())]))?;

    let ip_address = Some(addr.ip().to_string());

    let user_agent = headers
        .get(axum::http::header::USER_AGENT)
        .and_then(|v| v.to_str().ok())
        .map(str::to_owned);

    let response = state
        .services
        .auth
        .refresh_token(request, ip_address, user_agent)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success(response, "Refresh token successful")),
    ))
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/logout",
    tag = "Authentication",
    responses(
        (status = 200, description = "Logout successful"),
        (status = 400, description = "Validation error"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn logout(
    State(state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<std::net::SocketAddr>,
    headers: HeaderMap,
    ValidatedJson(request): ValidatedJson<RefreshTokenRequest>,
) -> Result<(StatusCode, Json<ApiResponse<()>>), AppError> {
    request
        .validate()
        .map_err(|e| AppError::Validation(vec![("request".into(), e.to_string())]))?;

    let ip_address = Some(addr.ip().to_string());
    let user_agent = headers
        .get(axum::http::header::USER_AGENT)
        .and_then(|v| v.to_str().ok())
        .map(str::to_owned);

    state
        .services
        .auth
        .logout(request, ip_address, user_agent)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::<()>::message("Logout successful")),
    ))
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/logout-all",
    tag = "Authentication",
    request_body = LogoutAllRequest,
    responses(
        (
            status = 200,
            description = "Logout from all devices successful"
        ),
        (
            status = 400,
            description = "Bad request"
        ),
        (
            status = 401,
            description = "Unauthorized"
        )
    )
)]
pub async fn logout_all(
    State(state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<std::net::SocketAddr>,
    headers: HeaderMap,
    ValidatedJson(request): ValidatedJson<LogoutAllRequest>,
) -> Result<(StatusCode, Json<ApiResponse<()>>), AppError> {
    let ip_address = Some(addr.ip().to_string());
    let user_agent = headers
        .get(axum::http::header::USER_AGENT)
        .and_then(|v| v.to_str().ok())
        .map(str::to_owned);

    state
        .services
        .auth
        .logout_all(request.user_id, ip_address, user_agent)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::<()>::message(
            "Logout from all devices successful",
        )),
    ))
}
