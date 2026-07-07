use axum::{
    Router,
    routing::{delete, get, patch, put},
};

use crate::{bootstrap::state::AppState, domain::user_setting::handler};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", put(handler::upsert))
        .route("/", get(handler::list))
        .route("/{key}", get(handler::find_by_key))
        .route("/{key}", delete(handler::delete))
        .route("/{key}/active", patch(handler::set_active))
}
