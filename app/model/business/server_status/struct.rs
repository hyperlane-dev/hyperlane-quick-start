use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
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

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
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

impl Default for ServerStatus {
    fn default() -> Self {
        Self {
            timestamp: 0,
            cpu_usage: 0.0,
            memory_usage: 0.0,
            memory_total: 0,
            memory_used: 0,
            disk_usage: 0.0,
            disk_total: 0,
            disk_used: 0,
            network_rx: 0,
            network_tx: 0,
            uptime: 0,
            load_average: 0.0,
            active_connections: 0,
            process_count: 0,
            hostname: String::new(),
            os_name: String::new(),
            os_version: String::new(),
            kernel_version: String::new(),
            cpu_cores: 0,
            cpu_model: String::new(),
        }
    }
}

impl Default for SystemInfo {
    fn default() -> Self {
        Self {
            hostname: String::new(),
            os_name: String::new(),
            os_version: String::new(),
            kernel_version: String::new(),
            cpu_cores: 0,
            cpu_model: String::new(),
            total_memory: 0,
            total_disk: 0,
        }
    }
}
