use super::*;

#[derive(Debug, Default, Clone, Serialize, Deserialize, ToSchema, Data)]
pub struct NetworkPacket {
    timestamp: u64,
    protocol: String,
    src_ip: String,
    dst_ip: String,
    src_port: usize,
    dst_port: usize,
    size: u32,
    direction: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Default, Data)]
pub struct NetworkStats {
    total_packets: u64,
    total_bytes: u64,
    protocols: HashMap<String, u64>,
    top_connections: Vec<ConnectionInfo>,
    recent_packets: Vec<NetworkPacket>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, ToSchema, Data)]
pub struct ConnectionInfo {
    remote_ip: String,
    port: usize,
    protocol: String,
    packets: u64,
    bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Data)]
pub struct NetworkCaptureRequest {
    duration_seconds: Option<u64>,
    filter_protocol: Option<String>,
    filter_port: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema, Data)]
pub struct NetworkCaptureResponse {
    status: String,
    message: String,
    data: Option<NetworkStats>,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize, ToSchema, Data)]
pub struct ServerStatus {
    timestamp: u64,
    cpu_usage: f64,
    memory_usage: f64,
    memory_total: u64,
    memory_used: u64,
    disk_usage: f64,
    disk_total: u64,
    disk_used: u64,
    network_rx: u64,
    network_tx: u64,
    uptime: u64,
    load_average: f64,
    active_connections: u32,
    process_count: u32,
    hostname: String,
    os_name: String,
    os_version: String,
    kernel_version: String,
    cpu_cores: u32,
    cpu_model: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, ToSchema, Data)]
pub struct SystemInfo {
    hostname: String,
    os_name: String,
    os_version: String,
    kernel_version: String,
    cpu_cores: u32,
    cpu_model: String,
    total_memory: u64,
    total_disk: u64,
}
