use super::*;

pub fn get_global_chat_sessions()
-> &'static std::sync::Arc<std::sync::Mutex<std::collections::HashMap<String, ChatSession>>> {
    GLOBAL_CHAT_SESSIONS.get_or_init(|| {
        std::sync::Arc::new(std::sync::Mutex::new(std::collections::HashMap::new()))
    })
}

pub fn get_or_create_session(session_id: &str) -> ChatSession {
    let sessions = get_global_chat_sessions();
    let mut sessions_guard = sessions.lock().unwrap();

    // 清理过期的会话（超过30分钟）
    sessions_guard.retain(|_, session| !session.is_expired(30));

    // 获取或创建会话
    sessions_guard
        .entry(session_id.to_string())
        .or_insert_with(|| ChatSession::new(session_id.to_string()))
        .clone()
}

pub fn update_session(session: ChatSession) {
    let sessions = get_global_chat_sessions();
    let mut sessions_guard = sessions.lock().unwrap();
    sessions_guard.insert(session.session_id.clone(), session);
}

pub fn get_global_online_users()
-> &'static std::sync::Arc<std::sync::Mutex<std::collections::HashMap<String, OnlineUser>>> {
    GLOBAL_ONLINE_USERS.get_or_init(|| {
        std::sync::Arc::new(std::sync::Mutex::new(std::collections::HashMap::new()))
    })
}

pub fn add_online_user(user_id: &str, username: &str) {
    let users = get_global_online_users();
    let mut users_guard = users.lock().unwrap();

    let online_user = OnlineUser {
        user_id: user_id.to_string(),
        username: username.to_string(),
        join_time: time(),
    };

    users_guard.insert(user_id.to_string(), online_user);
}

pub fn remove_online_user(user_id: &str) {
    let users = get_global_online_users();
    let mut users_guard = users.lock().unwrap();
    users_guard.remove(user_id);
}

pub fn get_online_users_list() -> UserListResponse {
    let users = get_global_online_users();
    let users_guard = users.lock().unwrap();

    let users_vec: Vec<OnlineUser> = users_guard.values().cloned().collect();
    let total_count = users_vec.len();

    UserListResponse {
        users: users_vec,
        total_count,
    }
}

pub async fn get_name(ctx: &Context) -> String {
    ctx.get_request_query("uuid").await.unwrap_or_default()
}

pub fn get_global_env_config() -> &'static EnvConfig {
    GLOBAL_ENV_CONFIG.get_or_init(|| EnvConfig::default())
}

pub fn init_env_config() -> Result<(), String> {
    let config: EnvConfig = EnvConfig::load()?;
    GLOBAL_ENV_CONFIG
        .set(config)
        .map_err(|_| "Failed to initialize global environment configuration".to_string())?;
    Ok(())
}
