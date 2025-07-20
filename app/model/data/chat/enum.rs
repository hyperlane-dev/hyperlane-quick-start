use super::*;

/// Represents different types of messages in the chat system
///
/// This enum defines all possible message types that can be sent
/// through the WebSocket chat interface
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub enum MessageType {
    /// Online user count message
    OnlineCount,
    /// Image message
    Image,
    /// Plain text message
    Text,
    /// File attachment message
    File,
    /// Markdown formatted message
    Markdown,
    /// Audio message
    Audio,
    /// Video message
    Video,
    /// Location sharing message
    Location,
    /// Custom message type
    Custom,
    /// Ping message for connection testing
    Ping,
    /// Pong response to ping
    Pang,
    /// GPT assistant response message
    GptResponse,
    /// Unknown message type
    Unknown,
}
