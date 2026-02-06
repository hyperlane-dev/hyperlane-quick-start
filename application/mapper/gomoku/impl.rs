use super::*;

impl GomokuRoomMapper {
    #[instrument_trace]
    pub async fn get_room(room_id: &str) -> Option<GomokuRoom> {
        let rooms: &ArcRwLock<HashMap<String, GomokuRoom>> = get_global_gomoku_rooms();
        let rooms_guard: RwLockReadGuard<'_, HashMap<String, GomokuRoom>> = rooms.read().await;
        rooms_guard.get(room_id).cloned()
    }

    #[instrument_trace]
    pub async fn save_room(room: GomokuRoom) {
        let rooms: &ArcRwLock<HashMap<String, GomokuRoom>> = get_global_gomoku_rooms();
        let mut rooms_guard: RwLockWriteGuard<'_, HashMap<String, GomokuRoom>> =
            rooms.write().await;
        rooms_guard.insert(room.get_room_id().clone(), room);
    }

    #[instrument_trace]
    pub async fn remove_room(room_id: &str) {
        let rooms: &ArcRwLock<HashMap<String, GomokuRoom>> = get_global_gomoku_rooms();
        let mut rooms_guard: RwLockWriteGuard<'_, HashMap<String, GomokuRoom>> =
            rooms.write().await;
        rooms_guard.remove(room_id);
    }

    #[instrument_trace]
    pub async fn set_user_room(user_id: &str, room_id: &str) {
        let user_rooms: &ArcRwLock<HashMap<String, String>> = get_global_gomoku_user_rooms();
        let mut rooms_guard: RwLockWriteGuard<'_, HashMap<String, String>> =
            user_rooms.write().await;
        rooms_guard.insert(user_id.to_string(), room_id.to_string());
    }

    #[instrument_trace]
    pub async fn remove_user_room(user_id: &str) {
        let user_rooms: &ArcRwLock<HashMap<String, String>> = get_global_gomoku_user_rooms();
        let mut rooms_guard: RwLockWriteGuard<'_, HashMap<String, String>> =
            user_rooms.write().await;
        rooms_guard.remove(user_id);
    }

    #[instrument_trace]
    pub async fn get_user_room(user_id: &str) -> Option<String> {
        let user_rooms: &ArcRwLock<HashMap<String, String>> = get_global_gomoku_user_rooms();
        let rooms_guard: RwLockReadGuard<'_, HashMap<String, String>> = user_rooms.read().await;
        rooms_guard.get(user_id).cloned()
    }

    #[instrument_trace]
    pub async fn get_room_user_ids(room_id: &str) -> Vec<String> {
        let mut users: HashSet<String> = HashSet::new();
        if let Some(room) = Self::get_room(room_id).await {
            for player in room.get_players().iter() {
                users.insert(player.get_user_id().clone());
            }
            for spectator in room.get_spectators().iter() {
                users.insert(spectator.clone());
            }
        }
        users.into_iter().collect()
    }
}
