use super::*;

#[derive(Data, Default, Serialize, ToSchema)]
pub struct WebSocketRespData {
    r#type: MessageType,
    name: String,
    data: String,
    time: String,
}

#[derive(Data, Default, Serialize, Deserialize, ToSchema)]
pub struct WebSocketReqData {
    r#type: MessageType,
    data: String,
}
