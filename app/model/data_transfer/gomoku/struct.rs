use super::*;

#[derive(Clone, Data, Default, Deserialize, Serialize, ToSchema)]
pub struct GomokuWsResponse {
    r#type: GomokuMessageType,
    room_id: String,
    sender_id: String,
    payload: serde_json::Value,
    time: String,
}
