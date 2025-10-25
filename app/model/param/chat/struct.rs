use super::*;

#[derive(Data, Default, Serialize, Deserialize, ToSchema, Clone)]
pub struct WebSocketReqData {
    r#type: MessageType,
    data: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Data)]
pub struct ChatHistoryParams {
    pub session_id: String,
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}
