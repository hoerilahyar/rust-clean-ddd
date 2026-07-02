use axum::{
    Router,
    routing::{delete, get, put},
};

use crate::{bootstrap::state::AppState, domain::user_role::handler};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/:user_id/roles", put(handler::assign))
        .route("/:user_id/roles", get(handler::list))
        .route("/:user_id/roles/:role_id", delete(handler::revoke))
}
