use super::*;

/// Room subscription and message broadcasting management for `RoomBroadcastManager`.
impl RoomBroadcastManager {
    /// Creates a new `RoomBroadcastManager` with empty broadcast channels and user subscriptions.
    ///
    /// # Returns
    ///
    /// - `RoomBroadcastManager`: A new instance with no active rooms or subscribers.
    #[instrument_trace]
    pub fn new() -> Self {
        Self {
            broadcast_map: Arc::new(BroadcastMap::new()),
            user_subscriptions: arc_rwlock(HashMap::new()),
        }
    }

    /// Subscribes a user to a room's broadcast channel, switching from their previous room if any.
    ///
    /// # Arguments
    ///
    /// - `&str`: The unique identifier of the user.
    /// - `&str`: The identifier of the room to subscribe to.
    #[instrument_trace]
    pub async fn subscribe_to_room(&self, user_id: &str, room_id: &str) {
        let mut subs: RwLockWriteGuard<'_, HashMap<String, String>> =
            self.user_subscriptions.write().await;
        if let Some(old_room) = subs.get(user_id) {
            if old_room == room_id {
                trace!("User {user_id} already subscribed to room {room_id}");
                return;
            }
            trace!("User {user_id} switching from room {old_room} to {room_id}");
        }
        let _receiver: BroadcastMapReceiver<String> = self
            .broadcast_map
            .subscribe_or_insert(room_id, DEFAULT_BROADCAST_CAPACITY);
        subs.insert(user_id.to_string(), room_id.to_string());
        trace!("User {user_id} subscribed to room {room_id}");
    }

    /// Removes a user from their subscribed room's broadcast channel.
    ///
    /// # Arguments
    ///
    /// - `&str`: The unique identifier of the user to unsubscribe.
    #[instrument_trace]
    pub async fn unsubscribe_user(&self, user_id: &str) {
        let mut subs: RwLockWriteGuard<'_, HashMap<String, String>> =
            self.user_subscriptions.write().await;
        if let Some(room_id) = subs.remove(user_id) {
            trace!("User {user_id} unsubscribed from room {room_id}");
        }
    }

    /// Sends a message to all subscribers of the specified room.
    ///
    /// # Arguments
    ///
    /// - `&str`: The identifier of the target room.
    /// - `&str`: The message content to broadcast.
    #[instrument_trace]
    pub fn broadcast_to_room(&self, room_id: &str, message: &str) {
        if self
            .broadcast_map
            .try_send(room_id, message.to_string())
            .is_err()
        {
            trace!("Failed to broadcast to room {room_id}: no active receivers");
        } else {
            trace!("Broadcasted message to room {room_id}");
        }
    }

    /// Returns the room identifier that the user is currently subscribed to.
    ///
    /// # Arguments
    ///
    /// - `&str`: The unique identifier of the user.
    ///
    /// # Returns
    ///
    /// - `Option<String>`: The room identifier if the user is subscribed, or `None`.
    #[instrument_trace]
    pub async fn get_user_room(&self, user_id: &str) -> Option<String> {
        let subs: RwLockReadGuard<'_, HashMap<String, String>> =
            self.user_subscriptions.read().await;
        subs.get(user_id).cloned()
    }
}

/// Default implementation for `RoomBroadcastManager`, delegating to `new`.
impl Default for RoomBroadcastManager {
    #[instrument_trace]
    fn default() -> Self {
        Self::new()
    }
}

/// WebSocket connection hook that re-subscribes users to their rooms on reconnect.
impl ServerHook for GomokuConnectedHook {
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    /// Handles a WebSocket connection by looking up the user's room and subscribing them
    /// to receive room state updates upon reconnection.
    #[try_get_request_query("uuid" => uuid_opt)]
    #[instrument_trace]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        let uuid: String = uuid_opt.unwrap_or_default();
        if uuid.is_empty() {
            return Status::Continue;
        }
        if let Some(room_id) = GomokuRoomMapper::get_user_room(&uuid).await {
            let manager: &RoomBroadcastManager = get_room_broadcast_manager();
            manager.subscribe_to_room(&uuid, &room_id).await;
            trace!("User {uuid} connected and subscribed to room {room_id}");
            if let Some(room) = GomokuRoomMapper::get_room(&room_id).await {
                let resp_body: ResponseBody = GomokuWebSocketService::build_response_body(
                    GomokuMessageType::RoomState,
                    &room_id,
                    &uuid,
                    json!(room),
                )
                .unwrap_or_default();
                ctx.get_mut_response().set_body(&resp_body);
            }
        }
        Status::Continue
    }
}

/// WebSocket request hook that processes Gomoku game actions and broadcasts updates.
impl ServerHook for GomokuRequestHook {
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    /// Handles incoming WebSocket messages by dispatching to `GomokuWebSocketService`
    /// and broadcasting room state changes to all subscribers.
    #[try_get_request_query("uid" => uid_opt)]
    #[request_body_json_result(req_data_res: GomokuWsRequest)]
    #[instrument_trace]
    async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
        let req_data: GomokuWsRequest = req_data_res.unwrap();
        if GomokuWebSocketService::handle_ping_request(stream, ctx, &req_data).await {
            return Status::Continue;
        }
        let uid: String = uid_opt.unwrap_or_default();
        match GomokuWebSocketService::handle_request(ctx, &req_data, &uid).await {
            Ok((resp_body, room_id)) => {
                ctx.get_mut_response().set_body(&resp_body);
                if !room_id.is_empty() {
                    GomokuWebSocketService::broadcast_room(&room_id, &uid, &resp_body).await;
                }
            }
            Err(error) => {
                let resp_body: ResponseBody =
                    GomokuWebSocketService::error_response(&uid, &req_data, error);
                ctx.get_mut_response().set_body(&resp_body);
            }
        }
        Status::Continue
    }
}

/// Post-send hook for Gomoku WebSocket messages (no-op).
impl ServerHook for GomokuSendedHook {
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[instrument_trace]
    async fn handle(self, _stream: &mut Stream, _: &mut Context) -> Status {
        Status::Continue
    }
}

/// WebSocket disconnection hook that unsubscribes users from their rooms.
impl ServerHook for GomokuClosedHook {
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    /// Handles a WebSocket disconnection by removing the user from their subscribed room.
    #[try_get_request_query("uid" => uid_opt)]
    #[instrument_trace]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        let uid: String = uid_opt.unwrap_or_default();
        if uid.is_empty() {
            return Status::Continue;
        }
        let manager: &RoomBroadcastManager = get_room_broadcast_manager();
        manager.unsubscribe_user(&uid).await;
        trace!("User {uid} disconnected, unsubscribed from WebSocket only");
        Status::Continue
    }
}

/// Game logic handlers and message building for `GomokuWebSocketService`.
impl GomokuWebSocketService {
    /// Handles a ping message by responding with a pong, keeping the WebSocket connection alive.
    ///
    /// # Arguments
    ///
    /// - `&mut Stream`: The WebSocket stream (unused).
    /// - `&mut Context`: The request context used to set the response body.
    /// - `&GomokuWsRequest`: The incoming ping request.
    ///
    /// # Returns
    ///
    /// - `bool`: `true` if the request was a ping and was handled, `false` otherwise.
    #[try_get_request_query("uid" => uid_opt)]
    #[instrument_trace]
    pub async fn handle_ping_request(
        _stream: &mut Stream,
        ctx: &mut Context,
        req_data: &GomokuWsRequest,
    ) -> bool {
        if req_data.get_type() == &GomokuMessageType::Ping {
            let uid: String = uid_opt.unwrap_or_default();
            let resp_body: ResponseBody = Self::build_response_body(
                GomokuMessageType::Pang,
                req_data.get_room_id(),
                &uid,
                json!({}),
            )
            .unwrap_or_default();
            ctx.get_mut_response().set_body(&resp_body);
            return true;
        }
        false
    }

    /// Dispatches a Gomoku WebSocket request to the appropriate handler based on message type.
    ///
    /// # Arguments
    ///
    /// - `&mut Context`: The request context.
    /// - `&GomokuWsRequest`: The incoming game request.
    /// - `&str`: The sender's user identifier.
    ///
    /// # Returns
    ///
    /// - `Result<(ResponseBody, String), String>`: The response body and room identifier on success, or an error message.
    #[instrument_trace]
    pub async fn handle_request(
        _: &mut Context,
        req_data: &GomokuWsRequest,
        sender_id: &str,
    ) -> Result<(ResponseBody, String), String> {
        match req_data.get_type() {
            GomokuMessageType::CreateRoom => Self::handle_create_room(req_data, sender_id).await,
            GomokuMessageType::JoinRoom => Self::handle_join_room(req_data, sender_id).await,
            GomokuMessageType::Spectate => Self::handle_spectate(req_data, sender_id).await,
            GomokuMessageType::Leave => Self::handle_leave(req_data, sender_id).await,
            GomokuMessageType::PlaceStone => Self::handle_place_stone(req_data, sender_id).await,
            GomokuMessageType::Sync => Self::handle_sync(req_data, sender_id).await,
            _ => Err("Unsupported message type".to_string()),
        }
    }

    /// Creates a new Gomoku game room with the sender as the first player.
    ///
    /// # Arguments
    ///
    /// - `&GomokuWsRequest`: The request containing the optional room identifier.
    /// - `&str`: The creator's user identifier.
    ///
    /// # Returns
    ///
    /// - `Result<(ResponseBody, String), String>`: The room state response and room identifier, or an error if the room already exists.
    #[instrument_trace]
    async fn handle_create_room(
        req_data: &GomokuWsRequest,
        sender_id: &str,
    ) -> Result<(ResponseBody, String), String> {
        let mut room_id: String = req_data.get_room_id().clone();
        if room_id.trim().is_empty() {
            room_id = Self::generate_room_id(sender_id);
        }
        if GomokuRoomMapper::get_room(&room_id).await.is_some() {
            return Err(ERROR_ROOM_ALREADY_EXISTS.to_string());
        }
        let room: GomokuRoom = GomokuDomain::create_room(&room_id, sender_id);
        GomokuRoomMapper::save_room(room.clone()).await;
        GomokuRoomMapper::set_user_room(sender_id, &room_id).await;
        let manager: &RoomBroadcastManager = get_room_broadcast_manager();
        manager.subscribe_to_room(sender_id, &room_id).await;
        let resp_body: ResponseBody = Self::build_response_body(
            GomokuMessageType::RoomState,
            &room_id,
            sender_id,
            json!(room),
        )?;
        Ok((resp_body, room_id))
    }

    /// Joins an existing Gomoku room as the second player and starts the game if both players are present.
    ///
    /// # Arguments
    ///
    /// - `&GomokuWsRequest`: The request containing the room identifier.
    /// - `&str`: The joining player's user identifier.
    ///
    /// # Returns
    ///
    /// - `Result<(ResponseBody, String), String>`: The room state response and room identifier, or an error if the room is not found or full.
    #[instrument_trace]
    async fn handle_join_room(
        req_data: &GomokuWsRequest,
        sender_id: &str,
    ) -> Result<(ResponseBody, String), String> {
        let room_id: String = req_data.get_room_id().clone();
        let mut room: GomokuRoom = GomokuRoomMapper::get_room(&room_id)
            .await
            .ok_or("Room not found".to_string())?;
        let _color: StoneColor = GomokuDomain::add_player(&mut room, sender_id)?;
        GomokuRoomMapper::set_user_room(sender_id, &room_id).await;
        let manager: &RoomBroadcastManager = get_room_broadcast_manager();
        manager.subscribe_to_room(sender_id, &room_id).await;
        if room.get_status() == &GameStatus::Waiting && room.get_players().len() == 2 {
            GomokuDomain::start_game(&mut room)?;
        }
        GomokuRoomMapper::save_room(room.clone()).await;
        let resp_body: ResponseBody = Self::build_response_body(
            GomokuMessageType::RoomState,
            &room_id,
            sender_id,
            json!(room),
        )?;
        Ok((resp_body, room_id))
    }

    /// Joins a Gomoku room as a spectator to observe the game without participating.
    ///
    /// # Arguments
    ///
    /// - `&GomokuWsRequest`: The request containing the room identifier.
    /// - `&str`: The spectator's user identifier.
    ///
    /// # Returns
    ///
    /// - `Result<(ResponseBody, String), String>`: The room state response and room identifier, or an error if the room is not found or already joined.
    #[instrument_trace]
    async fn handle_spectate(
        req_data: &GomokuWsRequest,
        sender_id: &str,
    ) -> Result<(ResponseBody, String), String> {
        let room_id: String = req_data.get_room_id().clone();
        let mut room: GomokuRoom = GomokuRoomMapper::get_room(&room_id)
            .await
            .ok_or("Room not found".to_string())?;
        let added: bool = GomokuDomain::add_spectator(&mut room, sender_id);
        if !added {
            return Err(ERROR_ALREADY_IN_ROOM.to_string());
        }
        GomokuRoomMapper::set_user_room(sender_id, &room_id).await;
        let manager: &RoomBroadcastManager = get_room_broadcast_manager();
        manager.subscribe_to_room(sender_id, &room_id).await;
        GomokuRoomMapper::save_room(room.clone()).await;
        let resp_body: ResponseBody = Self::build_response_body(
            GomokuMessageType::RoomState,
            &room_id,
            sender_id,
            json!(room),
        )?;
        Ok((resp_body, room_id))
    }

    /// Removes a user from a Gomoku room, cleaning up the room if it becomes empty.
    ///
    /// # Arguments
    ///
    /// - `&GomokuWsRequest`: The request containing the room identifier.
    /// - `&str`: The leaving user's identifier.
    ///
    /// # Returns
    ///
    /// - `Result<(ResponseBody, String), String>`: The updated room state response and room identifier, or an error if the room or user is not found.
    #[instrument_trace]
    async fn handle_leave(
        req_data: &GomokuWsRequest,
        sender_id: &str,
    ) -> Result<(ResponseBody, String), String> {
        let room_id: String = if req_data.get_room_id().is_empty() {
            GomokuRoomMapper::get_user_room(sender_id)
                .await
                .unwrap_or_default()
        } else {
            req_data.get_room_id().clone()
        };
        if room_id.is_empty() {
            return Err("Room not found".to_string());
        }
        let mut room: GomokuRoom = GomokuRoomMapper::get_room(&room_id)
            .await
            .ok_or("Room not found".to_string())?;
        let removed: bool = GomokuDomain::remove_user(&mut room, sender_id);
        if !removed {
            return Err("User not in room".to_string());
        }
        let manager: &RoomBroadcastManager = get_room_broadcast_manager();
        manager.unsubscribe_user(sender_id).await;
        GomokuRoomMapper::remove_user_room(sender_id).await;
        if room.get_players().is_empty() && room.get_spectators().is_empty() {
            GomokuRoomMapper::remove_room(&room_id).await;
        } else {
            GomokuRoomMapper::save_room(room.clone()).await;
        }
        let resp_body: ResponseBody = Self::build_response_body(
            GomokuMessageType::RoomState,
            &room_id,
            sender_id,
            json!(room),
        )?;
        Ok((resp_body, room_id))
    }

    /// Places a stone on the Gomoku board at the specified position and checks for a win.
    ///
    /// # Arguments
    ///
    /// - `&GomokuWsRequest`: The request containing the room identifier and position payload.
    /// - `&str`: The player's user identifier.
    ///
    /// # Returns
    ///
    /// - `Result<(ResponseBody, String), String>`: The move result and room state response, or an error if the move is invalid.
    #[instrument_trace]
    async fn handle_place_stone(
        req_data: &GomokuWsRequest,
        sender_id: &str,
    ) -> Result<(ResponseBody, String), String> {
        let room_id: String = req_data.get_room_id().clone();
        let mut room: GomokuRoom = GomokuRoomMapper::get_room(&room_id)
            .await
            .ok_or("Room not found".to_string())?;
        let (x, y): (usize, usize) = Self::parse_position(req_data.get_payload())?;
        let result: GomokuPlaceResult = GomokuDomain::place_stone(&mut room, sender_id, x, y)?;
        GomokuRoomMapper::save_room(room.clone()).await;
        let payload: serde_json::Value = json!({
            "result": result,
            "room": room
        });
        let resp_body: ResponseBody =
            Self::build_response_body(GomokuMessageType::MoveResult, &room_id, sender_id, payload)?;
        Ok((resp_body, room_id))
    }

    /// Synchronizes the client with the current room state, ensuring the board is initialized.
    ///
    /// # Arguments
    ///
    /// - `&GomokuWsRequest`: The request containing the room identifier.
    /// - `&str`: The user's identifier.
    ///
    /// # Returns
    ///
    /// - `Result<(ResponseBody, String), String>`: The current room state response and room identifier, or an error if the room is not found.
    #[instrument_trace]
    async fn handle_sync(
        req_data: &GomokuWsRequest,
        sender_id: &str,
    ) -> Result<(ResponseBody, String), String> {
        let room_id: String = if req_data.get_room_id().is_empty() {
            GomokuRoomMapper::get_user_room(sender_id)
                .await
                .unwrap_or_default()
        } else {
            req_data.get_room_id().clone()
        };
        let mut room: GomokuRoom = GomokuRoomMapper::get_room(&room_id)
            .await
            .ok_or("Room not found".to_string())?;
        GomokuDomain::ensure_board(&mut room);
        GomokuRoomMapper::save_room(room.clone()).await;
        let resp_body: ResponseBody = Self::build_response_body(
            GomokuMessageType::RoomState,
            &room_id,
            sender_id,
            json!(room),
        )?;
        Ok((resp_body, room_id))
    }

    /// Extracts the (x, y) board position from the JSON payload of a place-stone request.
    ///
    /// # Arguments
    ///
    /// - `&serde_json::Value`: The payload containing "x" and "y" fields.
    ///
    /// # Returns
    ///
    /// - `Result<(usize, usize), String>`: The parsed coordinates on success, or an error message if values are missing or invalid.
    #[instrument_trace]
    fn parse_position(payload: &serde_json::Value) -> Result<(usize, usize), String> {
        let x: usize = payload
            .get("x")
            .and_then(|val: &serde_json::Value| val.as_u64())
            .ok_or("Invalid x".to_string())? as usize;
        let y: usize = payload
            .get("y")
            .and_then(|val: &serde_json::Value| val.as_u64())
            .ok_or("Invalid y".to_string())? as usize;
        Ok((x, y))
    }

    /// Constructs a serialized WebSocket response body with the given message type, room, sender, and payload.
    ///
    /// # Arguments
    ///
    /// - `GomokuMessageType`: The type of the response message.
    /// - `&str`: The room identifier.
    /// - `&str`: The sender's user identifier.
    /// - `serde_json::Value`: The JSON payload to include in the response.
    ///
    /// # Returns
    ///
    /// - `Result<ResponseBody, String>`: The serialized response body on success, or a serialization error message.
    #[instrument_trace]
    fn build_response_body(
        msg_type: GomokuMessageType,
        room_id: &str,
        sender_id: &str,
        payload: serde_json::Value,
    ) -> Result<ResponseBody, String> {
        let mut resp: GomokuWsResponse = GomokuWsResponse::default();
        resp.set_type(msg_type)
            .set_room_id(room_id.to_string())
            .set_sender_id(sender_id.to_string())
            .set_payload(payload)
            .set_time(Utc::now().timestamp_millis());
        serde_json::to_vec(&resp).map_err(|error: serde_json::Error| error.to_string())
    }

    /// Builds an error response body for a failed Gomoku WebSocket request.
    ///
    /// # Arguments
    ///
    /// - `&str`: The sender's user identifier.
    /// - `&GomokuWsRequest`: The original request that caused the error.
    /// - `String`: The error message to include in the response.
    ///
    /// # Returns
    ///
    /// - `ResponseBody`: The serialized error response body.
    #[instrument_trace]
    pub fn error_response(
        sender_id: &str,
        req_data: &GomokuWsRequest,
        error: String,
    ) -> ResponseBody {
        let payload: serde_json::Value = json!({ "message": error });
        Self::build_response_body(
            GomokuMessageType::Error,
            req_data.get_room_id(),
            sender_id,
            payload,
        )
        .unwrap_or_default()
    }

    /// Broadcasts a response to all users in a Gomoku room via WebSocket.
    ///
    /// # Arguments
    ///
    /// - `&str`: The identifier of the room to broadcast to.
    /// - `&str`: The sender's user identifier (for logging only).
    /// - `&ResponseBody`: The response body to send to each room member.
    #[instrument_trace]
    pub async fn broadcast_room(room_id: &str, sender_id: &str, resp_body: &ResponseBody) {
        let user_ids: Vec<String> = GomokuRoomMapper::get_room_user_ids(room_id).await;
        let websocket: &WebSocket = get_global_websocket();
        for user_id in user_ids {
            let key: BroadcastType<String> = BroadcastType::PointToGroup(user_id.clone());
            let _: Result<Option<ReceiverCount>, SendError<Vec<u8>>> =
                websocket.try_send(key, resp_body.clone());
        }
        trace!("Broadcasted message to room {room_id} from {sender_id}");
    }

    /// Generates a unique room identifier by combining the sender's identifier with the current timestamp.
    ///
    /// # Arguments
    ///
    /// - `&str`: The creator's user identifier.
    ///
    /// # Returns
    ///
    /// - `String`: A unique room identifier in the format "{sender_id}_{timestamp_millis}".
    #[instrument_trace]
    fn generate_room_id(sender_id: &str) -> String {
        let timestamp: i64 = Utc::now().timestamp_millis();
        format!("{sender_id}_{timestamp}")
    }
}
