use super::*;

#[derive(Data, Default, Serialize)]
pub struct WebSocketRespData {
    r#type: MessageType,
    id: String,
    name: String,
    data: String,
    time: String,
}

#[derive(Data, Default, Serialize, Deserialize)]
pub struct WebSocketReqData {
    r#type: MessageType,
    data: String,
}
