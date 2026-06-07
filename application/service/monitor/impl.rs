use super::*;

/// Implementation of methods for `MonitorService`.
impl MonitorService {
    /// Returns the lazily-initialized static system information instance.
    ///
    /// # Returns
    ///
    /// - `&'static RwLock<System>`: The static reference to the system information.
    #[instrument_trace]
    fn get_or_init_system() -> &'static RwLock<System> {
        SYSTEM.get_or_init(|| RwLock::new(System::new_all()))
    }

    /// Returns the lazily-initialized static network information instance.
    ///
    /// # Returns
    ///
    /// - `&'static RwLock<Networks>`: The static reference to the network information.
    #[instrument_trace]
    fn get_or_init_networks() -> &'static RwLock<Networks> {
        NETWORKS.get_or_init(|| RwLock::new(Networks::new_with_refreshed_list()))
    }

    /// Refreshes all system information including CPU, memory, and processes.
    #[instrument_trace]
    async fn refresh_system() {
        Self::get_or_init_system().write().await.refresh_all();
    }

    /// Refreshes network interface information.
    #[instrument_trace]
    async fn refresh_networks() {
        Self::get_or_init_networks().write().await.refresh(true);
    }

    /// Refreshes CPU usage information only.
    #[instrument_trace]
    async fn refresh_cpu() {
        Self::get_or_init_system().write().await.refresh_cpu_all();
    }

    /// Refreshes memory information only.
    #[instrument_trace]
    async fn refresh_memory() {
        Self::get_or_init_system().write().await.refresh_memory();
    }

    /// Starts a background task that periodically captures network statistics.
    #[instrument_trace]
    pub async fn start_network_capture() {
        init_network_capture_globals();
        set_capture_status(CaptureStatus::Running).await;
        spawn(async {
            loop {
                if let Some(stats) = Self::capture_network_data().await {
                    set_network_stats(stats).await;
                }
                sleep(Duration::from_secs(MONITOR_INTERVAL_SECONDS)).await;
            }
        });
    }

    /// Starts a background task that periodically collects performance data points.
    #[instrument_trace]
    pub async fn start_performance_data_collection() {
        spawn(async {
            loop {
                let data_point: PerformanceDataPoint = Self::collect_performance_data_point().await;
                add_performance_data_point(data_point).await;
                sleep(Duration::from_secs(MONITOR_INTERVAL_SECONDS)).await;
            }
        });
    }

    /// Collects a single performance data point by refreshing system and network information.
    ///
    /// # Returns
    ///
    /// - `PerformanceDataPoint`: The collected data point containing CPU, memory, disk, network, and process metrics.
    #[instrument_trace]
    async fn collect_performance_data_point() -> PerformanceDataPoint {
        Self::refresh_system().await;
        Self::refresh_networks().await;
        let timestamp: u64 = Utc::now().timestamp_millis() as u64;
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

    /// Builds a performance history response from the collected data points.
    ///
    /// # Returns
    ///
    /// - `PerformanceHistoryResponse`: The response containing all data points, total count, and time range.
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

    /// Captures current network data by summing packets and bytes across non-loopback interfaces.
    ///
    /// # Returns
    ///
    /// - `Option<NetworkStats>`: The captured network statistics, or `None` if refresh fails.
    async fn capture_network_data() -> Option<NetworkStats> {
        Self::refresh_networks().await;
        let mut stats: NetworkStats = NetworkStats::default();
        let mut total_packets: u64 = 0;
        let mut total_bytes: u64 = 0;
        let networks: RwLockReadGuard<'_, Networks> = Self::get_or_init_networks().read().await;
        for (interface_name, network) in networks.iter() {
            if interface_name.contains(LOOPBACK_INTERFACE_LO)
                || interface_name.contains(LOOPBACK_INTERFACE_LOOPBACK)
            {
                continue;
            }
            total_packets += network.total_packets_received() + network.total_packets_transmitted();
            total_bytes += network.total_received() + network.total_transmitted();
        }
        stats.set_total_packets(total_packets);
        stats.set_total_bytes(total_bytes);
        stats.set_top_connections(vec![]);
        stats.set_recent_packets(vec![]);
        Some(stats)
    }

    /// Writes the current network capture data as JSON to the response body.
    ///
    /// # Arguments
    ///
    /// - `&mut Context`: The request context to write the response to.
    #[instrument_trace]
    pub async fn get_network_capture_data(ctx: &mut Context) {
        let response_data: NetworkStats = get_network_stats().await.unwrap_or_default();
        if let Ok(json) = serde_json::to_vec(&response_data) {
            ctx.get_mut_response().set_body(&json);
        }
    }

    /// Writes the current network capture data as an SSE event to the response body.
    ///
    /// # Arguments
    ///
    /// - `&mut Context`: The request context to write the SSE response to.
    #[instrument_trace]
    pub async fn get_network_capture_stream(ctx: &mut Context) {
        let response_data: NetworkStats = get_network_stats().await.unwrap_or_default();
        if let Ok(json) = serde_json::to_string(&response_data) {
            let event: String = format!("{SSE_DATA_PREFIX}{json}{DOUBLE_BR}");
            ctx.get_mut_response().set_body(&event);
        }
    }

    /// Collects the current server status including system info, resource usage, and the latest performance data.
    ///
    /// # Returns
    ///
    /// - `ServerStatus`: The comprehensive server status object.
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

    /// Collects static system information including hostname, OS, CPU, and memory details.
    ///
    /// # Returns
    ///
    /// - `SystemInfo`: The system information object.
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

    /// Refreshes CPU data and returns the average CPU usage percentage across all cores.
    ///
    /// # Returns
    ///
    /// - `f64`: The average CPU usage percentage (0.0 to 100.0).
    #[instrument_trace]
    async fn get_cpu_usage() -> f64 {
        Self::refresh_cpu().await;
        let system: RwLockReadGuard<'_, System> = Self::get_or_init_system().read().await;
        let cpus: &[Cpu] = system.cpus();
        if !cpus.is_empty() {
            let total_usage: f32 = cpus.iter().map(|cpu: &Cpu| cpu.cpu_usage()).sum();
            return (total_usage / cpus.len() as f32) as f64;
        }
        0.0
    }

    /// Refreshes memory data and returns memory usage information.
    ///
    /// # Returns
    ///
    /// - `(u64, u64, f64)`: A tuple of (used bytes, total bytes, usage percentage).
    #[instrument_trace]
    async fn get_memory_info() -> (u64, u64, f64) {
        Self::refresh_memory().await;
        let system: RwLockReadGuard<'_, System> = Self::get_or_init_system().read().await;
        let total: u64 = system.total_memory();
        let used: u64 = system.used_memory();
        let usage: f64 = if total > 0 {
            (used as f64 / total as f64) * 100.0
        } else {
            0.0
        };
        (used, total, usage)
    }

    /// Returns disk usage information by summing all disk spaces.
    ///
    /// # Returns
    ///
    /// - `(u64, u64, f64)`: A tuple of (used bytes, total bytes, usage percentage).
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

    /// Returns total network bytes received and transmitted across all interfaces.
    ///
    /// # Returns
    ///
    /// - `(u64, u64)`: A tuple of (received bytes, transmitted bytes).
    #[instrument_trace]
    async fn get_network_info() -> (u64, u64) {
        Self::refresh_networks().await;
        let networks: RwLockReadGuard<'_, Networks> = Self::get_or_init_networks().read().await;
        let mut rx_bytes: u64 = 0;
        let mut tx_bytes: u64 = 0;
        for (_, network) in networks.iter() {
            rx_bytes += network.total_received();
            tx_bytes += network.total_transmitted();
        }
        (rx_bytes, tx_bytes)
    }

    /// Returns the system uptime in seconds.
    ///
    /// # Returns
    ///
    /// - `u64`: The number of seconds since boot.
    #[instrument_trace]
    fn get_uptime() -> u64 {
        System::uptime()
    }

    /// Calculates the system load average based on CPU usage.
    ///
    /// # Returns
    ///
    /// - `f64`: The load average value (0.0 to ~number_of_cpus).
    #[instrument_trace]
    async fn get_load_average() -> f64 {
        let system: RwLockReadGuard<'_, System> = Self::get_or_init_system().read().await;
        let cpus: &[Cpu] = system.cpus();
        if !cpus.is_empty() {
            let total_usage: f32 = cpus.iter().map(|cpu: &Cpu| cpu.cpu_usage()).sum();
            let avg_usage: f64 = (total_usage / cpus.len() as f32) as f64;
            return avg_usage / 100.0;
        }
        0.0
    }

    /// Returns the number of active connections based on the process count.
    ///
    /// # Returns
    ///
    /// - `u32`: The number of active connections.
    #[instrument_trace]
    async fn get_active_connections() -> u32 {
        Self::get_or_init_system().read().await.processes().len() as u32
    }

    /// Returns the total number of running processes.
    ///
    /// # Returns
    ///
    /// - `u32`: The process count.
    #[instrument_trace]
    async fn get_process_count() -> u32 {
        Self::get_or_init_system().read().await.processes().len() as u32
    }

    /// Returns the system hostname, or a fallback value if unavailable.
    ///
    /// # Returns
    ///
    /// - `String`: The hostname string.
    #[instrument_trace]
    fn get_hostname() -> String {
        System::host_name().unwrap_or_else(|| FALLBACK_UNKNOWN.to_string())
    }

    /// Returns the operating system name, or a fallback value if unavailable.
    ///
    /// # Returns
    ///
    /// - `String`: The OS name string.
    #[instrument_trace]
    fn get_os_name() -> String {
        System::name().unwrap_or_else(|| FALLBACK_UNKNOWN.to_string())
    }

    /// Returns the operating system version, or a fallback value if unavailable.
    ///
    /// # Returns
    ///
    /// - `String`: The OS version string.
    #[instrument_trace]
    fn get_os_version() -> String {
        System::os_version().unwrap_or_else(|| FALLBACK_UNKNOWN.to_string())
    }

    /// Returns the kernel version, or a fallback value if unavailable.
    ///
    /// # Returns
    ///
    /// - `String`: The kernel version string.
    #[instrument_trace]
    fn get_kernel_version() -> String {
        System::kernel_version().unwrap_or_else(|| FALLBACK_UNKNOWN.to_string())
    }

    /// Returns the number of CPU cores.
    ///
    /// # Returns
    ///
    /// - `u32`: The CPU core count.
    #[instrument_trace]
    async fn get_cpu_cores() -> u32 {
        Self::get_or_init_system().read().await.cpus().len() as u32
    }

    /// Returns the CPU model name of the first CPU.
    ///
    /// # Returns
    ///
    /// - `String`: The CPU brand/model string.
    #[instrument_trace]
    async fn get_cpu_model() -> String {
        let system: RwLockReadGuard<'_, System> = Self::get_or_init_system().read().await;
        if let Some(cpu) = system.cpus().first() {
            return cpu.brand().to_string();
        }
        FALLBACK_UNKNOWN.to_string()
    }

    /// Refreshes memory data and returns the total physical memory in bytes.
    ///
    /// # Returns
    ///
    /// - `u64`: The total memory in bytes.
    #[instrument_trace]
    async fn get_total_memory() -> u64 {
        Self::refresh_memory().await;
        Self::get_or_init_system().read().await.total_memory()
    }

    /// Returns the total disk space across all disks in bytes.
    ///
    /// # Returns
    ///
    /// - `u64`: The total disk space in bytes.
    #[instrument_trace]
    fn get_total_disk() -> u64 {
        let disks: Disks = Disks::new_with_refreshed_list();
        disks.iter().map(|disk: &Disk| disk.total_space()).sum()
    }
}
