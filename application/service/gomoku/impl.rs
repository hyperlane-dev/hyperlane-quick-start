use super::*;

impl RoomBroadcastManager {
    #[instrument_trace]
    pub fn new() -> Self {
        Self {
            broadcast_map: Arc::new(BroadcastMap::new()),
            user_subscriptions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

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
            .subscribe_or_insert(room_id.to_string(), 128);
        subs.insert(user_id.to_string(), room_id.to_string());
        trace!("User {user_id} subscribed to room {room_id}");
    }

    #[instrument_trace]
    pub async fn unsubscribe_user(&self, user_id: &str) {
        let mut subs: RwLockWriteGuard<'_, HashMap<String, String>> =
            self.user_subscriptions.write().await;
        if let Some(room_id) = subs.remove(user_id) {
            trace!("User {user_id} unsubscribed from room {room_id}");
        }
    }

    #[instrument_trace]
    pub fn broadcast_to_room(&self, room_id: &str, message: &str) {
        if self
            .broadcast_map
            .send(room_id.to_string(), message.to_string())
            .is_err()
        {
            trace!("Failed to broadcast to room {room_id}: no active receivers");
        } else {
            trace!("Broadcasted message to room {room_id}");
        }
    }

    #[instrument_trace]
    pub async fn get_user_room(&self, user_id: &str) -> Option<String> {
        let subs: RwLockReadGuard<'_, HashMap<String, String>> =
            self.user_subscriptions.read().await;
        subs.get(user_id).cloned()
    }
}

impl Default for RoomBroadcastManager {
    #[instrument_trace]
    fn default() -> Self {
        Self::new()
    }
}

impl ServerHook for GomokuConnectedHook {
    #[instrument_trace]
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[instrument_trace]
    async fn handle(self, ctx: &Context) {
        let user_id: String = GomokuWebSocketService::get_user_id(ctx).await;
        if user_id.is_empty() {
            return;
        }
        if let Some(room_id) = GomokuRoomMapper::get_user_room(&user_id).await {
            let manager: &RoomBroadcastManager = get_room_broadcast_manager();
            manager.subscribe_to_room(&user_id, &room_id).await;
            trace!("User {user_id} connected and subscribed to room {room_id}");
            if let Some(room) = GomokuRoomMapper::get_room(&room_id).await {
                let resp_body: ResponseBody = GomokuWebSocketService::build_response_body(
                    GomokuMessageType::RoomState,
                    &room_id,
                    &user_id,
                    json!(room),
                )
                .unwrap_or_default();
                ctx.set_response_body(&resp_body).await;
            }
        }
    }
}

impl ServerHook for GomokuRequestHook {
    #[instrument_trace]
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[request_body_json_result(req_data_res: GomokuWsRequest)]
    #[instrument_trace]
    async fn handle(self, ctx: &Context) {
        let req_data: GomokuWsRequest = req_data_res.unwrap();
        if GomokuWebSocketService::handle_ping_request(ctx, &req_data).await {
            return;
        }
        let sender_id: String = GomokuWebSocketService::get_user_id(ctx).await;
        match GomokuWebSocketService::handle_request(ctx, &req_data, &sender_id).await {
            Ok((resp_body, room_id)) => {
                ctx.set_response_body(&resp_body).await;
                if !room_id.is_empty() {
                    GomokuWebSocketService::broadcast_room(&room_id, &sender_id, &resp_body).await;
                }
            }
            Err(error) => {
                let resp_body: ResponseBody =
                    GomokuWebSocketService::error_response(&sender_id, &req_data, error);
                ctx.set_response_body(&resp_body).await;
            }
        }
    }
}

impl ServerHook for GomokuSendedHook {
    #[instrument_trace]
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[instrument_trace]
    async fn handle(self, _ctx: &Context) {}
}

impl ServerHook for GomokuClosedHook {
    #[instrument_trace]
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[instrument_trace]
    async fn handle(self, ctx: &Context) {
        let user_id: String = GomokuWebSocketService::get_user_id(ctx).await;
        if user_id.is_empty() {
            return;
        }
        let manager: &RoomBroadcastManager = get_room_broadcast_manager();
        manager.unsubscribe_user(&user_id).await;
        trace!("User {user_id} disconnected, unsubscribed from WebSocket only");
    }
}

impl GomokuWebSocketService {
    #[instrument_trace]
    pub async fn get_user_id(ctx: &Context) -> String {
        #[request_query_option("uid" => uid_opt)]
        async fn get_uid(_ctx: &Context) -> Option<String> {
            uid_opt
        }
        #[request_query_option("user_id" => user_id_opt)]
        async fn get_user_id(_ctx: &Context) -> Option<String> {
            user_id_opt
        }
        let uid: Option<String> = get_uid(ctx).await;
        let user_id: Option<String> = get_user_id(ctx).await;
        uid.or(user_id).unwrap_or_default()
    }

    #[instrument_trace]
    pub async fn handle_ping_request(ctx: &Context, req_data: &GomokuWsRequest) -> bool {
        if req_data.get_type() == &GomokuMessageType::Ping {
            let sender_id: String = Self::get_user_id(ctx).await;
            let resp_body: ResponseBody = Self::build_response_body(
                GomokuMessageType::Pang,
                req_data.get_room_id(),
                &sender_id,
                json!({}),
            )
            .unwrap_or_default();
            ctx.set_response_body(&resp_body).await;
            return true;
        }
        false
    }

    #[instrument_trace]
    pub async fn handle_request(
        _ctx: &Context,
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
            return Err("Room already exists".to_string());
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
            return Err("Already in room".to_string());
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

    #[instrument_trace]
    fn parse_position(payload: &serde_json::Value) -> Result<(usize, usize), String> {
        let x: usize = payload
            .get("x")
            .and_then(|val| val.as_u64())
            .ok_or("Invalid x".to_string())? as usize;
        let y: usize = payload
            .get("y")
            .and_then(|val| val.as_u64())
            .ok_or("Invalid y".to_string())? as usize;
        Ok((x, y))
    }

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
            .set_time(time());
        serde_json::to_vec(&resp).map_err(|error| error.to_string())
    }

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

    #[instrument_trace]
    pub async fn broadcast_room(room_id: &str, sender_id: &str, resp_body: &ResponseBody) {
        let user_ids: Vec<String> = GomokuRoomMapper::get_room_user_ids(room_id).await;
        let websocket: &WebSocket = get_global_websocket();
        for user_id in user_ids {
            let key: BroadcastType<String> = BroadcastType::PointToGroup(user_id.clone());
            let _: Result<Option<ReceiverCount>, SendError<Vec<u8>>> =
                websocket.send(key, resp_body.clone());
        }
        trace!("Broadcasted message to room {room_id} from {sender_id}");
    }

    #[instrument_trace]
    fn generate_room_id(sender_id: &str) -> String {
        let timestamp: i64 = Utc::now().timestamp_millis();
        format!("{sender_id}_{timestamp}")
    }
}
