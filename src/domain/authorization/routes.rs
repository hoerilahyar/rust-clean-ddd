use axum::{Router, routing::get};

use crate::{bootstrap::state::AppState, domain::authorization::handler};

pub fn router() -> Router<AppState> {
    Router::new().route("/me", get(handler::me))
}
