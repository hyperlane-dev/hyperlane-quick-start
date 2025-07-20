use super::*;

pub async fn get_server_status() -> ServerStatus {
    let timestamp: u64 = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let cpu_usage: f64 = get_cpu_usage().await;
    let (memory_used, memory_total, memory_usage) = get_memory_info().await;
    let (disk_used, disk_total, disk_usage) = get_disk_info().await;
    let (network_rx, network_tx) = get_network_info().await;
    let uptime: u64 = get_uptime().await;
    let load_average: f64 = get_load_average().await;
    let active_connections: u32 = get_active_connections().await;
    let process_count: u32 = get_process_count().await;

    ServerStatus {
        timestamp,
        cpu_usage,
        memory_usage,
        memory_total,
        memory_used,
        disk_usage,
        disk_total,
        disk_used,
        network_rx,
        network_tx,
        uptime,
        load_average,
        active_connections,
        process_count,
    }
}

pub async fn get_system_info() -> SystemInfo {
    let hostname: String = get_hostname().await;
    let os_name: String = get_os_name().await;
    let os_version: String = get_os_version().await;
    let kernel_version: String = get_kernel_version().await;
    let cpu_cores: u32 = get_cpu_cores().await;
    let cpu_model: String = get_cpu_model().await;
    let total_memory: u64 = get_total_memory().await;
    let total_disk: u64 = get_total_disk().await;

    SystemInfo {
        hostname,
        os_name,
        os_version,
        kernel_version,
        cpu_cores,
        cpu_model,
        total_memory,
        total_disk,
    }
}

async fn get_cpu_usage() -> f64 {
    if let Ok(stat) = fs::read_to_string("/proc/stat") {
        if let Some(line) = stat.lines().next() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 5 {
                let user: u64 = parts[1].parse().unwrap_or(0);
                let nice: u64 = parts[2].parse().unwrap_or(0);
                let system: u64 = parts[3].parse().unwrap_or(0);
                let idle: u64 = parts[4].parse().unwrap_or(0);
                let total: u64 = user + nice + system + idle;
                let used: u64 = total - idle;
                if total > 0 {
                    return (used as f64 / total as f64) * 100.0;
                }
            }
        }
    }
    0.0
}

async fn get_memory_info() -> (u64, u64, f64) {
    let mut total: u64 = 0;
    let mut available: u64 = 0;

    if let Ok(meminfo) = fs::read_to_string("/proc/meminfo") {
        for line in meminfo.lines() {
            if line.starts_with("MemTotal:") {
                if let Some(value) = line.split_whitespace().nth(1) {
                    total = value.parse::<u64>().unwrap_or(0) * 1024;
                }
            } else if line.starts_with("MemAvailable:") {
                if let Some(value) = line.split_whitespace().nth(1) {
                    available = value.parse::<u64>().unwrap_or(0) * 1024;
                }
            }
        }
    }

    let used: u64 = total.saturating_sub(available);
    let usage: f64 = if total > 0 {
        (used as f64 / total as f64) * 100.0
    } else {
        0.0
    };

    (used, total, usage)
}

async fn get_disk_info() -> (u64, u64, f64) {
    if let Ok(_) = fs::metadata("/") {
        let total: u64 = 1024 * 1024 * 1024 * 100;
        let used: u64 = total / 2;
        let usage: f64 = (used as f64 / total as f64) * 100.0;
        return (used, total, usage);
    }
    (0, 0, 0.0)
}

async fn get_network_info() -> (u64, u64) {
    let mut rx_bytes: u64 = 0;
    let mut tx_bytes: u64 = 0;
    if let Ok(net_dev) = fs::read_to_string("/proc/net/dev") {
        for line in net_dev.lines().skip(2) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 10 && !parts[0].starts_with("lo:") {
                rx_bytes += parts[1].parse::<u64>().unwrap_or(0);
                tx_bytes += parts[9].parse::<u64>().unwrap_or(0);
            }
        }
    }

    (rx_bytes, tx_bytes)
}

async fn get_uptime() -> u64 {
    if let Ok(uptime_str) = fs::read_to_string("/proc/uptime") {
        if let Some(uptime_part) = uptime_str.split_whitespace().next() {
            if let Ok(uptime_f64) = uptime_part.parse::<f64>() {
                return uptime_f64 as u64;
            }
        }
    }
    0
}

async fn get_load_average() -> f64 {
    if let Ok(loadavg) = fs::read_to_string("/proc/loadavg") {
        if let Some(load_str) = loadavg.split_whitespace().next() {
            return load_str.parse().unwrap_or(0.0);
        }
    }
    0.0
}

async fn get_active_connections() -> u32 {
    let mut count: u32 = 0;
    if let Ok(tcp) = fs::read_to_string("/proc/net/tcp") {
        count += tcp.lines().skip(1).count() as u32;
    }
    if let Ok(tcp6) = fs::read_to_string("/proc/net/tcp6") {
        count += tcp6.lines().skip(1).count() as u32;
    }
    count
}

async fn get_process_count() -> u32 {
    if let Ok(entries) = fs::read_dir("/proc") {
        return entries
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                entry
                    .file_name()
                    .to_string_lossy()
                    .chars()
                    .all(|c| c.is_ascii_digit())
            })
            .count() as u32;
    }
    0
}

async fn get_hostname() -> String {
    fs::read_to_string("/proc/sys/kernel/hostname")
        .unwrap_or_else(|_| "Unknown".to_string())
        .trim()
        .to_string()
}

async fn get_os_name() -> String {
    if let Ok(os_release) = fs::read_to_string("/etc/os-release") {
        for line in os_release.lines() {
            if line.starts_with("PRETTY_NAME=") {
                return line
                    .split('=')
                    .nth(1)
                    .unwrap_or("Unknown")
                    .trim_matches('"')
                    .to_string();
            }
        }
    }
    "Unknown".to_string()
}

async fn get_os_version() -> String {
    if let Ok(os_release) = fs::read_to_string("/etc/os-release") {
        for line in os_release.lines() {
            if line.starts_with("VERSION=") {
                return line
                    .split('=')
                    .nth(1)
                    .unwrap_or("Unknown")
                    .trim_matches('"')
                    .to_string();
            }
        }
    }
    "Unknown".to_string()
}

async fn get_kernel_version() -> String {
    fs::read_to_string("/proc/version")
        .unwrap_or_else(|_| "Unknown".to_string())
        .split_whitespace()
        .nth(2)
        .unwrap_or("Unknown")
        .to_string()
}

async fn get_cpu_cores() -> u32 {
    if let Ok(cpuinfo) = fs::read_to_string("/proc/cpuinfo") {
        return cpuinfo
            .lines()
            .filter(|line| line.starts_with("processor"))
            .count() as u32;
    }
    1
}

async fn get_cpu_model() -> String {
    if let Ok(cpuinfo) = fs::read_to_string("/proc/cpuinfo") {
        for line in cpuinfo.lines() {
            if line.starts_with("model name") {
                if let Some(model) = line.split(':').nth(1) {
                    return model.trim().to_string();
                }
            }
        }
    }
    "Unknown".to_string()
}

async fn get_total_memory() -> u64 {
    if let Ok(meminfo) = fs::read_to_string("/proc/meminfo") {
        for line in meminfo.lines() {
            if line.starts_with("MemTotal:") {
                if let Some(value) = line.split_whitespace().nth(1) {
                    return value.parse::<u64>().unwrap_or(0) * 1024;
                }
            }
        }
    }
    0
}

async fn get_total_disk() -> u64 {
    1024 * 1024 * 1024 * 100
}
