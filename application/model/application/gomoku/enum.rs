use super::*;

/// Enumeration of game status.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema)]
pub enum GameStatus {
    Waiting,
    InProgress,
    Finished,
}

/// Enumeration of stone color.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema)]
pub enum StoneColor {
    Black,
    White,
}

/// Enumeration of gomoku message type.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema)]
pub enum GomokuMessageType {
    CreateRoom,
    JoinRoom,
    Spectate,
    Leave,
    Start,
    PlaceStone,
    Sync,
    RoomState,
    MoveResult,
    Error,
    Ping,
    Pang,
    Unknown,
}
