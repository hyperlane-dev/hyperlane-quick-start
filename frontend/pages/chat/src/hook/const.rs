/// The endpoint of the chat room.
pub(crate) const PATH_CHAT: &str = "/api/chat";

/// The endpoint of the chat history.
pub(crate) const PATH_HISTORY: &str = "/api/chat/history";

/// The endpoint of the visitors that are online.
pub(crate) const PATH_USERS_ONLINE: &str = "/chat/users/online";

/// The endpoint that registers an upload session.
pub(crate) const PATH_UPLOAD_REGISTER: &str = "/api/upload/register";

/// The endpoint that stores one chunk of an upload session.
pub(crate) const PATH_UPLOAD_SAVE: &str = "/api/upload/save";

/// The endpoint that merges the chunks of an upload session.
pub(crate) const PATH_UPLOAD_MERGE: &str = "/api/upload/merge";

/// The header carrying the identifier of the upload session.
pub(crate) const HEADER_FILE_ID: &str = "x-file-id";

/// The header carrying the number of chunks of the upload session.
pub(crate) const HEADER_TOTAL_CHUNKS: &str = "x-total-chunks";

/// The header carrying the name of the uploaded file.
pub(crate) const HEADER_FILE_NAME: &str = "x-file-name";

/// The header carrying the index of the stored chunk.
pub(crate) const HEADER_CHUNK_INDEX: &str = "x-chunk-index";

/// The size of one chunk of an upload.
pub(crate) const CHUNK_SIZE: i32 = 512 * 1024;

/// The key the identity of the visitor is kept under in the browser storage.
pub(crate) const STORAGE_UUID: &str = "browser_uuid";

/// The number of messages the history is loaded with.
pub(crate) const HISTORY_LIMIT: i32 = 20;

/// The number of milliseconds between two keep alive frames.
pub(crate) const PING_INTERVAL: i32 = 8_888;

/// The number of milliseconds the first reconnect waits.
pub(crate) const RECONNECT_BASE_DELAY: i32 = 3_000;

/// The number of milliseconds a reconnect never waits longer than.
pub(crate) const RECONNECT_MAX_DELAY: i32 = 30_000;

/// The number of failed attempts between two steps of the reconnect delay.
pub(crate) const RECONNECT_STEP: u32 = 5;

/// The type of the frames that carry a message.
pub(crate) const FRAME_TEXT: &str = "Text";

/// The type of the frames that keep the connection alive.
pub(crate) const FRAME_PING: &str = "Ping";

/// The type of the answer to a keep alive frame.
pub(crate) const FRAME_PANG: &str = "Pang";

/// The type of the frames that carry the number of the online visitors.
pub(crate) const FRAME_ONLINE_COUNT: &str = "OnlineCount";

/// The type of the frames the server writes.
pub(crate) const FRAME_SYSTEM: &str = "System";

/// The type of the frames that carry the answer of the assistant.
pub(crate) const FRAME_GPT_RESPONSE: &str = "GptResponse";

/// The prefix of the name of the visitor.
pub(crate) const USER_NAME_PREFIX: &str = "User";

/// The empty text of the page.
pub(crate) const TEXT_EMPTY: &str = "";

/// The HTTP method name of a create request.
pub(crate) const METHOD_POST: &str = "POST";

/// The business status code that marks a successful call.
pub(crate) const STATUS_SUCCESS: i32 = 200;

/// The message reported when the browser window is not available.
pub(crate) const MESSAGE_NO_WINDOW: &str = "the browser window is not available";

/// The message reported when the request could not be sent.
pub(crate) const MESSAGE_REQUEST_FAILED: &str = "the request could not be sent";

/// The message reported when the response payload is not valid.
pub(crate) const MESSAGE_INVALID_PAYLOAD: &str = "the response payload is not valid";

/// The separator between the failure prefix and its cause.
pub(crate) const MESSAGE_SEPARATOR: &str = ": ";
