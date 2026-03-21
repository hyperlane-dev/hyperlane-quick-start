use super::*;

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct GomokuWsResponse {
    pub(super) r#type: GomokuMessageType,
    pub(super) room_id: String,
    pub(super) sender_id: String,
    pub(super) payload: serde_json::Value,
    #[get(type(copy))]
    pub(super) time: i64,
}
