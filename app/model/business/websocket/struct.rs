use super::*;

#[derive(Data)]
pub struct WebSocketRespData {
    r#type: String,
    data: String,
    online: bool,
    time: String,
}
