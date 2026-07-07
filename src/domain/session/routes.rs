use axum::{
    Router,
    routing::{delete, get, post},
};

use crate::{bootstrap::state::AppState, domain::session::handler};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(handler::list))
        .route("/{id}", delete(handler::revoke))
        .route("/revoke-others", post(handler::revoke_others))
}
