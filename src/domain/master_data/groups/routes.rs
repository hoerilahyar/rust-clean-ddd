use axum::{
    Router,
    routing::{delete, get, post, put},
};

use crate::{bootstrap::state::AppState, domain::master_data::groups::handler};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", post(handler::create_group))
        .route("/", get(handler::list_groups))
        .route("/{code}", get(handler::find_group_by_code))
        .route("/{code}", put(handler::update_group))
        .route("/{code}", delete(handler::delete_group))
}
