use super::*;

#[derive(Data, Default, Serialize, Deserialize, ToSchema, Clone)]
pub struct WebSocketReqData {
    r#type: MessageType,
    data: String,
}
