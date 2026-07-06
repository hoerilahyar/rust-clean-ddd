use axum::{
    Router,
    routing::{delete, get, post},
};

use crate::{bootstrap::state::AppState, domain::role_permission::handler};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/{role_id}/permissions", post(handler::assign))
        .route("/{role_id}/permissions", get(handler::list))
        .route(
            "/{role_id}/permissions/{permission_id}",
            delete(handler::revoke),
        )
}
