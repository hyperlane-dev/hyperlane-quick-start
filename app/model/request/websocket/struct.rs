use super::*;

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct WebSocketMessage {
    #[get(pub)]
    pub(super) name: String,
    #[get(pub)]
    pub(super) message: String,
}
