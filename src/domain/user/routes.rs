use axum::{
    Router,
    routing::{delete, get, post, put},
};

use crate::{bootstrap::state::AppState, domain::user::handler};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", post(handler::create))
        .route("/", get(handler::list))
        .route("/{id}", get(handler::find_by_id))
        .route("/{id}", put(handler::update))
        .route("/{id}", delete(handler::delete))
}
