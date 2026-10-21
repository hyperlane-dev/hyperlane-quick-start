use super::*;

/// The reactive state the gomoku page renders from.
#[derive(Clone, Copy, CustomDebug, Data, Default, New)]
pub struct GomokuState {
    /// The identifier of the visitor.
    #[get(type(copy))]
    pub user_id: Signal<String>,
    /// The room the visitor asked for.
    #[get(type(copy))]
    pub room_id: Signal<String>,
    /// The room the visitor is in.
    #[get(type(copy))]
    pub room: Signal<Option<GomokuRoom>>,
    /// The cells of the board of the room.
    #[get(type(copy))]
    pub cells: Signal<Vec<GomokuCell>>,
    /// The players of the room.
    #[get(type(copy))]
    pub players: Signal<Vec<GomokuPlayer>>,
    /// The visitors that watch the room.
    #[get(type(copy))]
    pub spectators: Signal<Vec<String>>,
    /// The moves of the game, the newest one first.
    #[get(type(copy))]
    pub moves: Signal<Vec<GomokuMove>>,
    /// The role the visitor plays in the room.
    #[get(type(copy))]
    pub role: Signal<String>,
    /// Whether the client is connected to the server.
    #[get(type(copy))]
    pub connected: Signal<bool>,
    /// The message of the last failure.
    #[get(type(copy))]
    pub error: Signal<String>,
}
