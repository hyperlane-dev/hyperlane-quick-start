use super::*;

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct ChatMessage {
    #[get(pub)]
    pub(super) role: String,
    #[get(pub)]
    pub(super) content: String,
}

#[derive(Clone, Data, Debug)]
pub struct ChatSession {
    #[get(pub)]
    pub(super) session_id: String,
    #[get(pub)]
    pub(super) messages: Vec<ChatMessage>,
    #[get(pub)]
    pub(super) last_activity: Instant,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct OnlineUser {
    #[get(pub)]
    pub(super) username: String,
    #[get(pub)]
    pub(super) join_time: String,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct ChatHistory {
    #[get(type(copy), pub)]
    pub(super) id: i64,
    #[get(pub)]
    pub(super) session_id: String,
    #[get(pub)]
    pub(super) sender_name: String,
    #[get(pub)]
    pub(super) sender_type: String,
    #[get(pub)]
    pub(super) message_type: String,
    #[get(pub)]
    pub(super) content: String,
    #[get(pub)]
    pub(super) created_at: String,
}
