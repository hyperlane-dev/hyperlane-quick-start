use super::*;

pub static NETWORK_CAPTURE_STATS: OnceLock<RwLock<Option<NetworkStats>>> = OnceLock::new();
pub static CAPTURE_STATUS: OnceLock<RwLock<CaptureStatus>> = OnceLock::new();
pub static ACTIVE_CONNECTIONS: OnceLock<RwLock<HashMap<String, ConnectionInfo>>> = OnceLock::new();
pub static PERFORMANCE_HISTORY: OnceLock<RwLock<PerformanceRingBuffer>> = OnceLock::new();
