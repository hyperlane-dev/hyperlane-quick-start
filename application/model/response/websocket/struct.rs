use super::*;

/// Represents a WebSocket message response with content and timestamp.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct MessageResponse {
    /// The message.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) message: String,
    /// The time.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) time: i64,
}
