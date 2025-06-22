use super::*;

#[derive(Data, Default, Serialize, ToSchema)]
pub struct WebSocketRespData {
    r#type: MessageType,
    name: String,
    data: String,
    time: String,
}

#[derive(Data, Default, Serialize, Deserialize, ToSchema, Clone)]
pub struct WebSocketReqData {
    r#type: MessageType,
    data: String,
}

#[derive(Debug, Clone, Default)]
pub struct EnvConfig {
    pub gpt_api_url: String,
    pub gpt_api_key: String,
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

impl ChatSession {
    pub fn new(session_id: String) -> Self {
        Self {
            session_id,
            messages: Vec::new(),
            last_activity: std::time::Instant::now(),
        }
    }

    pub fn add_message(&mut self, role: String, content: String) {
        self.messages.push(ChatMessage { role, content });
        self.last_activity = std::time::Instant::now();

        if self.messages.len() > 20 {
            self.messages.drain(0..self.messages.len() - 20);
        }
    }

    pub fn is_expired(&self, timeout_minutes: u64) -> bool {
        self.last_activity.elapsed().as_secs() > timeout_minutes * 60
    }
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
