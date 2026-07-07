use axum::{
    Router,
    routing::{delete, get, patch, put},
};

use crate::{bootstrap::state::AppState, domain::system_settings::handler};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", put(handler::upsert))
        .route("/", get(handler::list))
        .route("/{key}", get(handler::find_by_key))
        .route("/{key}", delete(handler::delete))
        .route("/{key}/active", patch(handler::set_active))
}
