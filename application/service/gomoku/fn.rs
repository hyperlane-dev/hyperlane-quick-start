use super::*;

/// get room broadcast manager.
///
/// # Returns
///
/// - `&'static RoomBroadcastManager` - The room broadcast manager.
pub fn get_room_broadcast_manager() -> &'static RoomBroadcastManager {
    ROOM_BROADCAST_MANAGER.get_or_init(RoomBroadcastManager::new)
}
