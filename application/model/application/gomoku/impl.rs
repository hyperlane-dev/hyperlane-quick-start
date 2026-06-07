use super::*;

/// Default implementation for `GameStatus`, defaulting to `Waiting`.
impl Default for GameStatus {
    #[instrument_trace]
    fn default() -> Self {
        Self::Waiting
    }
}

/// Default implementation for `StoneColor`, defaulting to `Black`.
impl Default for StoneColor {
    #[instrument_trace]
    fn default() -> Self {
        Self::Black
    }
}

/// Color conversion and manipulation methods for `StoneColor`.
impl StoneColor {
    /// Returns the opposite stone color (Black ↔ White).
    ///
    /// # Returns
    ///
    /// - `StoneColor`: The opposite color.
    #[instrument_trace]
    pub fn opposite(&self) -> Self {
        match self {
            Self::Black => Self::White,
            Self::White => Self::Black,
        }
    }

    /// Converts the stone color to its numeric board representation.
    ///
    /// # Returns
    ///
    /// - `u8`: `1` for Black, `2` for White.
    #[instrument_trace]
    pub fn to_value(&self) -> u8 {
        match self {
            Self::Black => 1,
            Self::White => 2,
        }
    }
}

/// Default implementation for `GomokuMessageType`, defaulting to `Unknown`.
impl Default for GomokuMessageType {
    #[instrument_trace]
    fn default() -> Self {
        Self::Unknown
    }
}

/// Default implementation for `GomokuRoom`, creating an empty room with default settings.
impl Default for GomokuRoom {
    #[instrument_trace]
    fn default() -> Self {
        Self {
            room_id: String::new(),
            owner_id: String::new(),
            players: vec![],
            spectators: vec![],
            board: vec![],
            status: GameStatus::Waiting,
            next_turn: StoneColor::Black,
            winner: None,
            moves: vec![],
        }
    }
}
