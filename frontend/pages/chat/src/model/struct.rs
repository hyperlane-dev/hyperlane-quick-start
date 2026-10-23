use super::*;

/// One frame of the chat room.
///
/// The room sends the frames of the messages, of the system notices and of
/// the online count; every frame carries the same four fields.
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(default)]
pub struct ChatFrame {
    /// The type of the frame.
    #[serde(rename = "type")]
    pub kind: String,
    /// The name of the sender of the frame.
    pub name: String,
    /// The payload of the frame.
    pub data: String,
    /// The moment the frame was sent, in milliseconds since the Unix epoch.
    pub time: i64,
}

/// The frame the visitor sends to the chat room.
#[derive(Clone, Debug, Default, Serialize)]
pub struct ChatOutgoing {
    /// The type of the frame.
    #[serde(rename = "type")]
    pub kind: String,
    /// The payload of the frame.
    pub data: String,
}

/// One message of the chat room.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ChatMessage {
    /// The identity of the message inside the room.
    pub id: String,
    /// The name of the sender of the message.
    pub sender: String,
    /// The text of the message.
    pub content: String,
    /// The moment the message was sent, in milliseconds since the Unix epoch.
    pub time: i64,
    /// Whether the visitor of the page wrote the message.
    pub own: bool,
    /// Whether the server wrote the message.
    pub system: bool,
}

/// One message of the chat history of the server.
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(default)]
pub struct ChatHistoryMessage {
    /// The identifier of the message.
    pub id: i64,
    /// The identifier of the session the message belongs to.
    pub session_id: String,
    /// The name of the sender of the message.
    pub sender_name: String,
    /// The kind of the sender of the message.
    pub sender_type: String,
    /// The kind of the message.
    pub message_type: String,
    /// The text of the message.
    pub content: String,
    /// The moment the message was sent, in milliseconds since the Unix epoch.
    pub created_at: i64,
}

/// The answer of the chat history endpoint.
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(default)]
pub struct ChatHistory {
    /// The messages of the history.
    pub messages: Vec<ChatHistoryMessage>,
    /// The number of messages the history holds in total.
    pub total: usize,
    /// Whether the history holds older messages.
    pub has_more: bool,
}

/// One visitor that is online in the room.
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(default)]
pub struct OnlineUser {
    /// The name of the visitor.
    pub username: String,
    /// The moment the visitor joined, in milliseconds since the Unix epoch.
    pub join_time: i64,
}

/// The answer of the online users endpoint.
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(default)]
pub struct OnlineUserList {
    /// The visitors that are online.
    pub users: Vec<OnlineUser>,
    /// The number of the visitors that are online.
    pub total_count: usize,
}

/// The flat answer the three upload endpoints answer with.
///
/// The upload endpoints do not use the envelope of the other endpoints: they
/// answer with the status code, the address of the uploaded file and the
/// message of the call in one flat object.
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(default)]
pub struct UploadAnswer {
    /// The business status code; `200` marks a successful call.
    pub code: i32,
    /// The address of the uploaded file.
    pub url: String,
    /// The message of the call.
    pub msg: String,
}
