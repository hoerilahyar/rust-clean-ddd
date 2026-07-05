use crate::swagger::ApiDoc;
use axum::{Router, middleware};
use tower::ServiceBuilder;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::bootstrap::state::AppState;
use crate::middleware::auth::authenticate;
use crate::middleware::{compression, cors, request_id, timeout, trace};

pub fn create_router(state: AppState) -> Router {
    let (set_request_id, propagate_request_id) = request_id::layers();

    let middleware_stack = ServiceBuilder::new()
        .layer(set_request_id)
        .layer(trace::layer())
        .layer(cors::layer())
        .layer(compression::layer())
        .layer(timeout::layer())
        .layer(propagate_request_id);

    let protected = crate::routes::api::protected_routes()
        .layer(middleware::from_fn_with_state(state.clone(), authenticate));

    let public = crate::routes::api::public_routes();

    Router::new()
        .nest("/api/v1", public.merge(protected))
        .merge(SwaggerUi::new("/").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .layer(middleware_stack)
        .with_state(state)
}
