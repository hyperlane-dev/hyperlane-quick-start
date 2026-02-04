use super::*;

pub static NETWORK_CAPTURE_STATS: OnceLock<Arc<RwLock<Option<NetworkStats>>>> = OnceLock::new();
pub static CAPTURE_STATUS: OnceLock<Arc<RwLock<CaptureStatus>>> = OnceLock::new();
pub static ACTIVE_CONNECTIONS: OnceLock<Arc<RwLock<HashMap<String, ConnectionInfo>>>> =
    OnceLock::new();
