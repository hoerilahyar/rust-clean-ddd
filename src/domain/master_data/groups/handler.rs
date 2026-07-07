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
        master_data::groups::dto::{
            CreateMasterDataGroupRequest, ListMasterDataGroupRequest, MasterDataGroupListResponse,
            MasterDataGroupResponse, UpdateMasterDataGroupRequest,
        },
        permission::entity::PermissionCode,
    },
};

fn client_meta(
    addr: std::net::SocketAddr,
    headers: &HeaderMap,
) -> (Option<String>, Option<String>) {
    let ip_address = Some(addr.ip().to_string());
    let user_agent = headers
        .get(axum::http::header::USER_AGENT)
        .and_then(|v| v.to_str().ok())
        .map(str::to_owned);
    (ip_address, user_agent)
}

#[utoipa::path(
    post,
    path = "/api/v1/master/groups",
    tag = "Master Data Groups",
    request_body = CreateMasterDataGroupRequest,
    responses(
        (status = 201, description = "Group created successfully", body = ApiResponse<u64>),
        (status = 400, description = "Validation error"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden")
    )
)]
pub async fn create_group(
    current_user: CurrentUser,
    State(state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<std::net::SocketAddr>,
    headers: HeaderMap,
    ValidatedJson(request): ValidatedJson<CreateMasterDataGroupRequest>,
) -> Result<(StatusCode, Json<ApiResponse<u64>>), AppError> {
    current_user.require(PermissionCode::MasterDataGroupCreate)?;

    request
        .validate()
        .map_err(|e| AppError::Validation(vec![("request".to_string(), e.to_string())]))?;

    let (ip_address, user_agent) = client_meta(addr, &headers);

    let id = state
        .services
        .master_group
        .create_group(
            request,
            Some(current_user.user_id()),
            ip_address,
            user_agent,
        )
        .await
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse::success(id, "Group created successfully")),
    ))
}

#[utoipa::path(
    put,
    path = "/api/v1/master/groups/{code}",
    tag = "Master Data Groups",
    params(("code" = String, Path, description = "Group code")),
    request_body = UpdateMasterDataGroupRequest,
    responses(
        (status = 200, description = "Group updated successfully"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Group not found")
    )
)]
pub async fn update_group(
    current_user: CurrentUser,
    State(state): State<AppState>,
    Path(code): Path<String>,
    ConnectInfo(addr): ConnectInfo<std::net::SocketAddr>,
    headers: HeaderMap,
    ValidatedJson(request): ValidatedJson<UpdateMasterDataGroupRequest>,
) -> Result<(StatusCode, Json<ApiResponse<()>>), AppError> {
    current_user.require(PermissionCode::MasterDataGroupUpdate)?;

    let (ip_address, user_agent) = client_meta(addr, &headers);

    state
        .services
        .master_group
        .update_group(
            &code,
            request,
            Some(current_user.user_id()),
            ip_address,
            user_agent,
        )
        .await
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::<()>::message("Group updated successfully")),
    ))
}

#[utoipa::path(
    delete,
    path = "/api/v1/master/groups/{code}",
    tag = "Master Data Groups",
    params(("code" = String, Path, description = "Group code")),
    responses(
        (status = 200, description = "Group deleted successfully"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Group not found")
    )
)]
pub async fn delete_group(
    current_user: CurrentUser,
    State(state): State<AppState>,
    Path(code): Path<String>,
    ConnectInfo(addr): ConnectInfo<std::net::SocketAddr>,
    headers: HeaderMap,
) -> Result<(StatusCode, Json<ApiResponse<()>>), AppError> {
    current_user.require(PermissionCode::MasterDataGroupDelete)?;

    let (ip_address, user_agent) = client_meta(addr, &headers);

    state
        .services
        .master_group
        .delete_group(&code, Some(current_user.user_id()), ip_address, user_agent)
        .await
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::<()>::message("Group deleted successfully")),
    ))
}

#[utoipa::path(
    get,
    path = "/api/v1/master/groups/{code}",
    tag = "Master Data Groups",
    params(("code" = String, Path, description = "Group code")),
    responses(
        (status = 200, description = "Group retrieved successfully", body = ApiResponse<MasterDataGroupResponse>),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Group not found")
    )
)]
pub async fn find_group_by_code(
    current_user: CurrentUser,
    State(state): State<AppState>,
    Path(code): Path<String>,
) -> Result<(StatusCode, Json<ApiResponse<MasterDataGroupResponse>>), AppError> {
    current_user.require(PermissionCode::MasterDataGroupRead)?;

    let response = state
        .services
        .master_group
        .find_group_by_code(&code)
        .await
        .map_err(|e| AppError::NotFound(e.to_string()))?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success(
            response,
            "Group retrieved successfully",
        )),
    ))
}

#[utoipa::path(
    get,
    path = "/api/v1/master/groups",
    tag = "Master Data Groups",
    params(ListMasterDataGroupRequest),
    responses(
        (status = 200, description = "Groups retrieved successfully", body = ApiResponse<MasterDataGroupListResponse>),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden")
    )
)]
pub async fn list_groups(
    current_user: CurrentUser,
    State(state): State<AppState>,
    Query(request): Query<ListMasterDataGroupRequest>,
) -> Result<(StatusCode, Json<ApiResponse<MasterDataGroupListResponse>>), AppError> {
    current_user.require(PermissionCode::MasterDataGroupRead)?;

    let response = state
        .services
        .master_group
        .list_groups(request)
        .await
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success(
            response,
            "Groups retrieved successfully",
        )),
    ))
}
