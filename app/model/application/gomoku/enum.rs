use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub enum GameStatus {
    Waiting,
    InProgress,
    Finished,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub enum StoneColor {
    Black,
    White,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
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

