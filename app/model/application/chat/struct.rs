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
