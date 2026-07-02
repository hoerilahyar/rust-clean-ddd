use axum::{
    Router,
    routing::{delete, get, post, put},
};

use crate::{
    bootstrap::state::AppState,
    domain::{role::handler, role_permission},
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", post(handler::create))
        .route("/", get(handler::list))
        .route("/:id", get(handler::find_by_id))
        .route("/:id", put(handler::update))
        .route("/:id", delete(handler::delete))
        // Assignment Permission
        .route(
            "/:role_id/permissions",
            put(role_permission::handler::assign),
        )
        .route("/:role_id/permissions", get(role_permission::handler::list))
        .route(
            "/:role_id/permissions/:permission_id",
            delete(role_permission::handler::revoke),
        )
}
