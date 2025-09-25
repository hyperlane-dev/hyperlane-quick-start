use super::*;
use std::sync::Arc;

pub struct AuthService {
    pub user_repository: Arc<dyn UserRepository>,
    pub session_manager: Arc<SessionManager>,
}
