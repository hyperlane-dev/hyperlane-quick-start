use super::*;

/// Global singleton for storing captured network statistics.
pub static NETWORK_CAPTURE_STATS: OnceLock<RwLock<Option<NetworkStats>>> = OnceLock::new();
/// Global singleton for tracking the current network capture status.
pub static CAPTURE_STATUS: OnceLock<RwLock<CaptureStatus>> = OnceLock::new();
/// Global singleton for storing active network connection information.
pub static ACTIVE_CONNECTIONS: OnceLock<RwLock<HashMap<String, ConnectionInfo>>> = OnceLock::new();
/// Global singleton for storing historical performance data points in a ring buffer.
pub static PERFORMANCE_HISTORY: OnceLock<RwLock<PerformanceRingBuffer>> = OnceLock::new();
