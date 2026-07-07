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
    domain::session::dto::{ListSessionQuery, RevokeOtherSessionsRequest, SessionResponse},
};

#[utoipa::path(
    get,
    path = "/api/v1/sessions",
    tag = "Session",
    params(ListSessionQuery),
    responses(
        (status = 200, description = "Active sessions retrieved successfully", body = ApiResponse<Vec<SessionResponse>>),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn list(
    current_user: CurrentUser,
    State(state): State<AppState>,
    Query(query): Query<ListSessionQuery>,
) -> Result<(StatusCode, Json<ApiResponse<Vec<SessionResponse>>>), AppError> {
    let response = state
        .services
        .session
        .list(current_user.user_id(), query.device_id)
        .await
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success(
            response,
            "Active sessions retrieved successfully",
        )),
    ))
}

#[utoipa::path(
    delete,
    path = "/api/v1/sessions/{id}",
    tag = "Session",
    params(
        ("id" = u64, Path, description = "Session id")
    ),
    responses(
        (status = 200, description = "Session revoked successfully"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Session not found")
    )
)]
pub async fn revoke(
    current_user: CurrentUser,
    State(state): State<AppState>,
    Path(id): Path<u64>,
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
        .session
        .revoke(current_user.user_id(), id, ip_address, user_agent)
        .await
        .map_err(|e| AppError::NotFound(e.to_string()))?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::<()>::message("Session revoked successfully")),
    ))
}

#[utoipa::path(
    post,
    path = "/api/v1/sessions/revoke-others",
    tag = "Session",
    request_body = RevokeOtherSessionsRequest,
    responses(
        (status = 200, description = "Other sessions revoked successfully"),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn revoke_others(
    current_user: CurrentUser,
    State(state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<std::net::SocketAddr>,
    headers: HeaderMap,
    ValidatedJson(request): ValidatedJson<RevokeOtherSessionsRequest>,
) -> Result<(StatusCode, Json<ApiResponse<()>>), AppError> {
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
        .session
        .revoke_others(
            current_user.user_id(),
            request.device_id,
            ip_address,
            user_agent,
        )
        .await
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::<()>::message(
            "Other sessions revoked successfully",
        )),
    ))
}
