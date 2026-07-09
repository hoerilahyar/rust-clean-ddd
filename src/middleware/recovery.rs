use axum::{extract::Request, middleware::Next, response::Response};

pub async fn handle_panics(request: Request, next: Next) -> Response {
    next.run(request).await
}
