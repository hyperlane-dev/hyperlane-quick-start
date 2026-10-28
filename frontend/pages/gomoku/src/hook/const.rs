/// The role of a visitor that plays in the room.
pub(crate) const ROLE_PLAYER: &str = "Player";

/// The role of a visitor that watches the room.
pub(crate) const ROLE_SPECTATOR: &str = "Spectator";

/// The role of a visitor that is not in the room.
pub(crate) const ROLE_NONE: &str = "-";

/// The text an empty field starts with.
pub(crate) const TEXT_EMPTY: &str = "";

/// The feedback reported when a room action has no room identifier.
pub(crate) const ERROR_ROOM_REQUIRED: &str = "Enter a room ID before joining or watching.";

/// The number of the columns of the board.
pub(crate) const BOARD_COLUMNS: usize = 15;

/// The number of the rows of the board.
pub(crate) const BOARD_ROWS: usize = 15;
