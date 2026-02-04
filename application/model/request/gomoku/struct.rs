use super::*;

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct GomokuWsRequest {
    #[get(pub)]
    pub(super) r#type: GomokuMessageType,
    #[get(pub)]
    pub(super) room_id: String,
    #[get(pub)]
    pub(super) payload: serde_json::Value,
}
