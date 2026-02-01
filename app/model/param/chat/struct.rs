use super::*;

#[derive(Clone, Data, Default, Deserialize, Serialize, ToSchema)]
pub struct WebSocketReqData {
    r#type: MessageType,
    data: String,
}

#[derive(Clone, Data, Debug, Deserialize, Serialize, ToSchema)]
pub struct ChatHistoryParams {
    pub session_id: String,
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}
