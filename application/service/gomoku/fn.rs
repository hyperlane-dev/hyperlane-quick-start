use super::*;

pub fn get_room_broadcast_manager() -> &'static RoomBroadcastManager {
    ROOM_BROADCAST_MANAGER.get_or_init(RoomBroadcastManager::new)
}
