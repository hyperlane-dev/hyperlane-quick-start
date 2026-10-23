use super::*;

/// One player of a room.
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(default)]
pub struct GomokuPlayer {
    /// The identifier of the player.
    pub user_id: String,
    /// The colour the player plays.
    pub color: GomokuStone,
}

/// One move of a game.
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(default)]
pub struct GomokuMove {
    /// The column of the stone.
    pub x: usize,
    /// The row of the stone.
    pub y: usize,
    /// The colour of the stone.
    pub color: GomokuStone,
    /// The number of the move.
    pub step: usize,
}

/// The room a game is played in.
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(default)]
pub struct GomokuRoom {
    /// The identifier of the room.
    pub room_id: String,
    /// The identifier of the owner of the room.
    pub owner_id: String,
    /// The players of the room.
    pub players: Vec<GomokuPlayer>,
    /// The identifiers of the visitors that watch the room.
    pub spectators: Vec<String>,
    /// The board of the room.
    pub board: Vec<Vec<u8>>,
    /// The status of the room.
    pub status: GomokuStatus,
    /// The colour that plays next.
    pub next_turn: GomokuStone,
    /// The colour that won the game.
    pub winner: Option<GomokuStone>,
    /// The moves of the game.
    pub moves: Vec<GomokuMove>,
}

/// One frame the server sends over the WebSocket.
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(default)]
pub struct GomokuFrame {
    /// The kind of the frame.
    pub r#type: String,
    /// The room the frame belongs to.
    pub room_id: String,
    /// The identifier of the sender of the frame.
    pub sender_id: String,
    /// The payload of the frame.
    pub payload: serde_json::Value,
    /// The moment the frame was sent.
    pub time: i64,
}

/// One cell of the board.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct GomokuCell {
    /// The column of the cell.
    pub x: usize,
    /// The row of the cell.
    pub y: usize,
    /// The stone of the cell: empty, black or white.
    pub value: u8,
}
