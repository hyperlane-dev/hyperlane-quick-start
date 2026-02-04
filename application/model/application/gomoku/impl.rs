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
        Self {
            room_id: String::new(),
            owner_id: String::new(),
            players: Vec::new(),
            spectators: Vec::new(),
            board: Vec::new(),
            status: GameStatus::Waiting,
            next_turn: StoneColor::Black,
            winner: None,
            moves: Vec::new(),
        }
    }
}
