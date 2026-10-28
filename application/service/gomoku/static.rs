use super::*;

/// room broadcast manager.
pub(crate) static ROOM_BROADCAST_MANAGER: OnceLock<RoomBroadcastManager> = OnceLock::new();
