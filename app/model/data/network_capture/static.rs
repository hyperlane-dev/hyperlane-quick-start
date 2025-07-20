use super::*;

pub static NETWORK_CAPTURE_STATS: OnceLock<Arc<Mutex<Option<NetworkStats>>>> = OnceLock::new();
pub static CAPTURE_STATUS: OnceLock<Arc<Mutex<CaptureStatus>>> = OnceLock::new();
pub static ACTIVE_CONNECTIONS: OnceLock<Arc<Mutex<HashMap<String, ConnectionInfo>>>> =
    OnceLock::new();
