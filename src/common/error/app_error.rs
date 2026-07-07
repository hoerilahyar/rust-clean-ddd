use std::fmt;

use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use tracing::error;

use crate::common::error::error_response::{ErrorResponse, ValidationError};

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
        let (status, message, errors) = match self {
            AppError::BadRequest(message) => (StatusCode::BAD_REQUEST, message, vec![]),

            AppError::Unauthorized(message) => (StatusCode::UNAUTHORIZED, message, vec![]),

            AppError::Forbidden(message) => (StatusCode::FORBIDDEN, message, vec![]),

            AppError::NotFound(message) => (StatusCode::NOT_FOUND, message, vec![]),

            AppError::Conflict(message) => (StatusCode::CONFLICT, message, vec![]),

            AppError::Validation(fields) => {
                let errors = fields
                    .into_iter()
                    .map(|(field, message)| ValidationError { field, message })
                    .collect();

                (
                    StatusCode::UNPROCESSABLE_ENTITY,
                    "Validation error".to_string(),
                    errors,
                )
            }

            AppError::Internal(message) => {
                // Log pesan error asli agar terlihat di terminal/log,
                // tapi jangan bocorkan detailnya ke response client.
                error!("Internal server error: {message}");

                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal Server Error".to_string(),
                    vec![],
                )
            }
        };

        (
            status,
            Json(ErrorResponse {
                success: false,
                message,
                errors,
            }),
        )
            .into_response()
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => AppError::NotFound("Data not found".to_string()),
            other => AppError::Internal(other.to_string()),
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::BadRequest(msg) => write!(f, "{msg}"),
            AppError::Unauthorized(msg) => write!(f, "{msg}"),
            AppError::Forbidden(msg) => write!(f, "{msg}"),
            AppError::NotFound(msg) => write!(f, "{msg}"),
            AppError::Conflict(msg) => write!(f, "{msg}"),
            AppError::Validation(fields) => write!(f, "Validation error: {fields:?}"),
            AppError::Internal(msg) => write!(f, "{msg}"),
        }
    }
}

impl std::error::Error for AppError {}
