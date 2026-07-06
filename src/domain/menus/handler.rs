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
        menus::dto::{
            CreateMenuRequest, GetMenuRequest, ListMenuRequest, MenuListResponse, MenuResponse,
            UpdateMenuRequest,
        },
        permission::entity::PermissionCode,
    },
};

#[utoipa::path(
    post,
    path = "/api/v1/iam/menus",
    tag = "Menu",
    request_body = CreateMenuRequest,
    responses(
        (
            status = 201,
            description = "Menu created successfully",
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
    ConnectInfo(addr): ConnectInfo<std::net::SocketAddr>,
    headers: HeaderMap,
    ValidatedJson(request): ValidatedJson<CreateMenuRequest>,
) -> Result<(StatusCode, Json<ApiResponse<u64>>), AppError> {
    current_user.require(PermissionCode::MenuCreate)?;

    request
        .validate()
        .map_err(|e| AppError::Validation(vec![("request".to_string(), e.to_string())]))?;

    let ip_address = Some(addr.ip().to_string());
    let user_agent = headers
        .get(axum::http::header::USER_AGENT)
        .and_then(|v| v.to_str().ok())
        .map(str::to_owned);

    let id = state
        .services
        .menu
        .create(
            request,
            Some(current_user.user_id()),
            ip_address,
            user_agent,
        )
        .await
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse::success(id, "Menu created successfully")),
    ))
}

#[utoipa::path(
    put,
    path = "/api/v1/iam/menus/{id}",
    tag = "Menu",
    params(
        ("id" = u64, Path, description = "Menu ID")
    ),
    request_body = UpdateMenuRequest,
    responses(
        (
            status = 200,
            description = "Menu updated successfully"
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
            description = "Menu not found"
        )
    )
)]
pub async fn update(
    current_user: CurrentUser,
    State(state): State<AppState>,
    Path(id): Path<u64>,
    ConnectInfo(addr): ConnectInfo<std::net::SocketAddr>,
    headers: HeaderMap,
    ValidatedJson(request): ValidatedJson<UpdateMenuRequest>,
) -> Result<(StatusCode, Json<ApiResponse<()>>), AppError> {
    current_user.require(PermissionCode::MenuUpdate)?;

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
        .menu
        .update(
            id,
            request,
            Some(current_user.user_id()),
            ip_address,
            user_agent,
        )
        .await
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::<()>::message("Menu updated successfully")),
    ))
}

#[utoipa::path(
    delete,
    path = "/api/v1/iam/menus/{id}",
    tag = "Menu",
    params(
        ("id" = u64, Path, description = "Menu ID")
    ),
    responses(
        (
            status = 200,
            description = "Menu deleted successfully"
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
            description = "Menu not found"
        )
    )
)]
pub async fn delete(
    current_user: CurrentUser,
    State(state): State<AppState>,
    Path(id): Path<u64>,
    ConnectInfo(addr): ConnectInfo<std::net::SocketAddr>,
    headers: HeaderMap,
) -> Result<(StatusCode, Json<ApiResponse<()>>), AppError> {
    current_user.require(PermissionCode::MenuDelete)?;

    let ip_address = Some(addr.ip().to_string());
    let user_agent = headers
        .get(axum::http::header::USER_AGENT)
        .and_then(|v| v.to_str().ok())
        .map(str::to_owned);

    state
        .services
        .menu
        .delete(id, Some(current_user.user_id()), ip_address, user_agent)
        .await
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::<()>::message("Menu deleted successfully")),
    ))
}

#[utoipa::path(
    get,
    path = "/api/v1/iam/menus/{id}",
    tag = "Menu",
    params(
        ("id" = u64, Path, description = "Menu ID")
    ),
    responses(
        (
            status = 200,
            description = "Menu retrieved successfully",
            body = ApiResponse<MenuResponse>
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
            description = "Menu not found"
        )
    )
)]
pub async fn find_by_id(
    current_user: CurrentUser,
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<(StatusCode, Json<ApiResponse<MenuResponse>>), AppError> {
    current_user.require(PermissionCode::MenuRead)?;

    let response = state
        .services
        .menu
        .find_by_id(GetMenuRequest { id })
        .await
        .map_err(|e| AppError::NotFound(e.to_string()))?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success(
            response,
            "Menu retrieved successfully",
        )),
    ))
}

#[utoipa::path(
    get,
    path = "/api/v1/iam/menus",
    tag = "Menu",
    params(ListMenuRequest),
    responses(
        (
            status = 200,
            description = "Menus retrieved successfully",
            body = ApiResponse<MenuListResponse>
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
    Query(request): Query<ListMenuRequest>,
) -> Result<(StatusCode, Json<ApiResponse<MenuListResponse>>), AppError> {
    current_user.require(PermissionCode::MenuRead)?;

    let response = state
        .services
        .menu
        .list(request)
        .await
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success(
            response,
            "Menus retrieved successfully",
        )),
    ))
}
