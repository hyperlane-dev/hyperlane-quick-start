use super::*;

pub static NETWORK_CAPTURE_STATS: LazyLock<RwLock<Option<NetworkStats>>> =
    LazyLock::new(|| RwLock::new(None));
pub static CAPTURE_STATUS: LazyLock<RwLock<CaptureStatus>> =
    LazyLock::new(|| RwLock::new(CaptureStatus::Stopped));
pub static ACTIVE_CONNECTIONS: LazyLock<RwLock<HashMap<String, ConnectionInfo>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));
pub static PERFORMANCE_HISTORY: LazyLock<RwLock<PerformanceRingBuffer>> =
    LazyLock::new(|| RwLock::new(PerformanceRingBuffer::default()));
