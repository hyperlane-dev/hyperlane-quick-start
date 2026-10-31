use super::*;

/// Represents a WebSocket message with sender name and content.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct WebSocketMessage {
    /// The name.
    pub name: String,
    /// The message.
    pub message: String,
}
