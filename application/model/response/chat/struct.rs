use super::*;

/// web socket resp data.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct WebSocketRespData {
    #[get(type(copy))]
    pub(super) r#type: MessageType,
    /// The name.
    pub(super) name: String,
    /// The data.
    pub(super) data: String,
    /// The time.
    pub(super) time: i64,
}

/// Represents a chat user list response with online user information.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct UserListResponse {
    /// The users.
    pub(super) users: Vec<OnlineUser>,
    /// The total count.
    #[get(type(copy))]
    pub(super) total_count: usize,
}

/// chat history response.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct ChatHistoryResponse {
    /// The messages.
    pub(super) messages: Vec<ChatHistory>,
    /// The total.
    #[get(type(copy))]
    pub(super) total: usize,
    /// The has more.
    #[get(type(copy))]
    pub(super) has_more: bool,
}
