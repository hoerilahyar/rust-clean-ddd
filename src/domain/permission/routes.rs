use axum::{
    Router, middleware,
    routing::{delete, get, post, put},
};

use crate::{
    bootstrap::state::AppState,
    domain::permission::{entity::PermissionCode, handler},
    middleware::{permission, permission_layer},
};

pub fn router(state: AppState) -> Router<AppState> {
    Router::new()
        .route(
            "/",
            post(handler::create).route_layer(middleware::from_fn(permission::require(
                PermissionCode::PermissionCreate,
            ))),
        )
        .route(
            "/",
            get(handler::list).route_layer(middleware::from_fn(permission::require(
                PermissionCode::PermissionCreate,
            ))),
        )
        .route(
            "/:id",
            get(handler::find_by_id).route_layer(middleware::from_fn(permission::require(
                PermissionCode::PermissionCreate,
            ))),
        )
        .route(
            "/:id",
            put(handler::update).route_layer(middleware::from_fn(permission::require(
                PermissionCode::PermissionCreate,
            ))),
        )
        .route(
            "/:id",
            delete(handler::delete).route_layer(middleware::from_fn(permission::require(
                PermissionCode::PermissionDelete,
            ))),
        )
}
