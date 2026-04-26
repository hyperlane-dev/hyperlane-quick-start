use super::*;

#[derive(Clone, Data, Debug, Default)]
pub struct PasswordUtil;

#[derive(Clone, Data, Debug)]
pub struct RsaKeyCache {
    pub(super) response_json: String,
    pub(super) created_at: Instant,
}

#[derive(Clone, Data, Debug)]
pub struct AuthService {
    pub(super) rsa_private_key: Arc<RwLock<Option<RsaPrivateKey>>>,
    pub(super) rsa_key_cache: Arc<RwLock<Option<RsaKeyCache>>>,
}
