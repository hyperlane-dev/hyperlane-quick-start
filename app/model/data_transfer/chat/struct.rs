use super::*;

#[derive(Data, Default, Deserialize, Serialize, ToSchema)]
pub struct WebSocketRespData {
    r#type: MessageType,
    name: String,
    data: String,
    time: String,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct UserListResponse {
    users: Vec<OnlineUser>,
    total_count: usize,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct ChatHistoryResponse {
    pub messages: Vec<ChatHistory>,
    pub total: usize,
    pub has_more: bool,
}
