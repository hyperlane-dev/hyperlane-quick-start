use super::*;

impl GomokuRoomMapper {
    #[instrument_trace]
    pub fn get_room(room_id: &str) -> Option<GomokuRoom> {
        let rooms: &Arc<RwLock<HashMap<String, GomokuRoom>>> = get_global_gomoku_rooms();
        if let Ok(rooms_guard) = rooms.read() {
            return rooms_guard.get(room_id).cloned();
        }
        None
    }

    #[instrument_trace]
    pub fn save_room(room: GomokuRoom) {
        let rooms: &Arc<RwLock<HashMap<String, GomokuRoom>>> = get_global_gomoku_rooms();
        if let Ok(mut rooms_guard) = rooms.write() {
            rooms_guard.insert(room.get_room_id().clone(), room);
        }
    }

    #[instrument_trace]
    pub fn remove_room(room_id: &str) {
        let rooms: &Arc<RwLock<HashMap<String, GomokuRoom>>> = get_global_gomoku_rooms();
        if let Ok(mut rooms_guard) = rooms.write() {
            rooms_guard.remove(room_id);
        }
    }

    #[instrument_trace]
    pub fn set_user_room(user_id: &str, room_id: &str) {
        let user_rooms: &Arc<RwLock<HashMap<String, String>>> = get_global_gomoku_user_rooms();
        if let Ok(mut rooms_guard) = user_rooms.write() {
            rooms_guard.insert(user_id.to_string(), room_id.to_string());
        }
    }

    #[instrument_trace]
    pub fn remove_user_room(user_id: &str) {
        let user_rooms: &Arc<RwLock<HashMap<String, String>>> = get_global_gomoku_user_rooms();
        if let Ok(mut rooms_guard) = user_rooms.write() {
            rooms_guard.remove(user_id);
        }
    }

    #[instrument_trace]
    pub fn get_user_room(user_id: &str) -> Option<String> {
        let user_rooms: &Arc<RwLock<HashMap<String, String>>> = get_global_gomoku_user_rooms();
        if let Ok(rooms_guard) = user_rooms.read() {
            return rooms_guard.get(user_id).cloned();
        }
        None
    }

    #[instrument_trace]
    pub fn get_room_user_ids(room_id: &str) -> Vec<String> {
        let mut users: HashSet<String> = HashSet::new();
        if let Some(room) = Self::get_room(room_id) {
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
