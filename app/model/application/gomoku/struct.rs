use super::*;

#[derive(Debug, Clone, Data, Default, Serialize, Deserialize, ToSchema)]
pub struct GomokuPlayer {
    user_id: String,
    color: StoneColor,
}

#[derive(Debug, Clone, Data, Default, Serialize, Deserialize, ToSchema)]
pub struct GomokuMove {
    x: usize,
    y: usize,
    color: StoneColor,
    step: usize,
}

#[derive(Debug, Clone, Data, Default, Serialize, Deserialize, ToSchema)]
pub struct GomokuPlaceResult {
    status: GameStatus,
    winner: Option<StoneColor>,
    is_draw: bool,
    move_data: GomokuMove,
}

#[derive(Debug, Clone, Data, Serialize, Deserialize, ToSchema)]
pub struct GomokuRoom {
    pub(super) room_id: String,
    pub(super) owner_id: String,
    pub(super) players: Vec<GomokuPlayer>,
    pub(super) spectators: Vec<String>,
    pub(super) board: Vec<Vec<u8>>,
    pub(super) status: GameStatus,
    pub(super) next_turn: StoneColor,
    pub(super) winner: Option<StoneColor>,
    pub(super) moves: Vec<GomokuMove>,
}
