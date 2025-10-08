use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct NetworkPacket {
    pub timestamp: u64,
    pub protocol: String,
    pub src_ip: String,
    pub dst_ip: String,
    pub src_port: u16,
    pub dst_port: u16,
    pub size: u32,
    pub direction: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Default)]
pub struct NetworkStats {
    pub total_packets: u64,
    pub total_bytes: u64,
    pub protocols: HashMap<String, u64>,
    pub top_connections: Vec<ConnectionInfo>,
    pub recent_packets: Vec<NetworkPacket>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ConnectionInfo {
    pub remote_ip: String,
    pub port: u16,
    pub protocol: String,
    pub packets: u64,
    pub bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct NetworkCaptureRequest {
    pub duration_seconds: Option<u64>,
    pub filter_protocol: Option<String>,
    pub filter_port: Option<u16>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct NetworkCaptureResponse {
    pub status: String,
    pub message: String,
    pub data: Option<NetworkStats>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, ToSchema)]
pub struct ServerStatus {
    pub timestamp: u64,
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub memory_total: u64,
    pub memory_used: u64,
    pub disk_usage: f64,
    pub disk_total: u64,
    pub disk_used: u64,
    pub network_rx: u64,
    pub network_tx: u64,
    pub uptime: u64,
    pub load_average: f64,
    pub active_connections: u32,
    pub process_count: u32,
    pub hostname: String,
    pub os_name: String,
    pub os_version: String,
    pub kernel_version: String,
    pub cpu_cores: u32,
    pub cpu_model: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, ToSchema)]
pub struct SystemInfo {
    pub hostname: String,
    pub os_name: String,
    pub os_version: String,
    pub kernel_version: String,
    pub cpu_cores: u32,
    pub cpu_model: String,
    pub total_memory: u64,
    pub total_disk: u64,
}
