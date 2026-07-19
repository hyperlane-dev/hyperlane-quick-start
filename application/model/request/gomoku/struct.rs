use super::*;

/// gomoku ws request.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct GomokuWsRequest {
    pub(super) r#type: GomokuMessageType,
    /// The room id.
    pub(super) room_id: String,
    /// The payload.
    pub(super) payload: serde_json::Value,
}
