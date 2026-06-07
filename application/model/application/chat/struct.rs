use super::*;

/// A single message in a chat conversation, containing a role (e.g., "user", "assistant") and text content.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct ChatMessage {
    /// The role of the message sender (e.g., "user", "assistant", "system").
    #[set(type(AsRef<str>))]
    pub(super) role: String,
    /// The textual content of the message.
    #[set(type(AsRef<str>))]
    pub(super) content: String,
}

/// A chat session grouping related messages together with an activity tracker.
#[derive(Clone, Data, Debug)]
pub struct ChatSession {
    /// The unique session identifier.
    pub(super) session_id: String,
    /// The ordered list of messages in this conversation.
    pub(super) messages: Vec<ChatMessage>,
    /// The timestamp of the last message activity, used for session expiration.
    pub(super) last_activity: Instant,
}

/// Represents an online user in the chat system with their join time.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct OnlineUser {
    /// The display name of the online user.
    pub(super) username: String,
    /// The Unix timestamp (in milliseconds) when the user joined.
    pub(super) join_time: i64,
}

/// A persisted chat history record containing sender and message metadata.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct ChatHistory {
    /// The unique identifier for the chat history record.
    #[get(type(copy))]
    pub(super) id: i64,
    /// The session identifier grouping related messages together.
    pub(super) session_id: String,
    /// The display name of the message sender.
    pub(super) sender_name: String,
    /// The type of the sender (e.g., "user", "assistant").
    pub(super) sender_type: String,
    /// The type of the message (e.g., "text", "image").
    pub(super) message_type: String,
    /// The textual content of the chat message.
    pub(super) content: String,
    /// The Unix timestamp (in milliseconds) when the message was created.
    pub(super) created_at: i64,
}

/// A structured response from the GPT API containing generated text and a continuation flag.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct GptStructuredResponse {
    /// The generated text content from the GPT response.
    pub(super) data: String,
    /// Flag indicating whether more content is expected to follow (streaming).
    #[get(type(copy))]
    pub(super) continue_flag: bool,
}
