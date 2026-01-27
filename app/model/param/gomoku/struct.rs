use super::*;

#[derive(Data, Default, Serialize, Deserialize, ToSchema, Clone)]
pub struct GomokuWsRequest {
    r#type: GomokuMessageType,
    room_id: String,
    payload: serde_json::Value,
}
