use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
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
    Unknown,
}
