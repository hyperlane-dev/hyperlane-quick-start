use super::*;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema)]
pub enum MessageType {
    OnlineCount,
    Image,
    Text,
    File,
    Markdown,
    Audio,
    Video,
    Location,
    Custom,
    Ping,
    Pang,
    GptResponse,
    Unknown,
}
