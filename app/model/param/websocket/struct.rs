use super::*;

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct WebSocketMessage {
    name: String,
    message: String,
}
