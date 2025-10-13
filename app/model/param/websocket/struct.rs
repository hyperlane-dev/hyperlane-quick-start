use super::*;

#[derive(Debug, Clone, Default, Data, Deserialize, Serialize)]
pub struct WebSocketMessage {
    name: String,
    message: String,
}
