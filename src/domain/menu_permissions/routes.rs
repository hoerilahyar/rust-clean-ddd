use axum::{
    Router,
    routing::{delete, get, post},
};

use crate::{bootstrap::state::AppState, domain::menu_permissions::handler};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/{menu_id}/permissions", post(handler::assign))
        .route("/{menu_id}/permissions", get(handler::list))
        .route(
            "/{menu_id}/permissions/{permission_id}",
            delete(handler::revoke),
        )
}
