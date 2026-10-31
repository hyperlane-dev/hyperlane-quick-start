use super::*;

/// Represents a WebSocket message response with content and timestamp.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct MessageResponse {
    /// The message.
    pub message: String,
    /// The time.
    #[get(type(copy))]
    pub time: i64,
}
