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
        master_data::{
            groups::dto::MasterDataOptionListResponse,
            items::dto::{
                CreateMasterDataItemRequest, ListMasterDataItemRequest, MasterDataItemListResponse,
                MasterDataItemResponse, MasterDataOptionsQuery, UpdateMasterDataItemRequest,
            },
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
    path = "/api/v1/master/items/{group_code}/items",
    tag = "Master Data Items",
    params(("group_code" = String, Path, description = "Group code")),
    request_body = CreateMasterDataItemRequest,
    responses(
        (status = 201, description = "Item created successfully", body = ApiResponse<u64>),
        (status = 400, description = "Validation error"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden")
    )
)]
pub async fn create_item(
    current_user: CurrentUser,
    State(state): State<AppState>,
    Path(group_code): Path<String>,
    ConnectInfo(addr): ConnectInfo<std::net::SocketAddr>,
    headers: HeaderMap,
    ValidatedJson(request): ValidatedJson<CreateMasterDataItemRequest>,
) -> Result<(StatusCode, Json<ApiResponse<u64>>), AppError> {
    current_user.require(PermissionCode::MasterDataItemCreate)?;

    request
        .validate()
        .map_err(|e| AppError::Validation(vec![("request".to_string(), e.to_string())]))?;

    let (ip_address, user_agent) = client_meta(addr, &headers);

    let id = state
        .services
        .master_items
        .create_item(
            &group_code,
            request,
            Some(current_user.user_id()),
            ip_address,
            user_agent,
        )
        .await
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse::success(id, "Item created successfully")),
    ))
}

#[utoipa::path(
    put,
    path = "/api/v1/master/items/{group_code}/items/{id}",
    tag = "Master Data Items",
    params(
        ("group_code" = String, Path, description = "Group code"),
        ("id" = u64, Path, description = "Item id")
    ),
    request_body = UpdateMasterDataItemRequest,
    responses(
        (status = 200, description = "Item updated successfully"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Item not found")
    )
)]
pub async fn update_item(
    current_user: CurrentUser,
    State(state): State<AppState>,
    Path((group_code, id)): Path<(String, u64)>,
    ConnectInfo(addr): ConnectInfo<std::net::SocketAddr>,
    headers: HeaderMap,
    ValidatedJson(request): ValidatedJson<UpdateMasterDataItemRequest>,
) -> Result<(StatusCode, Json<ApiResponse<()>>), AppError> {
    current_user.require(PermissionCode::MasterDataItemUpdate)?;

    let (ip_address, user_agent) = client_meta(addr, &headers);

    state
        .services
        .master_items
        .update_item(
            &group_code,
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
        Json(ApiResponse::<()>::message("Item updated successfully")),
    ))
}

#[utoipa::path(
    delete,
    path = "/api/v1/master/items/{group_code}/items/{id}",
    tag = "Master Data Items",
    params(
        ("group_code" = String, Path, description = "Group code"),
        ("id" = u64, Path, description = "Item id")
    ),
    responses(
        (status = 200, description = "Item deleted successfully"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Item not found")
    )
)]
pub async fn delete_item(
    current_user: CurrentUser,
    State(state): State<AppState>,
    Path((group_code, id)): Path<(String, u64)>,
    ConnectInfo(addr): ConnectInfo<std::net::SocketAddr>,
    headers: HeaderMap,
) -> Result<(StatusCode, Json<ApiResponse<()>>), AppError> {
    current_user.require(PermissionCode::MasterDataItemDelete)?;

    let (ip_address, user_agent) = client_meta(addr, &headers);

    state
        .services
        .master_items
        .delete_item(
            &group_code,
            id,
            Some(current_user.user_id()),
            ip_address,
            user_agent,
        )
        .await
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::<()>::message("Item deleted successfully")),
    ))
}

#[utoipa::path(
    get,
    path = "/api/v1/master/items/{group_code}/items/{id}",
    tag = "Master Data Items",
    params(
        ("group_code" = String, Path, description = "Group code"),
        ("id" = u64, Path, description = "Item id")
    ),
    responses(
        (status = 200, description = "Item retrieved successfully", body = ApiResponse<MasterDataItemResponse>),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Item not found")
    )
)]
pub async fn find_item_by_id(
    current_user: CurrentUser,
    State(state): State<AppState>,
    Path((group_code, id)): Path<(String, u64)>,
) -> Result<(StatusCode, Json<ApiResponse<MasterDataItemResponse>>), AppError> {
    current_user.require(PermissionCode::MasterDataItemRead)?;

    let response = state
        .services
        .master_items
        .find_item_by_id(&group_code, id)
        .await
        .map_err(|e| AppError::NotFound(e.to_string()))?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success(
            response,
            "Item retrieved successfully",
        )),
    ))
}

#[utoipa::path(
    get,
    path = "/api/v1/master/items/{group_code}/items",
    tag = "Master Data Items",
    params(("group_code" = String, Path, description = "Group code"), ListMasterDataItemRequest),
    responses(
        (status = 200, description = "Items retrieved successfully", body = ApiResponse<MasterDataItemListResponse>),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden")
    )
)]
pub async fn list_items(
    current_user: CurrentUser,
    State(state): State<AppState>,
    Path(group_code): Path<String>,
    Query(request): Query<ListMasterDataItemRequest>,
) -> Result<(StatusCode, Json<ApiResponse<MasterDataItemListResponse>>), AppError> {
    current_user.require(PermissionCode::MasterDataItemRead)?;

    let response = state
        .services
        .master_items
        .list_items(&group_code, request)
        .await
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success(
            response,
            "Items retrieved successfully",
        )),
    ))
}

#[utoipa::path(
    get,
    path = "/api/v1/master/items/{group_code}/options",
    tag = "Master Data Items",
    params(("group_code" = String, Path, description = "Group code"), MasterDataOptionsQuery),
    responses(
        (status = 200, description = "Options retrieved successfully", body = ApiResponse<MasterDataOptionListResponse>),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden")
    )
)]
pub async fn list_options(
    current_user: CurrentUser,
    State(state): State<AppState>,
    Path(group_code): Path<String>,
    Query(query): Query<MasterDataOptionsQuery>,
) -> Result<(StatusCode, Json<ApiResponse<MasterDataOptionListResponse>>), AppError> {
    current_user.require(PermissionCode::MasterDataItemRead)?;

    let response = state
        .services
        .master_items
        .list_options(
            &group_code,
            query.parent_id,
            query.only_root.unwrap_or(false),
        )
        .await
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success(
            response,
            "Options retrieved successfully",
        )),
    ))
}
