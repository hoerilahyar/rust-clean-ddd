use axum::{
    Router,
    routing::{delete, get, post, put},
};

use crate::{bootstrap::state::AppState, domain::master_data::items::handler};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/{group_code}/items", post(handler::create_item))
        .route("/{group_code}/items", get(handler::list_items))
        .route("/{group_code}/items/{id}", get(handler::find_item_by_id))
        .route("/{group_code}/items/{id}", put(handler::update_item))
        .route("/{group_code}/items/{id}", delete(handler::delete_item))
        .route("/{group_code}/options", get(handler::list_options))
}
