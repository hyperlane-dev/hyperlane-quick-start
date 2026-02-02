use super::*;

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct GomokuPlayer {
    user_id: String,
    color: StoneColor,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct GomokuMove {
    x: usize,
    y: usize,
    color: StoneColor,
    step: usize,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct GomokuPlaceResult {
    status: GameStatus,
    winner: Option<StoneColor>,
    is_draw: bool,
    move_data: GomokuMove,
}

#[derive(Clone, Data, Debug, Deserialize, Serialize, ToSchema)]
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
