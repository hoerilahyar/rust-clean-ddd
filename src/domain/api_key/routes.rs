use axum::{
    Router,
    routing::{delete, get, patch, post, put},
};

use crate::{bootstrap::state::AppState, domain::api_key::handler};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", post(handler::create))
        .route("/", get(handler::list))
        .route("/{id}", get(handler::find_by_id))
        .route("/{id}", put(handler::update))
        .route("/{id}", delete(handler::delete))
        .route("/{id}/active", patch(handler::set_active))
}
