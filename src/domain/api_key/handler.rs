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
        api_key::dto::{
            ApiKeyListResponse, ApiKeyResponse, CreateApiKeyRequest, CreateApiKeyResponse,
            ListApiKeyRequest, SetApiKeyActiveRequest, UpdateApiKeyRequest,
        },
        permission::entity::PermissionCode,
    },
};

fn request_meta(headers: &HeaderMap, addr: std::net::SocketAddr) -> (Option<String>, Option<String>) {
    let ip_address = Some(addr.ip().to_string());
    let user_agent = headers
        .get(axum::http::header::USER_AGENT)
        .and_then(|v| v.to_str().ok())
        .map(str::to_owned);

    (ip_address, user_agent)
}

#[utoipa::path(
    post,
    path = "/api/v1/api-keys",
    tag = "API Key",
    request_body = CreateApiKeyRequest,
    responses(
        (status = 201, description = "API key created successfully. The plaintext key is only ever shown here.", body = ApiResponse<CreateApiKeyResponse>),
        (status = 400, description = "Validation error or disallowed permission"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden")
    )
)]
pub async fn create(
    current_user: CurrentUser,
    State(state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<std::net::SocketAddr>,
    headers: HeaderMap,
    ValidatedJson(request): ValidatedJson<CreateApiKeyRequest>,
) -> Result<(StatusCode, Json<ApiResponse<CreateApiKeyResponse>>), AppError> {
    current_user.require(PermissionCode::ApiKeyCreate)?;

    request
        .validate()
        .map_err(|e| AppError::Validation(vec![("request".to_string(), e.to_string())]))?;

    let (ip_address, user_agent) = request_meta(&headers, addr);

    let response = state
        .services
        .api_key
        .create(
            request,
            Some(current_user.user_id()),
            &current_user.context.permissions,
            ip_address,
            user_agent,
        )
        .await
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse::success(
            response,
            "API key created successfully. Copy the key now — it won't be shown again.",
        )),
    ))
}

#[utoipa::path(
    get,
    path = "/api/v1/api-keys",
    tag = "API Key",
    params(ListApiKeyRequest),
    responses(
        (status = 200, description = "API keys retrieved successfully", body = ApiResponse<ApiKeyListResponse>),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden")
    )
)]
pub async fn list(
    current_user: CurrentUser,
    State(state): State<AppState>,
    Query(request): Query<ListApiKeyRequest>,
) -> Result<(StatusCode, Json<ApiResponse<ApiKeyListResponse>>), AppError> {
    current_user.require(PermissionCode::ApiKeyRead)?;

    let response = state
        .services
        .api_key
        .list(request)
        .await
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success(
            response,
            "API keys retrieved successfully",
        )),
    ))
}

#[utoipa::path(
    get,
    path = "/api/v1/api-keys/{id}",
    tag = "API Key",
    params(("id" = u64, Path, description = "API key ID")),
    responses(
        (status = 200, description = "API key retrieved successfully", body = ApiResponse<ApiKeyResponse>),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "API key not found")
    )
)]
pub async fn find_by_id(
    current_user: CurrentUser,
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<(StatusCode, Json<ApiResponse<ApiKeyResponse>>), AppError> {
    current_user.require(PermissionCode::ApiKeyRead)?;

    let response = state
        .services
        .api_key
        .find_by_id(id)
        .await
        .map_err(|e| AppError::NotFound(e.to_string()))?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success(
            response,
            "API key retrieved successfully",
        )),
    ))
}

#[utoipa::path(
    put,
    path = "/api/v1/api-keys/{id}",
    tag = "API Key",
    params(("id" = u64, Path, description = "API key ID")),
    request_body = UpdateApiKeyRequest,
    responses(
        (status = 200, description = "API key updated successfully", body = ApiResponse<ApiKeyResponse>),
        (status = 400, description = "Validation error or disallowed permission"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "API key not found")
    )
)]
pub async fn update(
    current_user: CurrentUser,
    State(state): State<AppState>,
    Path(id): Path<u64>,
    ConnectInfo(addr): ConnectInfo<std::net::SocketAddr>,
    headers: HeaderMap,
    ValidatedJson(request): ValidatedJson<UpdateApiKeyRequest>,
) -> Result<(StatusCode, Json<ApiResponse<ApiKeyResponse>>), AppError> {
    current_user.require(PermissionCode::ApiKeyUpdate)?;

    let (ip_address, user_agent) = request_meta(&headers, addr);

    let response = state
        .services
        .api_key
        .update(
            id,
            request,
            Some(current_user.user_id()),
            &current_user.context.permissions,
            ip_address,
            user_agent,
        )
        .await
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success(response, "API key updated successfully")),
    ))
}

#[utoipa::path(
    patch,
    path = "/api/v1/api-keys/{id}/active",
    tag = "API Key",
    params(("id" = u64, Path, description = "API key ID")),
    request_body = SetApiKeyActiveRequest,
    responses(
        (status = 200, description = "API key status updated successfully"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "API key not found")
    )
)]
pub async fn set_active(
    current_user: CurrentUser,
    State(state): State<AppState>,
    Path(id): Path<u64>,
    ConnectInfo(addr): ConnectInfo<std::net::SocketAddr>,
    headers: HeaderMap,
    ValidatedJson(request): ValidatedJson<SetApiKeyActiveRequest>,
) -> Result<(StatusCode, Json<ApiResponse<()>>), AppError> {
    current_user.require(PermissionCode::ApiKeyRevoke)?;

    let (ip_address, user_agent) = request_meta(&headers, addr);

    state
        .services
        .api_key
        .set_active(
            id,
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
            "API key status updated successfully",
        )),
    ))
}

#[utoipa::path(
    delete,
    path = "/api/v1/api-keys/{id}",
    tag = "API Key",
    params(("id" = u64, Path, description = "API key ID")),
    responses(
        (status = 200, description = "API key deleted successfully"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "API key not found")
    )
)]
pub async fn delete(
    current_user: CurrentUser,
    State(state): State<AppState>,
    Path(id): Path<u64>,
    ConnectInfo(addr): ConnectInfo<std::net::SocketAddr>,
    headers: HeaderMap,
) -> Result<(StatusCode, Json<ApiResponse<()>>), AppError> {
    current_user.require(PermissionCode::ApiKeyDelete)?;

    let (ip_address, user_agent) = request_meta(&headers, addr);

    state
        .services
        .api_key
        .delete(id, Some(current_user.user_id()), ip_address, user_agent)
        .await
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::<()>::message("API key deleted successfully")),
    ))
}
