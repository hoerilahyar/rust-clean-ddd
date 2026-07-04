use crate::swagger::ApiDoc;
use axum::{Router, middleware};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::bootstrap::state::AppState;
use crate::middleware::auth::authenticate;

pub fn create_router(state: AppState) -> Router {
    let protected = crate::routes::api::protected_routes()
        .layer(middleware::from_fn_with_state(state.clone(), authenticate));

    let public = crate::routes::api::public_routes();

    Router::new()
        .nest("/api/v1", public.merge(protected))
        .merge(SwaggerUi::new("/").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .with_state(state)
}
