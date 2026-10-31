use super::*;

/// The static information of the server.
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(default)]
pub struct SystemInfo {
    /// The hostname of the server.
    pub hostname: String,
    /// The name of the operating system.
    pub os_name: String,
    /// The version of the operating system.
    pub os_version: String,
    /// The version of the kernel.
    pub kernel_version: String,
    /// The number of logical CPU cores.
    pub cpu_cores: u32,
    /// The model of the processor.
    pub cpu_model: String,
    /// The total physical memory in bytes.
    pub total_memory: u64,
    /// The total disk capacity in bytes.
    pub total_disk: u64,
}

/// The live status of the server.
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(default)]
pub struct ServerStatus {
    /// The moment the status was taken, in milliseconds since the Unix epoch.
    pub timestamp: u64,
    /// The share of the processor in use, as a percentage.
    pub cpu_usage: f64,
    /// The share of the memory in use, as a percentage.
    pub memory_usage: f64,
    /// The total physical memory in bytes.
    pub memory_total: u64,
    /// The used physical memory in bytes.
    pub memory_used: u64,
    /// The share of the disk in use, as a percentage.
    pub disk_usage: f64,
    /// The total disk capacity in bytes.
    pub disk_total: u64,
    /// The used disk space in bytes.
    pub disk_used: u64,
    /// The bytes received on the network interfaces.
    pub network_rx: u64,
    /// The bytes sent on the network interfaces.
    pub network_tx: u64,
    /// The uptime of the system in seconds.
    pub uptime: u64,
    /// The load average of the last minute.
    pub load_average: f64,
    /// The number of the open connections.
    pub active_connections: u32,
    /// The number of the running processes.
    pub process_count: u32,
    /// The hostname of the server.
    pub hostname: String,
    /// The name of the operating system.
    pub os_name: String,
    /// The version of the operating system.
    pub os_version: String,
    /// The version of the kernel.
    pub kernel_version: String,
    /// The number of logical CPU cores.
    pub cpu_cores: u32,
    /// The model of the processor.
    pub cpu_model: String,
}

/// One sample of the performance history of the server.
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(default)]
pub struct PerformancePoint {
    /// The moment the sample was taken, in milliseconds since the Unix epoch.
    pub timestamp: u64,
    /// The share of the processor in use, as a percentage.
    pub cpu_usage: f64,
    /// The share of the memory in use, as a percentage.
    pub memory_usage: f64,
    /// The used physical memory in bytes.
    pub memory_used: u64,
    /// The share of the disk in use, as a percentage.
    pub disk_usage: f64,
    /// The used disk space in bytes.
    pub disk_used: u64,
    /// The bytes received on the network interfaces.
    pub network_rx: u64,
    /// The bytes sent on the network interfaces.
    pub network_tx: u64,
    /// The load average of the last minute.
    pub load_average: f64,
    /// The number of the open connections.
    pub active_connections: u32,
    /// The number of the running processes.
    pub process_count: u32,
}

/// The performance history of the server.
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(default)]
pub struct PerformanceHistory {
    /// The samples of the history, oldest first.
    pub data_points: Vec<PerformancePoint>,
    /// The number of the samples the history holds.
    pub total_points: usize,
    /// The span of the history in seconds.
    pub time_range_seconds: u64,
}
