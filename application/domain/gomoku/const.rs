/// Error message returned when the requested player is not seated in the room.
pub const ERROR_PLAYER_NOT_FOUND: &str = "Player not found";

/// Error message returned when the move is not made on the player's turn.
pub const ERROR_NOT_YOUR_TURN: &str = "Not your turn";

/// Error message returned when the coordinates fall outside the board.
pub const ERROR_INVALID_POSITION: &str = "Invalid position";

/// Error message returned when the room already holds both players.
pub const ERROR_ROOM_IS_FULL: &str = "Room is full";

/// Error message returned when the target cell already holds a stone.
pub const ERROR_POSITION_OCCUPIED: &str = "Position occupied";

/// Error message returned when a move is attempted before the game has started.
pub const ERROR_GAME_NOT_IN_PROGRESS: &str = "Game is not in progress";

/// Error message returned when the game is started without a second player.
pub const ERROR_WAITING_FOR_SECOND_PLAYER: &str = "Waiting for second player";
