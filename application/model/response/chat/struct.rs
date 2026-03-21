use super::*;

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct WebSocketRespData {
    #[get(type(copy))]
    pub(super) r#type: MessageType,
    pub(super) name: String,
    pub(super) data: String,
    pub(super) time: i64,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct UserListResponse {
    pub(super) users: Vec<OnlineUser>,
    #[get(type(copy))]
    pub(super) total_count: usize,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct ChatHistoryResponse {
    pub(super) messages: Vec<ChatHistory>,
    #[get(type(copy))]
    pub(super) total: usize,
    #[get(type(copy))]
    pub(super) has_more: bool,
}
