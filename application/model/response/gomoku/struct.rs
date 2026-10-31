use super::*;

/// Represents a Gomoku WebSocket response with message type, room, and payload.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct GomokuWsResponse {
    pub r#type: GomokuMessageType,
    /// The room id.
    pub room_id: String,
    /// The sender id.
    pub sender_id: String,
    /// The payload.
    pub payload: Value,
    /// The time.
    #[get(type(copy))]
    pub time: i64,
}
