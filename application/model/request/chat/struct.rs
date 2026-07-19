use super::*;

/// web socket req data.
#[derive(Clone, Data, Default, Deserialize, Serialize, ToSchema)]
pub struct WebSocketReqData {
    #[get(type(copy))]
    pub(super) r#type: MessageType,
    /// The data.
    pub(super) data: String,
}

/// chat history params.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct ChatHistoryParams {
    /// The session id.
    pub(super) session_id: String,
    /// The offset.
    pub(super) offset: Option<i64>,
    /// The limit.
    pub(super) limit: Option<i64>,
}
