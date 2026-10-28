use super::*;

/// A player in a gomoku game room, associated with a user ID and stone color.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct GomokuPlayer {
    /// The unique identifier of the player.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) user_id: String,
    /// The color assigned to the player (black or white).
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) color: StoneColor,
}

/// Represents a single move (stone placement) on the gomoku board.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct GomokuMove {
    /// The column index (0-based) of the placed stone.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) x: usize,
    /// The row index (0-based) of the placed stone.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) y: usize,
    /// The color of the placed stone.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) color: StoneColor,
    /// The sequential step number of this move in the game.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) step: usize,
}

/// The result of placing a stone, including game status and winner information.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct GomokuPlaceResult {
    /// The current game status after the move.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) status: GameStatus,
    /// The winning stone color, if the game has been decided.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) winner: Option<StoneColor>,
    /// Flag indicating whether the game ended in a draw.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) is_draw: bool,
    /// The move data that was placed.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) move_data: GomokuMove,
}

/// A gomoku game room containing players, spectators, the board state, and game history.
#[derive(Clone, Data, Debug, Deserialize, Serialize, ToSchema)]
pub struct GomokuRoom {
    /// The unique identifier of the game room.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) room_id: String,
    /// The user ID of the room creator/owner.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) owner_id: String,
    /// The list of players currently in the game.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) players: Vec<GomokuPlayer>,
    /// The list of user IDs spectating the game.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) spectators: Vec<String>,
    /// The 2D game board represented as a grid of stone values.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) board: Vec<Vec<u8>>,
    /// The current status of the game (waiting, playing, finished).
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) status: GameStatus,
    /// The stone color for the next expected turn.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) next_turn: StoneColor,
    /// The winning stone color if the game has been decided.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) winner: Option<StoneColor>,
    /// The ordered list of all moves made in the game.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) moves: Vec<GomokuMove>,
}
