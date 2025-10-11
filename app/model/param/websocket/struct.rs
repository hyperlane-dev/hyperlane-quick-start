use super::*;

#[derive(Debug, Clone, Default, Data, Deserialize, Serialize)]
pub struct WebSocketMessage {
    pub name: String,
    pub message: String,
}
