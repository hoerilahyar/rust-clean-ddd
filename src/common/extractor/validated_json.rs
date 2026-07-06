use axum::{
    Json,
    body::to_bytes,
    extract::{FromRequest, Request},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Serialize, de::DeserializeOwned};
use serde_json::Value;
use std::collections::HashMap;
use validator::Validate;

pub struct ValidatedJson<T>(pub T);

pub trait RequiredFields {
    fn required_fields() -> &'static [&'static str] {
        &[]
    }
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    errors: HashMap<String, Vec<String>>,
}

impl<S, T> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate + RequiredFields,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request(req: Request, _state: &S) -> Result<Self, Self::Rejection> {
        let bytes = to_bytes(req.into_body(), usize::MAX)
            .await
            .map_err(|_| bad_request("Unable to read request body"))?;

        let value: Value = serde_json::from_slice(&bytes)
            .map_err(|e| bad_request(&format!("Invalid JSON: {e}")))?;

        let mut errors: HashMap<String, Vec<String>> = HashMap::new();

        if let Value::Object(map) = &value {
            for field in T::required_fields() {
                let missing = matches!(map.get(*field), None | Some(Value::Null));
                if missing {
                    errors
                        .entry(field.to_string())
                        .or_default()
                        .push(format!("The {} field is required.", humanize(field)));
                }
            }
        }

        if !errors.is_empty() {
            return Err(validation_response(errors));
        }

        let data: T = serde_json::from_value(value)
            .map_err(|e| bad_request(&format!("Invalid request body: {e}")))?;

        if let Err(validation_errors) = data.validate() {
            for (field, field_errors) in validation_errors.field_errors() {
                let messages = field_errors
                    .iter()
                    .map(|err| format_message(field.as_ref(), err))
                    .collect();
                errors.insert(field.to_string(), messages);
            }
            return Err(validation_response(errors));
        }

        Ok(ValidatedJson(data))
    }
}

fn humanize(field: &str) -> String {
    field.replace('_', " ")
}

fn format_message(field: &str, err: &validator::ValidationError) -> String {
    let name = humanize(field);
    match err.code.as_ref() {
        "length" => {
            let min = err.params.get("min").and_then(|v| v.as_u64());
            let max = err.params.get("max").and_then(|v| v.as_u64());
            match (min, max) {
                (Some(min), Some(max)) => {
                    format!("The {name} field must be between {min} and {max} characters.")
                }
                (Some(min), None) => format!("The {name} field must be at least {min} characters."),
                (None, Some(max)) => format!("The {name} field must not exceed {max} characters."),
                _ => format!("The {name} field is invalid."),
            }
        }
        _ => err
            .message
            .as_ref()
            .map(|m| m.to_string())
            .unwrap_or_else(|| format!("The {name} field is invalid.")),
    }
}

fn validation_response(errors: HashMap<String, Vec<String>>) -> Response {
    (
        StatusCode::UNPROCESSABLE_ENTITY,
        Json(ErrorResponse { errors }),
    )
        .into_response()
}

fn bad_request(message: &str) -> Response {
    let mut errors = HashMap::new();
    errors.insert("body".to_string(), vec![message.to_string()]);
    (StatusCode::BAD_REQUEST, Json(ErrorResponse { errors })).into_response()
}
