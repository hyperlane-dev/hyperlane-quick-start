use super::*;

#[derive(Clone, Data, Default, Deserialize, Serialize, ToSchema)]
pub struct WebSocketReqData {
    #[get(type(copy))]
    pub(super) r#type: MessageType,
    pub(super) data: String,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct ChatHistoryParams {
    pub(super) session_id: String,
    pub(super) offset: Option<i64>,
    pub(super) limit: Option<i64>,
}
