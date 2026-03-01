use super::*;

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct GomokuWsRequest {
    pub(super) r#type: GomokuMessageType,
    pub(super) room_id: String,
    pub(super) payload: serde_json::Value,
}
