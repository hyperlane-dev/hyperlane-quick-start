use super::*;

impl Default for GameStatus {
    #[instrument_trace]
    fn default() -> Self {
        Self::Waiting
    }
}

impl Default for StoneColor {
    #[instrument_trace]
    fn default() -> Self {
        Self::Black
    }
}

impl StoneColor {
    #[instrument_trace]
    pub fn opposite(&self) -> Self {
        match self {
            Self::Black => Self::White,
            Self::White => Self::Black,
        }
    }

    #[instrument_trace]
    pub fn to_value(&self) -> u8 {
        match self {
            Self::Black => 1,
            Self::White => 2,
        }
    }
}

impl Default for GomokuMessageType {
    #[instrument_trace]
    fn default() -> Self {
        Self::Unknown
    }
}

impl Default for GomokuRoom {
    #[instrument_trace]
    fn default() -> Self {
        let board: Vec<Vec<u8>> = build_empty_board(15);
        Self {
            room_id: String::new(),
            owner_id: String::new(),
            players: Vec::new(),
            spectators: Vec::new(),
            board,
            status: GameStatus::Waiting,
            next_turn: StoneColor::Black,
            winner: None,
            moves: Vec::new(),
        }
    }
}

#[instrument_trace]
fn build_empty_board(size: usize) -> Vec<Vec<u8>> {
    let mut board: Vec<Vec<u8>> = Vec::with_capacity(size);
    for _ in 0..size {
        let row: Vec<u8> = vec![0; size];
        board.push(row);
    }
    board
}

