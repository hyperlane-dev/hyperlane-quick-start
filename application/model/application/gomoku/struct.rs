use super::*;

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct GomokuPlayer {
    pub(super) user_id: String,
    pub(super) color: StoneColor,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct GomokuMove {
    #[get(type(copy), pub)]
    pub(super) x: usize,
    #[get(type(copy), pub)]
    pub(super) y: usize,
    pub(super) color: StoneColor,
    #[get(type(copy), pub)]
    pub(super) step: usize,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct GomokuPlaceResult {
    pub(super) status: GameStatus,
    pub(super) winner: Option<StoneColor>,
    #[get(type(copy), pub)]
    pub(super) is_draw: bool,
    pub(super) move_data: GomokuMove,
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
