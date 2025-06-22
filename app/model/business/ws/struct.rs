use super::*;

#[derive(Data, Default, Serialize, ToSchema)]
pub struct WebSocketRespData {
    r#type: MessageType,
    name: String,
    data: String,
    time: String,
}

#[derive(Data, Default, Serialize, Deserialize, ToSchema, Clone)]
pub struct WebSocketReqData {
    r#type: MessageType,
    data: String,
}

#[derive(Debug, Clone, Default)]
pub struct EnvConfig {
    pub gpt_api_url: String,
    pub gpt_api_key: String,
}
