use super::*;

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct NetworkPacket {
    #[get(type(copy), pub)]
    pub(super) timestamp: u64,
    pub(super) protocol: String,
    pub(super) src_ip: String,
    pub(super) dst_ip: String,
    #[get(type(copy), pub)]
    pub(super) src_port: usize,
    #[get(type(copy), pub)]
    pub(super) dst_port: usize,
    #[get(type(copy), pub)]
    pub(super) size: u32,
    pub(super) direction: String,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct NetworkStats {
    #[get(type(copy), pub)]
    pub(super) total_packets: u64,
    #[get(type(copy), pub)]
    pub(super) total_bytes: u64,
    pub(super) protocols: HashMap<String, u64>,
    pub(super) top_connections: Vec<ConnectionInfo>,
    pub(super) recent_packets: Vec<NetworkPacket>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct ConnectionInfo {
    pub(super) remote_ip: String,
    #[get(type(copy), pub)]
    pub(super) port: usize,
    pub(super) protocol: String,
    #[get(type(copy), pub)]
    pub(super) packets: u64,
    #[get(type(copy), pub)]
    pub(super) bytes: u64,
}

#[derive(Clone, Data, Debug, Deserialize, Serialize, ToSchema)]
pub struct NetworkCaptureRequest {
    pub(super) duration_seconds: Option<u64>,
    pub(super) filter_protocol: Option<String>,
    pub(super) filter_port: Option<usize>,
}

#[derive(Clone, Data, Debug, Deserialize, Serialize, ToSchema)]
pub struct NetworkCaptureResponse {
    pub(super) status: String,
    pub(super) message: String,
    pub(super) data: Option<NetworkStats>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct ServerStatus {
    #[get(type(copy), pub)]
    pub(super) timestamp: u64,
    #[get(type(copy), pub)]
    pub(super) cpu_usage: f64,
    #[get(type(copy), pub)]
    pub(super) memory_usage: f64,
    #[get(type(copy), pub)]
    pub(super) memory_total: u64,
    #[get(type(copy), pub)]
    pub(super) memory_used: u64,
    #[get(type(copy), pub)]
    pub(super) disk_usage: f64,
    #[get(type(copy), pub)]
    pub(super) disk_total: u64,
    #[get(type(copy), pub)]
    pub(super) disk_used: u64,
    #[get(type(copy), pub)]
    pub(super) network_rx: u64,
    #[get(type(copy), pub)]
    pub(super) network_tx: u64,
    #[get(type(copy), pub)]
    pub(super) uptime: u64,
    #[get(type(copy), pub)]
    pub(super) load_average: f64,
    #[get(type(copy), pub)]
    pub(super) active_connections: u32,
    #[get(type(copy), pub)]
    pub(super) process_count: u32,
    pub(super) hostname: String,
    pub(super) os_name: String,
    pub(super) os_version: String,
    pub(super) kernel_version: String,
    #[get(type(copy), pub)]
    pub(super) cpu_cores: u32,
    pub(super) cpu_model: String,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct SystemInfo {
    pub(super) hostname: String,
    pub(super) os_name: String,
    pub(super) os_version: String,
    pub(super) kernel_version: String,
    #[get(type(copy), pub)]
    pub(super) cpu_cores: u32,
    pub(super) cpu_model: String,
    #[get(type(copy), pub)]
    pub(super) total_memory: u64,
    #[get(type(copy), pub)]
    pub(super) total_disk: u64,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct PerformanceDataPoint {
    #[get(type(copy), pub)]
    pub(super) timestamp: u64,
    #[get(type(copy), pub)]
    pub(super) cpu_usage: f64,
    #[get(type(copy), pub)]
    pub(super) memory_usage: f64,
    #[get(type(copy), pub)]
    pub(super) memory_used: u64,
    #[get(type(copy), pub)]
    pub(super) disk_usage: f64,
    #[get(type(copy), pub)]
    pub(super) disk_used: u64,
    #[get(type(copy), pub)]
    pub(super) network_rx: u64,
    #[get(type(copy), pub)]
    pub(super) network_tx: u64,
    #[get(type(copy), pub)]
    pub(super) load_average: f64,
    #[get(type(copy), pub)]
    pub(super) active_connections: u32,
    #[get(type(copy), pub)]
    pub(super) process_count: u32,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct PerformanceHistoryResponse {
    pub(super) data_points: Vec<PerformanceDataPoint>,
    #[get(type(copy), pub)]
    pub(super) total_points: usize,
    #[get(type(copy), pub)]
    pub(super) time_range_seconds: u64,
}
