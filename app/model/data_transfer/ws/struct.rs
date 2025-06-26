use super::*;

#[derive(Data, Default, Serialize, ToSchema)]
pub struct WebSocketRespData {
    r#type: MessageType,
    name: String,
    data: String,
    time: String,
}
