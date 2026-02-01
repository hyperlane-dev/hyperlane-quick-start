use super::*;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema)]
pub enum GameStatus {
    Waiting,
    InProgress,
    Finished,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema)]
pub enum StoneColor {
    Black,
    White,
}

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
