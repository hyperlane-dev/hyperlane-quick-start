use super::*;

#[derive(Debug, Clone, Default)]
pub struct EnvConfig {
    pub gpt_api_url: String,
    pub gpt_api_key: String,
    pub gtp_model: String,
    pub gtp_max_tokens: isize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone)]
pub struct ChatSession {
    pub session_id: String,
    pub messages: Vec<ChatMessage>,
    pub last_activity: std::time::Instant,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct OnlineUser {
    pub username: String,
    pub join_time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UserListResponse {
    pub users: Vec<OnlineUser>,
    pub total_count: usize,
}
