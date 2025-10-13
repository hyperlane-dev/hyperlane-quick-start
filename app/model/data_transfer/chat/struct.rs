use super::*;

#[derive(Data, Default, Serialize, ToSchema)]
pub struct WebSocketRespData {
    r#type: MessageType,
    name: String,
    data: String,
    time: String,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, ToSchema, Data)]
pub struct UserListResponse {
    users: Vec<OnlineUser>,
    total_count: usize,
}
