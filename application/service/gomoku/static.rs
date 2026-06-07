use super::*;

/// room broadcast manager.
pub static ROOM_BROADCAST_MANAGER: OnceLock<RoomBroadcastManager> = OnceLock::new();
