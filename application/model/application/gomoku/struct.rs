use super::*;

/// A player in a gomoku game room, associated with a user ID and stone color.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct GomokuPlayer {
    /// The unique identifier of the player.
    pub(super) user_id: String,
    /// The color assigned to the player (black or white).
    pub(super) color: StoneColor,
}

/// Represents a single move (stone placement) on the gomoku board.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct GomokuMove {
    /// The column index (0-based) of the placed stone.
    #[get(type(copy))]
    pub(super) x: usize,
    /// The row index (0-based) of the placed stone.
    #[get(type(copy))]
    pub(super) y: usize,
    /// The color of the placed stone.
    pub(super) color: StoneColor,
    /// The sequential step number of this move in the game.
    #[get(type(copy))]
    pub(super) step: usize,
}

/// The result of placing a stone, including game status and winner information.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct GomokuPlaceResult {
    /// The current game status after the move.
    pub(super) status: GameStatus,
    /// The winning stone color, if the game has been decided.
    pub(super) winner: Option<StoneColor>,
    /// Flag indicating whether the game ended in a draw.
    #[get(type(copy))]
    pub(super) is_draw: bool,
    /// The move data that was placed.
    pub(super) move_data: GomokuMove,
}

/// A gomoku game room containing players, spectators, the board state, and game history.
#[derive(Clone, Data, Debug, Deserialize, Serialize, ToSchema)]
pub struct GomokuRoom {
    /// The unique identifier of the game room.
    pub(super) room_id: String,
    /// The user ID of the room creator/owner.
    pub(super) owner_id: String,
    /// The list of players currently in the game.
    pub(super) players: Vec<GomokuPlayer>,
    /// The list of user IDs spectating the game.
    pub(super) spectators: Vec<String>,
    /// The 2D game board represented as a grid of stone values.
    pub(super) board: Vec<Vec<u8>>,
    /// The current status of the game (waiting, playing, finished).
    pub(super) status: GameStatus,
    /// The stone color for the next expected turn.
    pub(super) next_turn: StoneColor,
    /// The winning stone color if the game has been decided.
    pub(super) winner: Option<StoneColor>,
    /// The ordered list of all moves made in the game.
    pub(super) moves: Vec<GomokuMove>,
}
