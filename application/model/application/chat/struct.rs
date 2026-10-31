use super::*;

/// A single message in a chat conversation, containing a role (e.g., "user", "assistant") and text content.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct ChatMessage {
    /// The role of the message sender (e.g., "user", "assistant", "system").
    #[set(type(AsRef<str>))]
    pub role: String,
    /// The textual content of the message.
    #[set(type(AsRef<str>))]
    pub content: String,
}

/// A chat session grouping related messages together with an activity tracker.
#[derive(Clone, Data, Debug)]
pub struct ChatSession {
    /// The unique session identifier.
    pub session_id: String,
    /// The ordered list of messages in this conversation.
    pub messages: Vec<ChatMessage>,
    /// The timestamp of the last message activity, used for session expiration.
    pub last_activity: Instant,
}

/// Represents an online user in the chat system with their join time.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct OnlineUser {
    /// The display name of the online user.
    pub username: String,
    /// The Unix timestamp (in milliseconds) when the user joined.
    #[get(type(copy))]
    pub join_time: i64,
}

/// A persisted chat history record containing sender and message metadata.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct ChatHistory {
    /// The unique identifier for the chat history record.
    #[get(type(copy))]
    pub id: i64,
    /// The session identifier grouping related messages together.
    pub session_id: String,
    /// The display name of the message sender.
    pub sender_name: String,
    /// The type of the sender (e.g., "user", "assistant").
    pub sender_type: String,
    /// The type of the message (e.g., "text", "image").
    pub message_type: String,
    /// The textual content of the chat message.
    pub content: String,
    /// The Unix timestamp (in milliseconds) when the message was created.
    #[get(type(copy))]
    pub created_at: i64,
}

/// A structured response from the GPT API containing generated text and a continuation flag.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct GptStructuredResponse {
    /// The generated text content from the GPT response.
    pub data: String,
    /// Flag indicating whether more content is expected to follow (streaming).
    #[get(type(copy))]
    pub continue_flag: bool,
}
