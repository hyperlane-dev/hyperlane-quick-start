use super::*;

#[derive(Data, Default, Serialize, ToSchema)]
pub struct WebSocketRespData {
    r#type: MessageType,
    name: String,
    data: String,
    time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UserListResponse {
    pub users: Vec<OnlineUser>,
    pub total_count: usize,
}
