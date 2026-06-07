use super::*;

/// Default implementation for `MessageType`, defaulting to `Text`.
impl Default for MessageType {
    #[instrument_trace]
    fn default() -> Self {
        Self::Text
    }
}

/// Type-checking methods for `MessageType`.
impl MessageType {
    /// Checks whether this message type is a ping.
    ///
    /// # Returns
    ///
    /// - `bool`: `true` if the type is `Ping`.
    #[instrument_trace]
    fn is_ping(&self) -> bool {
        matches!(self, MessageType::Ping)
    }
}

/// Request data transformation methods for `WebSocketReqData`.
impl WebSocketReqData {
    /// Checks whether this request is a ping message.
    ///
    /// # Returns
    ///
    /// - `bool`: `true` if the request type is `Ping`.
    #[instrument_trace]
    pub fn is_ping(&self) -> bool {
        self.get_type().is_ping()
    }

    /// Converts this request data into a response by extracting the user UUID from the context.
    ///
    /// # Arguments
    ///
    /// - `&mut Stream`: The WebSocket stream (unused).
    /// - `&mut Context`: The request context used to extract the UUID query parameter.
    ///
    /// # Returns
    ///
    /// - `WebSocketRespData`: The constructed response data.
    #[instrument_trace]
    pub async fn into_resp(&self, _stream: &mut Stream, ctx: &mut Context) -> WebSocketRespData {
        let uuid_opt: Option<RequestQuerysValue> = ctx.get_request().try_get_query("uuid");
        let uuid: String = uuid_opt.unwrap_or_default();
        let mut resp: WebSocketRespData = WebSocketRespData::default();
        resp.set_type(self.get_type())
            .set_name(uuid)
            .set_data(self.get_data().clone())
            .set_time(Utc::now().timestamp_millis());
        resp
    }
}

/// Response data construction methods for `WebSocketRespData`.
impl WebSocketRespData {
    /// Creates a `WebSocketRespData` from a message type, context, and serializable data.
    ///
    /// # Arguments
    ///
    /// - `MessageType`: The type of the response message.
    /// - `&mut Stream`: The WebSocket stream (unused).
    /// - `&mut Context`: The request context used to extract the UUID.
    /// - `T`: The data to include, implementing `ToString`.
    ///
    /// # Returns
    ///
    /// - `WebSocketRespData`: The constructed response data.
    #[instrument_trace]
    pub async fn from<T: ToString>(
        msg_type: MessageType,
        _stream: &mut Stream,
        ctx: &mut Context,
        data: T,
    ) -> Self {
        let uuid_opt: Option<RequestQuerysValue> = ctx.get_request().try_get_query("uuid");
        let uuid: String = uuid_opt.unwrap_or_default();
        let mut resp_data: Self = Self::default();
        resp_data
            .set_type(msg_type)
            .set_data(data.to_string())
            .set_time(Utc::now().timestamp_millis());
        if matches!(msg_type, MessageType::System | MessageType::OnlineCount) {
            resp_data.set_name("System".to_string());
        } else {
            resp_data.set_name(uuid.to_string());
        }
        resp_data
    }

    /// Serializes a `WebSocketRespData` into a JSON response body.
    ///
    /// # Arguments
    ///
    /// - `MessageType`: The type of the response message.
    /// - `&mut Stream`: The WebSocket stream.
    /// - `&mut Context`: The request context.
    /// - `T`: The data to serialize, implementing `ToString`.
    ///
    /// # Returns
    ///
    /// - `serde_json::Result<ResponseBody>`: The serialized response body, or a serialization error.
    #[instrument_trace]
    pub async fn get_json_data<T: ToString>(
        msg_type: MessageType,
        stream: &mut Stream,
        ctx: &mut Context,
        data: T,
    ) -> serde_json::Result<ResponseBody> {
        serde_json::to_vec(&WebSocketRespData::from(msg_type, stream, ctx, data).await)
    }
}

/// Session expiry check for `ChatSession`.
impl ChatSession {
    /// Checks whether this session has expired based on the given timeout.
    ///
    /// # Arguments
    ///
    /// - `u64`: The timeout duration in minutes.
    ///
    /// # Returns
    ///
    /// - `bool`: `true` if the session has been inactive longer than the timeout.
    #[instrument_trace]
    pub fn is_expired(&self, timeout_minutes: u64) -> bool {
        self.get_last_activity().elapsed().as_secs() > timeout_minutes * 60
    }
}

/// Chat domain management methods for `ChatDomain`.
impl ChatDomain {
    /// Returns a static reference to the global chat sessions map, initializing it lazily.
    ///
    /// # Returns
    ///
    /// - `&'static ArcRwLock<HashMap<String, ChatSession>>`: The global sessions map.
    #[instrument_trace]
    pub fn get_global_chat_sessions() -> &'static ArcRwLock<HashMap<String, ChatSession>> {
        GLOBAL_CHAT_SESSIONS.get_or_init(|| arc_rwlock(HashMap::new()))
    }

    /// Retrieves an existing session or creates a new one for the given session ID,
    /// cleaning up expired sessions (30-minute timeout) in the process.
    ///
    /// # Arguments
    ///
    /// - `&str`: The session identifier.
    ///
    /// # Returns
    ///
    /// - `ChatSession`: The existing or newly created session.
    #[instrument_trace]
    pub async fn get_or_create_session(session_id: &str) -> ChatSession {
        let sessions: &ArcRwLock<HashMap<String, ChatSession>> = Self::get_global_chat_sessions();
        let mut sessions_guard: RwLockWriteGuard<'_, HashMap<String, ChatSession>> =
            sessions.write().await;
        sessions_guard.retain(|_: &String, session: &mut ChatSession| !session.is_expired(30));
        sessions_guard
            .entry(session_id.to_string())
            .or_insert_with(|| {
                let mut session: ChatSession = ChatSession::default();
                session
                    .set_session_id(session_id.to_string())
                    .set_messages(vec![])
                    .set_last_activity(std::time::Instant::now());
                session
            })
            .clone()
    }

    /// Updates or inserts a chat session into the global sessions map.
    ///
    /// # Arguments
    ///
    /// - `ChatSession`: The session to update.
    #[instrument_trace]
    pub async fn update_session(session: ChatSession) {
        Self::get_global_chat_sessions()
            .write()
            .await
            .insert(session.get_session_id().clone(), session);
    }

    /// Returns a static reference to the global online users map, initializing it lazily.
    ///
    /// # Returns
    ///
    /// - `&'static ArcRwLock<HashMap<String, OnlineUser>>`: The global online users map.
    #[instrument_trace]
    pub fn get_global_online_users() -> &'static ArcRwLock<HashMap<String, OnlineUser>> {
        GLOBAL_ONLINE_USERS.get_or_init(|| arc_rwlock(HashMap::new()))
    }

    /// Adds a user to the global online users map.
    ///
    /// # Arguments
    ///
    /// - `&str`: The username of the user to add.
    #[instrument_trace]
    pub async fn add_online_user(username: &str) {
        let mut online_user: OnlineUser = OnlineUser::default();
        online_user
            .set_username(username.to_string())
            .set_join_time(timestamp_millis() as i64);
        Self::get_global_online_users()
            .write()
            .await
            .insert(username.to_string(), online_user);
    }

    /// Removes a user from the global online users map.
    ///
    /// # Arguments
    ///
    /// - `&str`: The username of the user to remove.
    #[instrument_trace]
    pub async fn remove_online_user(username: &str) {
        Self::get_global_online_users()
            .write()
            .await
            .remove(username);
    }

    /// Returns the list of all online users, including the GPT system user.
    ///
    /// # Returns
    ///
    /// - `UserListResponse`: The response containing the user list and total count.
    #[instrument_trace]
    pub async fn get_online_users_list() -> UserListResponse {
        let mut users_vec: Vec<OnlineUser> = Self::get_global_online_users()
            .read()
            .await
            .values()
            .cloned()
            .collect();
        let mut gpt_user: OnlineUser = OnlineUser::default();
        gpt_user
            .set_username(GPT.to_string())
            .set_join_time(timestamp_millis() as i64);
        users_vec.insert(0, gpt_user);
        let total_count: usize = users_vec.len();
        let mut response: UserListResponse = UserListResponse::default();
        response.set_users(users_vec).set_total_count(total_count);
        response
    }
}
