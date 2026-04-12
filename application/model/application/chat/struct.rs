use super::*;

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct ChatMessage {
    #[set(type(AsRef<str>))]
    pub(super) role: String,
    #[set(type(AsRef<str>))]
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
    pub(super) join_time: i64,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct ChatHistory {
    #[get(type(copy))]
    pub(super) id: i64,
    pub(super) session_id: String,
    pub(super) sender_name: String,
    pub(super) sender_type: String,
    pub(super) message_type: String,
    pub(super) content: String,
    pub(super) created_at: i64,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct GptStructuredResponse {
    pub(super) data: String,
    #[get(type(copy))]
    pub(super) continue_flag: bool,
}
