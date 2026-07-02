use std::time::Duration;

use axum::http::StatusCode;
use tower_http::timeout::TimeoutLayer;

pub fn layer() -> TimeoutLayer {
    TimeoutLayer::with_status_code(StatusCode::REQUEST_TIMEOUT, Duration::from_secs(30))
}
