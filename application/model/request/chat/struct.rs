use super::*;

/// web socket req data.
#[derive(Clone, Data, Default, Deserialize, Serialize, ToSchema)]
pub struct WebSocketReqData {
    #[get(type(copy))]
    pub r#type: MessageType,
    /// The data.
    pub data: String,
}

/// chat history params.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct ChatHistoryParams {
    /// The session id.
    pub session_id: String,
    #[get(type(copy))]
    /// The offset.
    pub offset: Option<i64>,
    #[get(type(copy))]
    /// The limit.
    pub limit: Option<i64>,
}
