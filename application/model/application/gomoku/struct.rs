use super::*;

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct GomokuPlayer {
    #[get(pub)]
    pub(super) user_id: String,
    #[get(pub)]
    pub(super) color: StoneColor,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct GomokuMove {
    #[get(type(copy), pub)]
    pub(super) x: usize,
    #[get(type(copy), pub)]
    pub(super) y: usize,
    #[get(pub)]
    pub(super) color: StoneColor,
    #[get(type(copy), pub)]
    pub(super) step: usize,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct GomokuPlaceResult {
    #[get(pub)]
    pub(super) status: GameStatus,
    #[get(pub)]
    pub(super) winner: Option<StoneColor>,
    #[get(type(copy), pub)]
    pub(super) is_draw: bool,
    #[get(pub)]
    pub(super) move_data: GomokuMove,
}

#[derive(Clone, Data, Debug, Deserialize, Serialize, ToSchema)]
pub struct GomokuRoom {
    #[get(pub)]
    pub(super) room_id: String,
    #[get(pub)]
    pub(super) owner_id: String,
    #[get(pub)]
    pub(super) players: Vec<GomokuPlayer>,
    #[get(pub)]
    pub(super) spectators: Vec<String>,
    #[get(pub)]
    pub(super) board: Vec<Vec<u8>>,
    #[get(pub)]
    pub(super) status: GameStatus,
    #[get(pub)]
    pub(super) next_turn: StoneColor,
    #[get(pub)]
    pub(super) winner: Option<StoneColor>,
    #[get(pub)]
    pub(super) moves: Vec<GomokuMove>,
}
