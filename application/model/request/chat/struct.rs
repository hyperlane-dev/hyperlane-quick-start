use super::*;

/// web socket req data.
#[derive(Clone, Data, Default, Deserialize, Serialize, ToSchema)]
pub struct WebSocketReqData {
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) r#type: MessageType,
    /// The data.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) data: String,
}

/// chat history params.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct ChatHistoryParams {
    /// The session id.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) session_id: String,
    #[get(pub(crate), type(copy))]
    /// The offset.
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) offset: Option<i64>,
    #[get(pub(crate), type(copy))]
    /// The limit.
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) limit: Option<i64>,
}
