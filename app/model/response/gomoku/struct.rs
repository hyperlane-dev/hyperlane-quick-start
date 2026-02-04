use super::*;

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct GomokuWsResponse {
    #[get(pub(crate))]
    pub(super) r#type: GomokuMessageType,
    #[get(pub(crate))]
    pub(super) room_id: String,
    #[get(pub(crate))]
    pub(super) sender_id: String,
    #[get(pub(crate))]
    pub(super) payload: serde_json::Value,
    #[get(pub(crate))]
    pub(super) time: String,
}
