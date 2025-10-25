use super::*;

#[derive(Debug, Default, Clone, Serialize, Deserialize, Data)]
pub struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Debug, Clone, Data)]
pub struct ChatSession {
    pub(super) session_id: String,
    pub(super) messages: Vec<ChatMessage>,
    pub(super) last_activity: Instant,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, ToSchema, Data)]
pub struct OnlineUser {
    username: String,
    join_time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Data)]
pub struct ChatHistory {
    pub id: i64,
    pub session_id: String,
    pub sender_name: String,
    pub sender_type: String,
    pub message_type: String,
    pub content: String,
    pub created_at: String,
}
