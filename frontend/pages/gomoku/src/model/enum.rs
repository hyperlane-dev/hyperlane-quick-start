use super::*;

/// The colour of a stone of the board.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq)]
pub enum GomokuStone {
    /// The black stone, the one the owner of a room plays.
    #[default]
    Black,
    /// The white stone, the one the second player plays.
    White,
}

/// The status of a room.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq)]
pub enum GomokuStatus {
    /// The room waits for a second player.
    #[default]
    Waiting,
    /// The room holds a game that is being played.
    InProgress,
    /// The game of the room is over.
    Finished,
}
