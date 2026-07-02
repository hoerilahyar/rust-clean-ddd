use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use crate::common::error::error_response::ErrorResponse;

#[derive(Debug)]
pub enum AppError {
    BadRequest(String),

    Unauthorized(String),

    Forbidden(String),

    NotFound(String),

    Conflict(String),

    Validation(Vec<(String, String)>),

    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::BadRequest(message) => (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    success: false,

                    message,

                    errors: vec![],
                }),
            )
                .into_response(),

            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    success: false,

                    message: "Internal Server Error".to_string(),

                    errors: vec![],
                }),
            )
                .into_response(),
        }
    }
}
