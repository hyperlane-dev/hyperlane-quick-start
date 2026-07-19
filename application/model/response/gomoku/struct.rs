use super::*;

/// Represents a Gomoku WebSocket response with message type, room, and payload.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct GomokuWsResponse {
    pub(super) r#type: GomokuMessageType,
    /// The room id.
    pub(super) room_id: String,
    /// The sender id.
    pub(super) sender_id: String,
    /// The payload.
    pub(super) payload: serde_json::Value,
    /// The time.
    #[get(type(copy))]
    pub(super) time: i64,
}
