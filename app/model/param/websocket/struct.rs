use super::*;

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct WebSocketMessage {
    name: String,
    message: String,
}
