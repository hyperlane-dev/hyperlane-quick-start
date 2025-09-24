use super::*;
use std::sync::Arc;

pub struct AuthController {
    pub auth_service: Arc<AuthService>,
}