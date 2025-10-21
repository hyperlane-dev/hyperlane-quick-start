use super::*;

impl Default for MessageType {
    fn default() -> Self {
        Self::Text
    }
}

impl MessageType {
    fn is_ping(&self) -> bool {
        matches!(self, MessageType::Ping)
    }
}

impl WebSocketReqData {
    pub fn new<T: ToString>(r#type: MessageType, data: T) -> Self {
        let mut resp_data: Self = Self::default();
        resp_data.set_type(r#type).set_data(data.to_string());
        resp_data
    }

    pub fn is_ping(&self) -> bool {
        self.get_type().is_ping()
    }

    pub async fn into_resp(&self, ctx: &Context) -> WebSocketRespData {
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
    pub async fn new<T: ToString>(r#type: MessageType, ctx: &Context, data: T) -> Self {
        let name: String = ChatService::get_name(ctx).await;
        let mut resp_data: Self = Self::default();
        resp_data
            .set_type(r#type)
            .set_data(data.to_string())
            .set_time(time());
        if r#type == MessageType::OnlineCount {
            resp_data.set_name("System".to_string());
        } else {
            resp_data.set_name(name.to_string());
        }
        resp_data
    }

    pub async fn get_json_data<T: ToString>(
        r#type: MessageType,
        ctx: &Context,
        data: T,
    ) -> ResultJsonError<ResponseBody> {
        serde_json::to_vec(&WebSocketRespData::new(r#type, ctx, data).await)
    }
}

impl ChatSession {
    pub fn is_expired(&self, timeout_minutes: u64) -> bool {
        self.get_last_activity().elapsed().as_secs() > timeout_minutes * 60
    }
}

impl ChatDomain {
    pub fn get_global_chat_sessions() -> &'static Arc<Mutex<HashMap<String, ChatSession>>> {
        GLOBAL_CHAT_SESSIONS.get_or_init(|| Arc::new(Mutex::new(HashMap::new())))
    }

    pub fn get_or_create_session(session_id: &str) -> ChatSession {
        let sessions: &Arc<Mutex<HashMap<String, ChatSession>>> = Self::get_global_chat_sessions();
        if let Ok(mut sessions_guard) = sessions.lock() {
            sessions_guard.retain(|_, session| !session.is_expired(30));
            sessions_guard
                .entry(session_id.to_string())
                .or_insert_with(|| {
                    let mut session = ChatSession::default();
                    session
                        .set_session_id(session_id.to_string())
                        .set_messages(Vec::new())
                        .set_last_activity(std::time::Instant::now());
                    session
                })
                .clone()
        } else {
            let mut session = ChatSession::default();
            session
                .set_session_id(session_id.to_string())
                .set_messages(Vec::new())
                .set_last_activity(std::time::Instant::now());
            session
        }
    }

    pub fn update_session(session: ChatSession) {
        let sessions: &Arc<Mutex<HashMap<String, ChatSession>>> = Self::get_global_chat_sessions();
        let mut sessions_guard = sessions.lock().unwrap();
        sessions_guard.insert(session.get_session_id().clone(), session);
    }

    pub fn get_global_online_users() -> &'static Arc<Mutex<HashMap<String, OnlineUser>>> {
        GLOBAL_ONLINE_USERS.get_or_init(|| Arc::new(Mutex::new(HashMap::new())))
    }

    pub fn add_online_user(username: &str) {
        let users: &Arc<Mutex<HashMap<String, OnlineUser>>> = Self::get_global_online_users();
        let mut users_guard: MutexGuard<'_, HashMap<String, OnlineUser>> = users.lock().unwrap();
        let mut online_user: OnlineUser = OnlineUser::default();
        online_user
            .set_username(username.to_string())
            .set_join_time(time());
        users_guard.insert(username.to_string(), online_user);
    }

    pub fn remove_online_user(username: &str) {
        let users: &Arc<Mutex<HashMap<String, OnlineUser>>> = Self::get_global_online_users();
        let mut users_guard: MutexGuard<'_, HashMap<String, OnlineUser>> = users.lock().unwrap();
        users_guard.remove(username);
    }

    pub fn get_online_users_list() -> UserListResponse {
        let users: &Arc<Mutex<HashMap<String, OnlineUser>>> = Self::get_global_online_users();
        let mut users_vec: Vec<OnlineUser> = if let Ok(users_guard) = users.lock() {
            users_guard.values().cloned().collect()
        } else {
            Vec::new()
        };
        let mut gpt_user: OnlineUser = OnlineUser::default();
        gpt_user.set_username(GPT.to_string()).set_join_time(time());
        users_vec.insert(0, gpt_user);
        let total_count: usize = users_vec.len();
        let mut response = UserListResponse::default();
        response.set_users(users_vec).set_total_count(total_count);
        response
    }
}
