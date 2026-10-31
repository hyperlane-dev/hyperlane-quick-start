/// The endpoint of the gomoku WebSocket.
pub(crate) const PATH_GOMOKU: &str = "/api/gomoku";

/// The query parameter that carries the identifier of the visitor.
pub(crate) const QUERY_UID: &str = "uid";

/// The key the identifier of the visitor is kept under.
pub(crate) const STORAGE_USER_ID: &str = "gomoku_user_id";

/// The prefix of the identifier a visitor gets when it visits the page.
pub(crate) const PREFIX_USER_ID: &str = "user_";

/// The number of times the client tries to connect again before it gives up.
pub(crate) const MAX_RECONNECT_ATTEMPTS: u32 = 5;

/// The number of milliseconds the client waits between two attempts.
pub(crate) const RECONNECT_INTERVAL_MS: u32 = 1000;

/// The frame that creates a room.
pub(crate) const TYPE_CREATE_ROOM: &str = "CreateRoom";

/// The frame that joins a room.
pub(crate) const TYPE_JOIN_ROOM: &str = "JoinRoom";

/// The frame that watches a room.
pub(crate) const TYPE_SPECTATE: &str = "Spectate";

/// The frame that leaves a room.
pub(crate) const TYPE_LEAVE: &str = "Leave";

/// The frame that places a stone.
pub(crate) const TYPE_PLACE_STONE: &str = "PlaceStone";

/// The frame that carries the state of a room.
pub(crate) const TYPE_ROOM_STATE: &str = "RoomState";

/// The frame that carries the result of a move.
pub(crate) const TYPE_MOVE_RESULT: &str = "MoveResult";

/// The frame that carries a failure.
pub(crate) const TYPE_ERROR: &str = "Error";

/// The key of the room inside the payload of a move.
pub(crate) const KEY_ROOM: &str = "room";

/// The key of the message inside the payload of a failure.
pub(crate) const KEY_MESSAGE: &str = "message";

/// The message reported when the server sends a failure without a message.
pub(crate) const MESSAGE_UNKNOWN_ERROR: &str = "The operation failed";

/// The number of the digits of a base number the identifier of a visitor
/// holds.
pub(crate) const DIGITS_BASE: u32 = 36;
