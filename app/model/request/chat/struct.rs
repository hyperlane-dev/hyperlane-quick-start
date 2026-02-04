use super::*;

#[derive(Clone, Data, Default, Deserialize, Serialize, ToSchema)]
pub struct WebSocketReqData {
    #[get(pub(crate))]
    pub(super) r#type: MessageType,
    #[get(pub(crate))]
    pub(super) data: String,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct ChatHistoryParams {
    #[get(pub(crate))]
    pub(super) session_id: String,
    #[get(pub(crate))]
    pub(super) offset: Option<i64>,
    #[get(pub(crate))]
    pub(super) limit: Option<i64>,
}
