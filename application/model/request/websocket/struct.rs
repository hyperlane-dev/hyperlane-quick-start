use super::*;

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct WebSocketMessage {
    pub(super) name: String,
    pub(super) message: String,
}
