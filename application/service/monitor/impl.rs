use super::*;

use sysinfo::{Disks, Networks, System};

static SYSTEM: LazyLock<RwLock<System>> = LazyLock::new(|| RwLock::new(System::new_all()));
static NETWORKS: LazyLock<RwLock<Networks>> =
    LazyLock::new(|| RwLock::new(Networks::new_with_refreshed_list()));

impl MonitorService {
    #[instrument_trace]
    async fn refresh_system() {
        SYSTEM.write().await.refresh_all();
    }

    #[instrument_trace]
    async fn refresh_networks() {
        NETWORKS.write().await.refresh(true);
    }

    #[instrument_trace]
    async fn refresh_cpu() {
        SYSTEM.write().await.refresh_cpu_all();
    }

    #[instrument_trace]
    async fn refresh_memory() {
        SYSTEM.write().await.refresh_memory();
    }

    #[instrument_trace]
    pub async fn start_network_capture() {
        init_network_capture_globals();
        set_capture_status(CaptureStatus::Running).await;
        let _handle: std::thread::JoinHandle<()> = std::thread::spawn(|| {
            let rt: Runtime = Runtime::new().unwrap();
            rt.block_on(async {
                loop {
                    if let Some(stats) = Self::capture_network_data().await {
                        set_network_stats(stats).await;
                    }
                    std::thread::sleep(Duration::from_secs(CAPTURE_INTERVAL_SECONDS));
                }
            });
        });
    }

    #[instrument_trace]
    pub async fn start_performance_data_collection() {
        let _handle: std::thread::JoinHandle<()> = std::thread::spawn(|| {
            let rt: Runtime = Runtime::new().unwrap();
            rt.block_on(async {
                loop {
                    let data_point: PerformanceDataPoint =
                        Self::collect_performance_data_point().await;
                    add_performance_data_point(data_point).await;
                    sleep(Duration::from_secs(1)).await;
                }
            });
        });
    }

    #[instrument_trace]
    async fn collect_performance_data_point() -> PerformanceDataPoint {
        Self::refresh_system().await;
        Self::refresh_networks().await;
        let timestamp: u64 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let cpu_usage: f64 = Self::get_cpu_usage().await;
        let (memory_used, _memory_total, memory_usage) = Self::get_memory_info().await;
        let (disk_used, _disk_total, disk_usage) = Self::get_disk_info();
        let (network_rx, network_tx) = Self::get_network_info().await;
        let load_average: f64 = Self::get_load_average().await;
        let active_connections: u32 = Self::get_active_connections().await;
        let process_count: u32 = Self::get_process_count().await;
        let mut data_point: PerformanceDataPoint = PerformanceDataPoint::default();
        data_point
            .set_timestamp(timestamp)
            .set_cpu_usage(cpu_usage)
            .set_memory_usage(memory_usage)
            .set_memory_used(memory_used)
            .set_disk_usage(disk_usage)
            .set_disk_used(disk_used)
            .set_network_rx(network_rx)
            .set_network_tx(network_tx)
            .set_load_average(load_average)
            .set_active_connections(active_connections)
            .set_process_count(process_count);
        data_point
    }

    #[instrument_trace]
    pub async fn get_performance_history_response() -> PerformanceHistoryResponse {
        let data_points: Vec<PerformanceDataPoint> = get_performance_history().await;
        let total_points: usize = data_points.len();
        let time_range_seconds: u64 = if total_points > 1 {
            let first_timestamp: u64 = data_points
                .first()
                .map(|p: &PerformanceDataPoint| p.get_timestamp())
                .unwrap_or(0);
            let last_timestamp: u64 = data_points
                .last()
                .map(|p: &PerformanceDataPoint| p.get_timestamp())
                .unwrap_or(0);
            last_timestamp.saturating_sub(first_timestamp)
        } else {
            0
        };
        let mut response: PerformanceHistoryResponse = PerformanceHistoryResponse::default();
        response
            .set_data_points(data_points)
            .set_total_points(total_points)
            .set_time_range_seconds(time_range_seconds);
        response
    }

    async fn capture_network_data() -> Option<NetworkStats> {
        Self::refresh_networks().await;
        let mut stats: NetworkStats = NetworkStats::default();
        let mut total_packets: u64 = 0;
        let mut total_bytes: u64 = 0;
        let networks = NETWORKS.read().await;
        for (interface_name, network) in networks.iter() {
            if interface_name.contains("lo") || interface_name.contains("Loopback") {
                continue;
            }
            total_packets += network.total_packets_received() + network.total_packets_transmitted();
            total_bytes += network.total_received() + network.total_transmitted();
        }
        stats.set_total_packets(total_packets);
        stats.set_total_bytes(total_bytes);
        stats.set_top_connections(Vec::new());
        stats.set_recent_packets(Vec::new());
        Some(stats)
    }

    #[response_header(CONTENT_TYPE => APPLICATION_JSON)]
    #[instrument_trace]
    pub async fn get_network_capture_data(ctx: &Context) {
        let response_data: NetworkStats = get_network_stats().await.unwrap_or_default();
        if let Ok(json) = serde_json::to_vec(&response_data) {
            ctx.set_response_body(&json).await;
        }
    }

    #[instrument_trace]
    pub async fn get_network_capture_stream(ctx: &Context) {
        let response_data: NetworkStats = get_network_stats().await.unwrap_or_default();
        if let Ok(json) = serde_json::to_string(&response_data) {
            let event: String = format!("{SSE_DATA_PREFIX}{json}{DOUBLE_BR}");
            ctx.set_response_body(&event).await;
        }
    }

    #[instrument_trace]
    pub async fn get_server_status() -> ServerStatus {
        let recent_data: Vec<PerformanceDataPoint> = get_recent_performance_data(1).await;
        let latest: PerformanceDataPoint = recent_data.into_iter().next().unwrap_or_default();
        let hostname: String = Self::get_hostname();
        let os_name: String = Self::get_os_name();
        let os_version: String = Self::get_os_version();
        let kernel_version: String = Self::get_kernel_version();
        let cpu_cores: u32 = Self::get_cpu_cores().await;
        let cpu_model: String = Self::get_cpu_model().await;
        let memory_total: u64 = Self::get_total_memory().await;
        let disk_total: u64 = Self::get_total_disk();
        let uptime: u64 = Self::get_uptime();
        let mut status: ServerStatus = ServerStatus::default();
        status
            .set_timestamp(latest.get_timestamp())
            .set_cpu_usage(latest.get_cpu_usage())
            .set_memory_usage(latest.get_memory_usage())
            .set_memory_total(memory_total)
            .set_memory_used(latest.get_memory_used())
            .set_disk_usage(latest.get_disk_usage())
            .set_disk_total(disk_total)
            .set_disk_used(latest.get_disk_used())
            .set_network_rx(latest.get_network_rx())
            .set_network_tx(latest.get_network_tx())
            .set_uptime(uptime)
            .set_load_average(latest.get_load_average())
            .set_active_connections(latest.get_active_connections())
            .set_process_count(latest.get_process_count())
            .set_hostname(hostname)
            .set_os_name(os_name)
            .set_os_version(os_version)
            .set_kernel_version(kernel_version)
            .set_cpu_cores(cpu_cores)
            .set_cpu_model(cpu_model);
        status
    }

    #[instrument_trace]
    pub async fn get_system_info() -> SystemInfo {
        let hostname: String = Self::get_hostname();
        let os_name: String = Self::get_os_name();
        let os_version: String = Self::get_os_version();
        let kernel_version: String = Self::get_kernel_version();
        let cpu_cores: u32 = Self::get_cpu_cores().await;
        let cpu_model: String = Self::get_cpu_model().await;
        let total_memory: u64 = Self::get_total_memory().await;
        let total_disk: u64 = Self::get_total_disk();
        let mut info: SystemInfo = SystemInfo::default();
        info.set_hostname(hostname)
            .set_os_name(os_name)
            .set_os_version(os_version)
            .set_kernel_version(kernel_version)
            .set_cpu_cores(cpu_cores)
            .set_cpu_model(cpu_model)
            .set_total_memory(total_memory)
            .set_total_disk(total_disk);
        info
    }

    #[instrument_trace]
    async fn get_cpu_usage() -> f64 {
        Self::refresh_cpu().await;
        let system = SYSTEM.read().await;
        let cpus = system.cpus();
        if !cpus.is_empty() {
            let total_usage: f32 = cpus.iter().map(|cpu| cpu.cpu_usage()).sum();
            return (total_usage / cpus.len() as f32) as f64;
        }
        0.0
    }

    #[instrument_trace]
    async fn get_memory_info() -> (u64, u64, f64) {
        Self::refresh_memory().await;
        let system = SYSTEM.read().await;
        let total: u64 = system.total_memory();
        let used: u64 = system.used_memory();
        let usage: f64 = if total > 0 {
            (used as f64 / total as f64) * 100.0
        } else {
            0.0
        };
        (used, total, usage)
    }

    #[instrument_trace]
    fn get_disk_info() -> (u64, u64, f64) {
        let disks: Disks = Disks::new_with_refreshed_list();
        let mut total: u64 = 0;
        let mut used: u64 = 0;
        for disk in disks.iter() {
            total += disk.total_space();
            used += disk.total_space().saturating_sub(disk.available_space());
        }
        let usage: f64 = if total > 0 {
            (used as f64 / total as f64) * 100.0
        } else {
            0.0
        };
        (used, total, usage)
    }

    #[instrument_trace]
    async fn get_network_info() -> (u64, u64) {
        Self::refresh_networks().await;
        let networks = NETWORKS.read().await;
        let mut rx_bytes: u64 = 0;
        let mut tx_bytes: u64 = 0;
        for (_, network) in networks.iter() {
            rx_bytes += network.total_received();
            tx_bytes += network.total_transmitted();
        }
        (rx_bytes, tx_bytes)
    }

    #[instrument_trace]
    fn get_uptime() -> u64 {
        System::uptime()
    }

    #[instrument_trace]
    async fn get_load_average() -> f64 {
        let system = SYSTEM.read().await;
        let cpus = system.cpus();
        if !cpus.is_empty() {
            let total_usage: f32 = cpus.iter().map(|cpu| cpu.cpu_usage()).sum();
            let avg_usage: f64 = (total_usage / cpus.len() as f32) as f64;
            return avg_usage / 100.0;
        }
        0.0
    }

    #[instrument_trace]
    async fn get_active_connections() -> u32 {
        let system = SYSTEM.read().await;
        system.processes().len() as u32
    }

    #[instrument_trace]
    async fn get_process_count() -> u32 {
        let system = SYSTEM.read().await;
        system.processes().len() as u32
    }

    #[instrument_trace]
    fn get_hostname() -> String {
        System::host_name().unwrap_or_else(|| "Unknown".to_string())
    }

    #[instrument_trace]
    fn get_os_name() -> String {
        System::name().unwrap_or_else(|| "Unknown".to_string())
    }

    #[instrument_trace]
    fn get_os_version() -> String {
        System::os_version().unwrap_or_else(|| "Unknown".to_string())
    }

    #[instrument_trace]
    fn get_kernel_version() -> String {
        System::kernel_version().unwrap_or_else(|| "Unknown".to_string())
    }

    #[instrument_trace]
    async fn get_cpu_cores() -> u32 {
        let system = SYSTEM.read().await;
        system.cpus().len() as u32
    }

    #[instrument_trace]
    async fn get_cpu_model() -> String {
        let system = SYSTEM.read().await;
        if let Some(cpu) = system.cpus().first() {
            return cpu.brand().to_string();
        }
        "Unknown".to_string()
    }

    #[instrument_trace]
    async fn get_total_memory() -> u64 {
        Self::refresh_memory().await;
        let system = SYSTEM.read().await;
        system.total_memory()
    }

    #[instrument_trace]
    fn get_total_disk() -> u64 {
        let disks: Disks = Disks::new_with_refreshed_list();
        disks.iter().map(|disk| disk.total_space()).sum()
    }
}
