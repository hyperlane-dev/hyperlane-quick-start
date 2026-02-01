use super::*;

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Clone, Data, Debug)]
pub struct ChatSession {
    pub(super) session_id: String,
    pub(super) messages: Vec<ChatMessage>,
    pub(super) last_activity: Instant,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct OnlineUser {
    username: String,
    join_time: String,
}

#[derive(Clone, Data, Debug, Deserialize, Serialize, ToSchema)]
pub struct ChatHistory {
    pub id: i64,
    pub session_id: String,
    pub sender_name: String,
    pub sender_type: String,
    pub message_type: String,
    pub content: String,
    pub created_at: String,
}
