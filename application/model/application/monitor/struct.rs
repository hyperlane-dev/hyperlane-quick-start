use super::*;

/// Represents a captured network packet with source/destination information and metadata.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct NetworkPacket {
    /// The timestamp (in milliseconds) when the packet was captured.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) timestamp: u64,
    /// The network protocol of the packet (e.g., "TCP", "UDP", "HTTP").
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) protocol: String,
    /// The source IP address of the packet.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) src_ip: String,
    /// The destination IP address of the packet.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) dst_ip: String,
    /// The source port number of the packet.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) src_port: usize,
    /// The destination port number of the packet.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) dst_port: usize,
    /// The size of the packet in bytes.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) size: u32,
    /// The direction of the packet relative to the host ("inbound" or "outbound").
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) direction: String,
}

/// Aggregated network traffic statistics including protocol breakdown and top connections.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct NetworkStats {
    /// The total number of packets captured.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) total_packets: u64,
    /// The total number of bytes transferred.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) total_bytes: u64,
    /// The breakdown of packet counts by network protocol.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) protocols: HashMap<String, u64>,
    /// The top network connections ranked by traffic volume.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) top_connections: Vec<ConnectionInfo>,
    /// The most recently captured network packets.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) recent_packets: Vec<NetworkPacket>,
}

/// Information about a single network connection including its traffic volume.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct ConnectionInfo {
    /// The remote IP address of the connection.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) remote_ip: String,
    /// The remote port number of the connection.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) port: usize,
    /// The network protocol used by the connection.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) protocol: String,
    /// The number of packets exchanged in this connection.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) packets: u64,
    /// The total bytes exchanged in this connection.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) bytes: u64,
}

/// Request parameters for starting a network packet capture session.
#[derive(Clone, Data, Debug, Deserialize, Serialize, ToSchema)]
pub struct NetworkCaptureRequest {
    #[get(pub(crate), type(copy))]
    /// The optional duration in seconds for the capture session.
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) duration_seconds: Option<u64>,
    /// The optional protocol filter to capture only specific protocols.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) filter_protocol: Option<String>,
    #[get(pub(crate), type(copy))]
    /// The optional port filter to capture only traffic on a specific port.
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) filter_port: Option<usize>,
}

/// Response returned after a network capture operation, containing the capture status and results.
#[derive(Clone, Data, Debug, Deserialize, Serialize, ToSchema)]
pub struct NetworkCaptureResponse {
    /// The status of the capture operation (e.g., "started", "completed", "error").
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) status: String,
    /// A human-readable message describing the capture result.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) message: String,
    /// The captured network statistics, if the capture completed successfully.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) data: Option<NetworkStats>,
}

/// Real-time server status snapshot including CPU, memory, disk, network, and system information.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct ServerStatus {
    /// The timestamp (in milliseconds) when this status was captured.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) timestamp: u64,
    /// The current CPU usage as a percentage (0.0-100.0).
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) cpu_usage: f64,
    /// The current memory usage as a percentage (0.0-100.0).
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) memory_usage: f64,
    /// The total physical memory in bytes.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) memory_total: u64,
    /// The used physical memory in bytes.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) memory_used: u64,
    /// The current disk usage as a percentage (0.0-100.0).
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) disk_usage: f64,
    /// The total disk capacity in bytes.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) disk_total: u64,
    /// The used disk space in bytes.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) disk_used: u64,
    /// The total bytes received on all network interfaces.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) network_rx: u64,
    /// The total bytes transmitted on all network interfaces.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) network_tx: u64,
    /// The system uptime in seconds.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) uptime: u64,
    /// The system load average over the last minute.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) load_average: f64,
    /// The number of currently active network connections.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) active_connections: u32,
    /// The total number of running processes.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) process_count: u32,
    /// The hostname of the server.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) hostname: String,
    /// The operating system name.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) os_name: String,
    /// The operating system version.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) os_version: String,
    /// The kernel version string.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) kernel_version: String,
    /// The number of logical CPU cores.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) cpu_cores: u32,
    /// The CPU model identifier string.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) cpu_model: String,
}

/// Static system information that does not change during runtime.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct SystemInfo {
    /// The hostname of the server.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) hostname: String,
    /// The operating system name.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) os_name: String,
    /// The operating system version.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) os_version: String,
    /// The kernel version string.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) kernel_version: String,
    /// The number of logical CPU cores.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) cpu_cores: u32,
    /// The CPU model identifier string.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) cpu_model: String,
    /// The total physical memory in bytes.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) total_memory: u64,
    /// The total disk capacity in bytes.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) total_disk: u64,
}

/// A single data point in the performance history time series.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct PerformanceDataPoint {
    /// The timestamp (in milliseconds) when this data point was collected.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) timestamp: u64,
    /// The CPU usage as a percentage (0.0-100.0) at this point.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) cpu_usage: f64,
    /// The memory usage as a percentage (0.0-100.0) at this point.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) memory_usage: f64,
    /// The used physical memory in bytes at this point.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) memory_used: u64,
    /// The disk usage as a percentage (0.0-100.0) at this point.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) disk_usage: f64,
    /// The used disk space in bytes at this point.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) disk_used: u64,
    /// The total bytes received on all network interfaces at this point.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) network_rx: u64,
    /// The total bytes transmitted on all network interfaces at this point.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) network_tx: u64,
    /// The system load average at this point.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) load_average: f64,
    /// The number of active connections at this point.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) active_connections: u32,
    /// The number of running processes at this point.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) process_count: u32,
}

/// Response containing a time series of performance data points for historical analysis.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct PerformanceHistoryResponse {
    /// The ordered list of performance data points.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) data_points: Vec<PerformanceDataPoint>,
    /// The total number of data points in the response.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) total_points: usize,
    /// The time range in seconds covered by the data points.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) time_range_seconds: u64,
}
