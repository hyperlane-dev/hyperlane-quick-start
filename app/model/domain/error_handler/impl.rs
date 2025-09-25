use super::*;
use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub code: String,
    pub timestamp: String,
}

impl ErrorHandler {
    pub fn new() -> Self {
        Self
    }

    pub fn handle_error(error: &str, code: &str) -> ErrorResponse {
        ErrorResponse {
            error: error.to_string(),
            code: code.to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }
}
