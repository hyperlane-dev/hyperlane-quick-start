use super::*;

/// The reactive state the chat page renders from.
#[derive(Clone, Copy, CustomDebug, Data, Default, New)]
pub struct ChatState {
    /// The messages of the room.
    #[get(type(copy))]
    pub messages: Signal<Vec<ChatMessage>>,
    /// The visitors that are online.
    #[get(type(copy))]
    pub users: Signal<Vec<OnlineUser>>,
    /// The number of the visitors that are online.
    #[get(type(copy))]
    pub online_count: Signal<String>,
    /// Whether the room is connected.
    #[get(type(copy))]
    pub connected: Signal<bool>,
    /// The name of the visitor of the page.
    #[get(type(copy))]
    pub username: Signal<String>,
    /// The identity of the visitor of the page.
    #[get(type(copy))]
    pub uuid: Signal<String>,
    /// The message the visitor is writing.
    #[get(type(copy))]
    pub draft: Signal<String>,
    /// Whether the list of the mentioned visitors is open.
    #[get(type(copy))]
    pub mention_open: Signal<bool>,
    /// The text the list of the mentioned visitors is filtered by.
    #[get(type(copy))]
    pub mention_filter: Signal<String>,
    /// The visitor of the list that is selected.
    #[get(type(copy))]
    pub mention_index: Signal<usize>,
    /// The visitors of the room that match the text of the mention.
    #[get(type(copy))]
    pub candidates: Signal<Vec<OnlineUser>>,
    /// The number of the failed connection attempts.
    #[get(type(copy))]
    pub attempts: Signal<u32>,
    /// The message of the last failure.
    #[get(type(copy))]
    pub error: Signal<String>,
    /// Whether a file is being uploaded.
    #[get(type(copy))]
    pub uploading: Signal<bool>,
    /// The message of the last upload.
    #[get(type(copy))]
    pub upload_message: Signal<String>,
}

/// The connection of the chat room of the page.
///
/// The socket and the handlers of the socket live together, so the handlers
/// stay alive for as long as the socket does.
#[derive(Clone)]
pub(crate) struct ChatConnection {
    /// The socket of the room.
    pub socket: WebSocket,
    /// The handlers of the events of the socket.
    pub handlers: Vec<JsValue>,
}
