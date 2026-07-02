use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ValidationError {
    pub field: String,

    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub success: bool,

    pub message: String,

    pub errors: Vec<ValidationError>,
}
