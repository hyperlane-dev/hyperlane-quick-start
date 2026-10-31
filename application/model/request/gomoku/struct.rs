use super::*;

/// gomoku ws request.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct GomokuWsRequest {
    pub r#type: GomokuMessageType,
    /// The room id.
    pub room_id: String,
    /// The payload.
    pub payload: Value,
}
