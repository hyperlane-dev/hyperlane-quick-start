use super::*;

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct ChatMessage {
    pub(super) role: String,
    pub(super) content: String,
}

#[derive(Clone, Data, Debug)]
pub struct ChatSession {
    pub(super) session_id: String,
    pub(super) messages: Vec<ChatMessage>,
    pub(super) last_activity: Instant,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct OnlineUser {
    pub(super) username: String,
    pub(super) join_time: String,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct ChatHistory {
    #[get(type(copy), pub)]
    pub(super) id: i64,
    pub(super) session_id: String,
    pub(super) sender_name: String,
    pub(super) sender_type: String,
    pub(super) message_type: String,
    pub(super) content: String,
    pub(super) created_at: String,
}
