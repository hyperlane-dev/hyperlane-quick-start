use super::*;

pub fn get_global_chat_sessions() -> &'static Arc<Mutex<HashMap<String, ChatSession>>> {
    GLOBAL_CHAT_SESSIONS.get_or_init(|| Arc::new(Mutex::new(HashMap::new())))
}

pub fn get_or_create_session(session_id: &str) -> ChatSession {
    let sessions: &Arc<Mutex<HashMap<String, ChatSession>>> = get_global_chat_sessions();
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
    let sessions: &Arc<Mutex<HashMap<String, ChatSession>>> = get_global_chat_sessions();
    let mut sessions_guard = sessions.lock().unwrap();
    sessions_guard.insert(session.get_session_id().clone(), session);
}

pub fn get_global_online_users() -> &'static Arc<Mutex<HashMap<String, OnlineUser>>> {
    GLOBAL_ONLINE_USERS.get_or_init(|| Arc::new(Mutex::new(HashMap::new())))
}

pub fn add_online_user(username: &str) {
    let users: &Arc<Mutex<HashMap<String, OnlineUser>>> = get_global_online_users();
    let mut users_guard: MutexGuard<'_, HashMap<String, OnlineUser>> = users.lock().unwrap();
    let mut online_user: OnlineUser = OnlineUser::default();
    online_user
        .set_username(username.to_string())
        .set_join_time(time());
    users_guard.insert(username.to_string(), online_user);
}

pub fn remove_online_user(username: &str) {
    let users: &Arc<Mutex<HashMap<String, OnlineUser>>> = get_global_online_users();
    let mut users_guard: MutexGuard<'_, HashMap<String, OnlineUser>> = users.lock().unwrap();
    users_guard.remove(username);
}

pub fn get_online_users_list() -> UserListResponse {
    let users: &Arc<Mutex<HashMap<String, OnlineUser>>> = get_global_online_users();
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

#[request_query("uuid" => uuid_opt)]
pub async fn get_name(ctx: &Context) -> String {
    uuid_opt.unwrap_or_default()
}
