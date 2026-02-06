use super::*;

pub static NETWORK_CAPTURE_STATS: OnceLock<ArcRwLock<Option<NetworkStats>>> = OnceLock::new();
pub static CAPTURE_STATUS: OnceLock<ArcRwLock<CaptureStatus>> = OnceLock::new();
pub static ACTIVE_CONNECTIONS: OnceLock<ArcRwLock<HashMap<String, ConnectionInfo>>> =
    OnceLock::new();
