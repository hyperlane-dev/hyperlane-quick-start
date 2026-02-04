use super::*;

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct WebSocketRespData {
    #[get(pub)]
    pub(super) r#type: MessageType,
    #[get(pub)]
    pub(super) name: String,
    #[get(pub)]
    pub(super) data: String,
    #[get(pub)]
    pub(super) time: String,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct UserListResponse {
    #[get(pub)]
    pub(super) users: Vec<OnlineUser>,
    #[get(type(copy), pub)]
    pub(super) total_count: usize,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct ChatHistoryResponse {
    #[get(pub)]
    pub(super) messages: Vec<ChatHistory>,
    #[get(type(copy), pub)]
    pub(super) total: usize,
    #[get(type(copy), pub)]
    pub(super) has_more: bool,
}
