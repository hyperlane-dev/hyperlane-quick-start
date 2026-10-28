use super::*;

/// A single message in a chat conversation, containing a role (e.g., "user", "assistant") and text content.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct ChatMessage {
    /// The role of the message sender (e.g., "user", "assistant", "system").
    #[set(pub(crate), type(AsRef<str>))]
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    pub(crate) role: String,
    /// The textual content of the message.
    #[set(pub(crate), type(AsRef<str>))]
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    pub(crate) content: String,
}

/// A chat session grouping related messages together with an activity tracker.
#[derive(Clone, Data, Debug)]
pub struct ChatSession {
    /// The unique session identifier.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) session_id: String,
    /// The ordered list of messages in this conversation.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) messages: Vec<ChatMessage>,
    /// The timestamp of the last message activity, used for session expiration.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) last_activity: Instant,
}

/// Represents an online user in the chat system with their join time.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct OnlineUser {
    /// The display name of the online user.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) username: String,
    /// The Unix timestamp (in milliseconds) when the user joined.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) join_time: i64,
}

/// A persisted chat history record containing sender and message metadata.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct ChatHistory {
    /// The unique identifier for the chat history record.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) id: i64,
    /// The session identifier grouping related messages together.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) session_id: String,
    /// The display name of the message sender.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) sender_name: String,
    /// The type of the sender (e.g., "user", "assistant").
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) sender_type: String,
    /// The type of the message (e.g., "text", "image").
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) message_type: String,
    /// The textual content of the chat message.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) content: String,
    /// The Unix timestamp (in milliseconds) when the message was created.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) created_at: i64,
}

/// A structured response from the GPT API containing generated text and a continuation flag.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct GptStructuredResponse {
    /// The generated text content from the GPT response.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) data: String,
    /// Flag indicating whether more content is expected to follow (streaming).
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) continue_flag: bool,
}
