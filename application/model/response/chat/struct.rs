use super::*;

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct WebSocketRespData {
    pub(super) r#type: MessageType,
    pub(super) name: String,
    pub(super) data: String,
    pub(super) time: String,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct UserListResponse {
    pub(super) users: Vec<OnlineUser>,
    #[get(type(copy), pub)]
    pub(super) total_count: usize,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct ChatHistoryResponse {
    pub(super) messages: Vec<ChatHistory>,
    #[get(type(copy), pub)]
    pub(super) total: usize,
    #[get(type(copy), pub)]
    pub(super) has_more: bool,
}
