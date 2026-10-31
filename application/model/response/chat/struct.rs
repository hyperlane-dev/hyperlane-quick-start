use super::*;

/// web socket resp data.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct WebSocketRespData {
    #[get(type(copy))]
    pub r#type: MessageType,
    /// The name.
    pub name: String,
    /// The data.
    pub data: String,
    /// The time.
    #[get(type(copy))]
    pub time: i64,
}

/// Represents a chat user list response with online user information.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct UserListResponse {
    /// The users.
    pub users: Vec<OnlineUser>,
    /// The total count.
    #[get(type(copy))]
    pub total_count: usize,
}

/// chat history response.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct ChatHistoryResponse {
    /// The messages.
    pub messages: Vec<ChatHistory>,
    /// The total.
    #[get(type(copy))]
    pub total: usize,
    /// The has more.
    #[get(type(copy))]
    pub has_more: bool,
}
