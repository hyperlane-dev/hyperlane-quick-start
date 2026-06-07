use super::*;

/// Utility for hashing passwords with bcrypt and verifying password hashes.
#[derive(Clone, Data, Debug, Default)]
pub struct PasswordUtil;

/// Cache entry storing the RSA public key JWK response and its creation timestamp.
#[derive(Clone, Data, Debug)]
pub struct RsaKeyCache {
    /// The JSON-serialized RSA public key response string.
    pub(super) response_json: String,
    /// The timestamp when this cache entry was created, used for TTL expiration.
    pub(super) created_at: Instant,
}

/// Authentication service managing RSA key pairs, user registration, login, and account operations.
#[derive(Clone, Data, Debug)]
pub struct AuthService {
    /// The currently loaded RSA private key, stored behind a read-write lock for concurrent access.
    pub(super) rsa_private_key: Arc<RwLock<Option<RsaPrivateKey>>>,
    /// The cached RSA public key response, stored behind a read-write lock for concurrent access.
    pub(super) rsa_key_cache: Arc<RwLock<Option<RsaKeyCache>>>,
}
