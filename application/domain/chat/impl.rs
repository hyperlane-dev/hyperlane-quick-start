use super::*;

impl Default for MessageType {
    #[instrument_trace]
    fn default() -> Self {
        Self::Text
    }
}

impl MessageType {
    #[instrument_trace]
    fn is_ping(&self) -> bool {
        matches!(self, MessageType::Ping)
    }
}

impl WebSocketReqData {
    #[instrument_trace]
    pub fn is_ping(&self) -> bool {
        self.get_type().is_ping()
    }

    #[instrument_trace]
    pub async fn into_resp(&self, ctx: &mut Context) -> WebSocketRespData {
        let name: String = ChatService::get_name(ctx).await;
        let mut resp: WebSocketRespData = WebSocketRespData::default();
        resp.set_type(*self.get_type())
            .set_name(name)
            .set_data(self.get_data().clone())
            .set_time(time());
        resp
    }
}

impl WebSocketRespData {
    #[instrument_trace]
    pub async fn from<T: ToString>(msg_type: MessageType, ctx: &mut Context, data: T) -> Self {
        let name: String = ChatService::get_name(ctx).await;
        let mut resp_data: Self = Self::default();
        resp_data
            .set_type(msg_type)
            .set_data(data.to_string())
            .set_time(time());
        if msg_type == MessageType::OnlineCount {
            resp_data.set_name("System".to_string());
        } else {
            resp_data.set_name(name.to_string());
        }
        resp_data
    }

    #[instrument_trace]
    pub async fn get_json_data<T: ToString>(
        msg_type: MessageType,
        ctx: &mut Context,
        data: T,
    ) -> serde_json::Result<ResponseBody> {
        serde_json::to_vec(&WebSocketRespData::from(msg_type, ctx, data).await)
    }
}

impl ChatSession {
    #[instrument_trace]
    pub fn is_expired(&self, timeout_minutes: u64) -> bool {
        self.get_last_activity().elapsed().as_secs() > timeout_minutes * 60
    }
}

impl ChatDomain {
    #[instrument_trace]
    pub fn get_global_chat_sessions() -> &'static ArcRwLock<HashMap<String, ChatSession>> {
        GLOBAL_CHAT_SESSIONS.get_or_init(|| arc_rwlock(HashMap::new()))
    }

    #[instrument_trace]
    pub async fn get_or_create_session(session_id: &str) -> ChatSession {
        let sessions: &ArcRwLock<HashMap<String, ChatSession>> = Self::get_global_chat_sessions();
        let mut sessions_guard = sessions.write().await;
        sessions_guard.retain(|_, session| !session.is_expired(30));
        sessions_guard
            .entry(session_id.to_string())
            .or_insert_with(|| {
                let mut session: ChatSession = ChatSession::default();
                session
                    .set_session_id(session_id.to_string())
                    .set_messages(Vec::new())
                    .set_last_activity(std::time::Instant::now());
                session
            })
            .clone()
    }

    #[instrument_trace]
    pub async fn update_session(session: ChatSession) {
        Self::get_global_chat_sessions()
            .write()
            .await
            .insert(session.get_session_id().clone(), session);
    }

    #[instrument_trace]
    pub fn get_global_online_users() -> &'static ArcRwLock<HashMap<String, OnlineUser>> {
        GLOBAL_ONLINE_USERS.get_or_init(|| arc_rwlock(HashMap::new()))
    }

    #[instrument_trace]
    pub async fn add_online_user(username: &str) {
        let mut online_user: OnlineUser = OnlineUser::default();
        online_user
            .set_username(username.to_string())
            .set_join_time(time());
        Self::get_global_online_users()
            .write()
            .await
            .insert(username.to_string(), online_user);
    }

    #[instrument_trace]
    pub async fn remove_online_user(username: &str) {
        Self::get_global_online_users()
            .write()
            .await
            .remove(username);
    }

    #[instrument_trace]
    pub async fn get_online_users_list() -> UserListResponse {
        let mut users_vec: Vec<OnlineUser> = Self::get_global_online_users()
            .read()
            .await
            .values()
            .cloned()
            .collect();
        let mut gpt_user: OnlineUser = OnlineUser::default();
        gpt_user.set_username(GPT.to_string()).set_join_time(time());
        users_vec.insert(0, gpt_user);
        let total_count: usize = users_vec.len();
        let mut response: UserListResponse = UserListResponse::default();
        response.set_users(users_vec).set_total_count(total_count);
        response
    }
}
