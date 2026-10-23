use super::*;

/// The envelope every Hyperlane API endpoint answers with.
#[derive(Clone, Debug, Deserialize)]
pub struct ApiResponse<T> {
    /// The business status code; `200` marks a successful call.
    pub code: i32,
    /// The human readable status message.
    pub message: String,
    /// The payload, present on a successful call.
    pub data: Option<T>,
    /// The server timestamp in milliseconds since the Unix epoch.
    pub timestamp: Option<i64>,
}

/// The failure a page call reports.
#[derive(Clone, Debug)]
pub struct ApiError {
    /// The business status code, or `0` when the request never reached the server.
    pub code: i32,
    /// The human readable failure description.
    pub message: String,
}
