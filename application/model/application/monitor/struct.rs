use super::*;

/// Represents a captured network packet with source/destination information and metadata.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct NetworkPacket {
    /// The timestamp (in milliseconds) when the packet was captured.
    #[get(type(copy))]
    pub(super) timestamp: u64,
    /// The network protocol of the packet (e.g., "TCP", "UDP", "HTTP").
    pub(super) protocol: String,
    /// The source IP address of the packet.
    pub(super) src_ip: String,
    /// The destination IP address of the packet.
    pub(super) dst_ip: String,
    /// The source port number of the packet.
    #[get(type(copy))]
    pub(super) src_port: usize,
    /// The destination port number of the packet.
    #[get(type(copy))]
    pub(super) dst_port: usize,
    /// The size of the packet in bytes.
    #[get(type(copy))]
    pub(super) size: u32,
    /// The direction of the packet relative to the host ("inbound" or "outbound").
    pub(super) direction: String,
}

/// Aggregated network traffic statistics including protocol breakdown and top connections.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct NetworkStats {
    /// The total number of packets captured.
    #[get(type(copy))]
    pub(super) total_packets: u64,
    /// The total number of bytes transferred.
    #[get(type(copy))]
    pub(super) total_bytes: u64,
    /// The breakdown of packet counts by network protocol.
    pub(super) protocols: HashMap<String, u64>,
    /// The top network connections ranked by traffic volume.
    pub(super) top_connections: Vec<ConnectionInfo>,
    /// The most recently captured network packets.
    pub(super) recent_packets: Vec<NetworkPacket>,
}

/// Information about a single network connection including its traffic volume.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct ConnectionInfo {
    /// The remote IP address of the connection.
    pub(super) remote_ip: String,
    /// The remote port number of the connection.
    #[get(type(copy))]
    pub(super) port: usize,
    /// The network protocol used by the connection.
    pub(super) protocol: String,
    /// The number of packets exchanged in this connection.
    #[get(type(copy))]
    pub(super) packets: u64,
    /// The total bytes exchanged in this connection.
    #[get(type(copy))]
    pub(super) bytes: u64,
}

/// Request parameters for starting a network packet capture session.
#[derive(Clone, Data, Debug, Deserialize, Serialize, ToSchema)]
pub struct NetworkCaptureRequest {
    /// The optional duration in seconds for the capture session.
    pub(super) duration_seconds: Option<u64>,
    /// The optional protocol filter to capture only specific protocols.
    pub(super) filter_protocol: Option<String>,
    /// The optional port filter to capture only traffic on a specific port.
    pub(super) filter_port: Option<usize>,
}

/// Response returned after a network capture operation, containing the capture status and results.
#[derive(Clone, Data, Debug, Deserialize, Serialize, ToSchema)]
pub struct NetworkCaptureResponse {
    /// The status of the capture operation (e.g., "started", "completed", "error").
    pub(super) status: String,
    /// A human-readable message describing the capture result.
    pub(super) message: String,
    /// The captured network statistics, if the capture completed successfully.
    pub(super) data: Option<NetworkStats>,
}

/// Real-time server status snapshot including CPU, memory, disk, network, and system information.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct ServerStatus {
    /// The timestamp (in milliseconds) when this status was captured.
    #[get(type(copy))]
    pub(super) timestamp: u64,
    /// The current CPU usage as a percentage (0.0-100.0).
    #[get(type(copy))]
    pub(super) cpu_usage: f64,
    /// The current memory usage as a percentage (0.0-100.0).
    #[get(type(copy))]
    pub(super) memory_usage: f64,
    /// The total physical memory in bytes.
    #[get(type(copy))]
    pub(super) memory_total: u64,
    /// The used physical memory in bytes.
    #[get(type(copy))]
    pub(super) memory_used: u64,
    /// The current disk usage as a percentage (0.0-100.0).
    #[get(type(copy))]
    pub(super) disk_usage: f64,
    /// The total disk capacity in bytes.
    #[get(type(copy))]
    pub(super) disk_total: u64,
    /// The used disk space in bytes.
    #[get(type(copy))]
    pub(super) disk_used: u64,
    /// The total bytes received on all network interfaces.
    #[get(type(copy))]
    pub(super) network_rx: u64,
    /// The total bytes transmitted on all network interfaces.
    #[get(type(copy))]
    pub(super) network_tx: u64,
    /// The system uptime in seconds.
    #[get(type(copy))]
    pub(super) uptime: u64,
    /// The system load average over the last minute.
    #[get(type(copy))]
    pub(super) load_average: f64,
    /// The number of currently active network connections.
    #[get(type(copy))]
    pub(super) active_connections: u32,
    /// The total number of running processes.
    #[get(type(copy))]
    pub(super) process_count: u32,
    /// The hostname of the server.
    pub(super) hostname: String,
    /// The operating system name.
    pub(super) os_name: String,
    /// The operating system version.
    pub(super) os_version: String,
    /// The kernel version string.
    pub(super) kernel_version: String,
    /// The number of logical CPU cores.
    #[get(type(copy))]
    pub(super) cpu_cores: u32,
    /// The CPU model identifier string.
    pub(super) cpu_model: String,
}

/// Static system information that does not change during runtime.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct SystemInfo {
    /// The hostname of the server.
    pub(super) hostname: String,
    /// The operating system name.
    pub(super) os_name: String,
    /// The operating system version.
    pub(super) os_version: String,
    /// The kernel version string.
    pub(super) kernel_version: String,
    /// The number of logical CPU cores.
    #[get(type(copy))]
    pub(super) cpu_cores: u32,
    /// The CPU model identifier string.
    pub(super) cpu_model: String,
    /// The total physical memory in bytes.
    #[get(type(copy))]
    pub(super) total_memory: u64,
    /// The total disk capacity in bytes.
    #[get(type(copy))]
    pub(super) total_disk: u64,
}

/// A single data point in the performance history time series.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct PerformanceDataPoint {
    /// The timestamp (in milliseconds) when this data point was collected.
    #[get(type(copy))]
    pub(super) timestamp: u64,
    /// The CPU usage as a percentage (0.0-100.0) at this point.
    #[get(type(copy))]
    pub(super) cpu_usage: f64,
    /// The memory usage as a percentage (0.0-100.0) at this point.
    #[get(type(copy))]
    pub(super) memory_usage: f64,
    /// The used physical memory in bytes at this point.
    #[get(type(copy))]
    pub(super) memory_used: u64,
    /// The disk usage as a percentage (0.0-100.0) at this point.
    #[get(type(copy))]
    pub(super) disk_usage: f64,
    /// The used disk space in bytes at this point.
    #[get(type(copy))]
    pub(super) disk_used: u64,
    /// The total bytes received on all network interfaces at this point.
    #[get(type(copy))]
    pub(super) network_rx: u64,
    /// The total bytes transmitted on all network interfaces at this point.
    #[get(type(copy))]
    pub(super) network_tx: u64,
    /// The system load average at this point.
    #[get(type(copy))]
    pub(super) load_average: f64,
    /// The number of active connections at this point.
    #[get(type(copy))]
    pub(super) active_connections: u32,
    /// The number of running processes at this point.
    #[get(type(copy))]
    pub(super) process_count: u32,
}

/// Response containing a time series of performance data points for historical analysis.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct PerformanceHistoryResponse {
    /// The ordered list of performance data points.
    pub(super) data_points: Vec<PerformanceDataPoint>,
    /// The total number of data points in the response.
    #[get(type(copy))]
    pub(super) total_points: usize,
    /// The time range in seconds covered by the data points.
    #[get(type(copy))]
    pub(super) time_range_seconds: u64,
}
