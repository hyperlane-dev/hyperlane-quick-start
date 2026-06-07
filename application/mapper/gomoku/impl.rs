use super::*;

/// Implementation of methods for `GomokuRoomMapper`.
impl GomokuRoomMapper {
    /// Retrieves a gomoku room by its room ID from the global in-memory store.
    ///
    /// # Arguments
    ///
    /// - `&str`: The room ID to look up.
    ///
    /// # Returns
    ///
    /// - `Option<GomokuRoom>`: The room if found, or None.
    #[instrument_trace]
    pub async fn get_room(room_id: &str) -> Option<GomokuRoom> {
        let rooms: &ArcRwLock<HashMap<String, GomokuRoom>> = get_global_gomoku_rooms();
        let rooms_guard: RwLockReadGuard<'_, HashMap<String, GomokuRoom>> = rooms.read().await;
        rooms_guard.get(room_id).cloned()
    }

    /// Saves or updates a gomoku room in the global in-memory store.
    ///
    /// # Arguments
    ///
    /// - `GomokuRoom`: The room instance to save.
    #[instrument_trace]
    pub async fn save_room(room: GomokuRoom) {
        let rooms: &ArcRwLock<HashMap<String, GomokuRoom>> = get_global_gomoku_rooms();
        let mut rooms_guard: RwLockWriteGuard<'_, HashMap<String, GomokuRoom>> =
            rooms.write().await;
        rooms_guard.insert(room.get_room_id().clone(), room);
    }

    /// Removes a gomoku room from the global in-memory store by its room ID.
    ///
    /// # Arguments
    ///
    /// - `&str`: The room ID to remove.
    #[instrument_trace]
    pub async fn remove_room(room_id: &str) {
        let rooms: &ArcRwLock<HashMap<String, GomokuRoom>> = get_global_gomoku_rooms();
        let mut rooms_guard: RwLockWriteGuard<'_, HashMap<String, GomokuRoom>> =
            rooms.write().await;
        rooms_guard.remove(room_id);
    }

    /// Associates a user with a room in the global user-room mapping.
    ///
    /// # Arguments
    ///
    /// - `&str`: The user ID.
    /// - `&str`: The room ID to associate.
    #[instrument_trace]
    pub async fn set_user_room(user_id: &str, room_id: &str) {
        let user_rooms: &ArcRwLock<HashMap<String, String>> = get_global_gomoku_user_rooms();
        let mut rooms_guard: RwLockWriteGuard<'_, HashMap<String, String>> =
            user_rooms.write().await;
        rooms_guard.insert(user_id.to_string(), room_id.to_string());
    }

    /// Removes a user's room association from the global user-room mapping.
    ///
    /// # Arguments
    ///
    /// - `&str`: The user ID to remove.
    #[instrument_trace]
    pub async fn remove_user_room(user_id: &str) {
        let user_rooms: &ArcRwLock<HashMap<String, String>> = get_global_gomoku_user_rooms();
        let mut rooms_guard: RwLockWriteGuard<'_, HashMap<String, String>> =
            user_rooms.write().await;
        rooms_guard.remove(user_id);
    }

    /// Retrieves the room ID associated with a user from the global user-room mapping.
    ///
    /// # Arguments
    ///
    /// - `&str`: The user ID to look up.
    ///
    /// # Returns
    ///
    /// - `Option<String>`: The room ID if the user is in a room, or None.
    #[instrument_trace]
    pub async fn get_user_room(user_id: &str) -> Option<String> {
        let user_rooms: &ArcRwLock<HashMap<String, String>> = get_global_gomoku_user_rooms();
        let rooms_guard: RwLockReadGuard<'_, HashMap<String, String>> = user_rooms.read().await;
        rooms_guard.get(user_id).cloned()
    }

    /// Collects all user IDs (players and spectators) currently in a gomoku room.
    ///
    /// # Arguments
    ///
    /// - `&str`: The room ID to query.
    ///
    /// # Returns
    ///
    /// - `Vec<String>`: A list of unique user IDs in the room.
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
