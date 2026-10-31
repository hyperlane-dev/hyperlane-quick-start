/// Query parameter key for the x coordinate.
pub const POSITION_X: &str = "x";

/// Query parameter key for the y coordinate.
pub const POSITION_Y: &str = "y";

/// Query parameter key for uuid.
pub const QUERY_UUID: &str = "uuid";

/// Error message when unsupported message type.
pub(crate) const ERROR_UNSUPPORTED_MESSAGE_TYPE: &str = "Unsupported message type";

/// Error message when room already exists.
pub(crate) const ERROR_ROOM_ALREADY_EXISTS: &str = "Room already exists";

/// Error message when room not found.
pub(crate) const ERROR_ROOM_NOT_FOUND: &str = "Room not found";

/// Error message when already in room.
pub(crate) const ERROR_ALREADY_IN_ROOM: &str = "Already in room";

/// Error message when user not in room.
pub(crate) const ERROR_USER_NOT_IN_ROOM: &str = "User not in room";

/// Error message when invalid x.
pub(crate) const ERROR_INVALID_X: &str = "Invalid x";

/// Error message when invalid y.
pub(crate) const ERROR_INVALID_Y: &str = "Invalid y";

/// Default broadcast capacity.
pub(crate) const DEFAULT_BROADCAST_CAPACITY: usize = 128;

/// Query parameter key for uid.
pub(crate) const QUERY_UID: &str = "uid";

/// JSON key carrying the operation result.
pub(crate) const JSON_KEY_RESULT: &str = "result";

/// JSON key carrying the room identifier.
pub(crate) const JSON_KEY_ROOM: &str = "room";

/// JSON key carrying a message.
pub(crate) const JSON_KEY_MESSAGE: &str = "message";
