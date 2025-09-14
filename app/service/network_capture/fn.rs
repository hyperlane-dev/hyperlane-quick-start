use super::*;

pub async fn start_network_capture() {
    init_network_capture_globals();
    set_capture_status(CaptureStatus::Running);
    let _handle: std::thread::JoinHandle<()> = std::thread::spawn(|| {
        let rt: Runtime = Runtime::new().unwrap();
        rt.block_on(async {
            loop {
                if let Some(stats) = capture_network_data().await {
                    set_network_stats(stats);
                }
                std::thread::sleep(Duration::from_secs(CAPTURE_INTERVAL_SECONDS));
            }
        });
    });
}

async fn capture_network_data() -> Option<NetworkStats> {
    #[cfg(target_os = "windows")]
    {
        capture_windows_network().await
    }
    #[cfg(not(target_os = "windows"))]
    {
        capture_linux_network().await
    }
}

#[cfg(target_os = "windows")]
fn create_empty_network_stats() -> NetworkStats {
    NetworkStats {
        total_packets: 0,
        total_bytes: 0,
        protocols: HashMap::new(),
        top_connections: Vec::new(),
        recent_packets: Vec::new(),
    }
}

#[cfg(target_os = "windows")]
fn process_netstat_output(output_str: &str) -> Vec<ConnectionInfo> {
    let connections: HashMap<String, ConnectionInfo> = output_str
        .lines()
        .skip(WIN_NETSTAT_SKIP_LINES)
        .filter_map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 4 {
                return None;
            }
            let (_local_addr, remote_addr) = parse_connection_line(&parts)?;
            Some(remote_addr)
        })
        .fold(HashMap::new(), |mut acc, remote_addr| {
            let key: String = format!("{}:{}", remote_addr.0, remote_addr.1);
            let entry = acc.entry(key).or_insert(ConnectionInfo {
                remote_ip: remote_addr.0,
                port: remote_addr.1,
                protocol: PROTOCOL_TCP.to_string(),
                packets: 0,
                bytes: WIN_DEFAULT_PACKET_BYTES,
            });
            entry.packets += 1;
            acc
        });
    let mut top_connections: Vec<ConnectionInfo> = connections.into_values().collect();
    top_connections.sort_by(|a, b| b.packets.cmp(&a.packets));
    top_connections.truncate(TOP_CONNECTIONS_LIMIT);
    top_connections
}

#[cfg(target_os = "windows")]
fn get_network_performance_counters() -> (u64, u64) {
    let result: Option<(u64, u64)> = (|| {
        let output = Command::new(WIN_POWERSHELL_COMMAND)
            .args(&[WIN_POWERSHELL_ARG, WIN_PERF_COUNTER_SCRIPT])
            .output()
            .ok()?;
        let output_str = String::from_utf8_lossy(&output.stdout);
        let total = output_str.trim().parse::<u64>().ok()?;
        Some((total, total * WIN_DEFAULT_PACKET_BYTES))
    })();
    result.unwrap_or((0, 0))
}

#[cfg(target_os = "windows")]
fn generate_sample_packets() -> Vec<NetworkPacket> {
    let now: u64 = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    (0..SAMPLE_PACKET_COUNT)
        .map(|i| NetworkPacket {
            timestamp: now - i * 10,
            protocol: match i % 3 {
                0 => PROTOCOL_TCP,
                1 => PROTOCOL_UDP,
                _ => PROTOCOL_ICMP,
            }
            .to_string(),
            src_ip: format!("{}{}", SAMPLE_IP_PREFIX_A, 100 + i),
            dst_ip: format!("{}{}", SAMPLE_IP_PREFIX_B, 8 + i % 2),
            src_port: SAMPLE_BASE_SRC_PORT + i as u16,
            dst_port: if i % 2 == 0 {
                SAMPLE_DST_PORT_A
            } else {
                SAMPLE_DST_PORT_B
            },
            size: SAMPLE_PACKET_BASE_SIZE + i as u32 * SAMPLE_PACKET_SIZE_MULTIPLIER,
            direction: if i % 2 == 0 {
                DIRECTION_OUT
            } else {
                DIRECTION_IN
            }
            .to_string(),
        })
        .collect()
}

#[cfg(target_os = "windows")]
async fn capture_windows_network() -> Option<NetworkStats> {
    let mut stats: NetworkStats = create_empty_network_stats();
    if let Ok(output) = Command::new(WIN_NETSTAT_COMMAND)
        .args(WIN_NETSTAT_ARGS)
        .output()
    {
        let output_str: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&output.stdout);
        stats.top_connections = process_netstat_output(&output_str);
        stats
            .protocols
            .insert(PROTOCOL_TCP.to_string(), stats.top_connections.len() as u64);
    }
    let (total_packets, total_bytes) = get_network_performance_counters();
    stats.total_packets = total_packets;
    stats.total_bytes = total_bytes;
    stats.recent_packets = generate_sample_packets();
    Some(stats)
}

#[cfg(not(target_os = "windows"))]
fn create_empty_linux_network_stats() -> NetworkStats {
    NetworkStats {
        total_packets: 0,
        total_bytes: 0,
        protocols: HashMap::new(),
        top_connections: Vec::new(),
        recent_packets: Vec::new(),
    }
}

#[cfg(not(target_os = "windows"))]
fn parse_network_dev_stats(content: &str) -> (u64, u64) {
    content
        .lines()
        .skip(LINUX_NET_DEV_SKIP_LINES)
        .filter_map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 11 {
                return None;
            }
            let rx_bytes = parts[1].parse::<u64>().ok()?;
            let tx_bytes = parts[9].parse::<u64>().ok()?;
            let rx_packets = parts[2].parse::<u64>().ok()?;
            let tx_packets = parts[10].parse::<u64>().ok()?;
            Some((rx_bytes + tx_bytes, rx_packets + tx_packets))
        })
        .fold((0, 0), |(acc_bytes, acc_packets), (bytes, packets)| {
            (acc_bytes + bytes, acc_packets + packets)
        })
}

#[cfg(not(target_os = "windows"))]
async fn capture_linux_network() -> Option<NetworkStats> {
    let mut stats: NetworkStats = create_empty_linux_network_stats();
    if let Ok(output) = Command::new(LINUX_SS_COMMAND).args(LINUX_SS_ARGS).output() {
        let _output_str: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&output.stdout);
    }
    if let Ok(content) = std::fs::read_to_string(LINUX_NET_DEV_PATH) {
        let (total_bytes, total_packets) = parse_network_dev_stats(&content);
        stats.total_bytes = total_bytes;
        stats.total_packets = total_packets;
    }
    Some(stats)
}

#[cfg(target_os = "windows")]
fn parse_connection_line(parts: &[&str]) -> Option<((String, u16), (String, u16))> {
    if parts.len() < 3 {
        return None;
    }
    let local: (String, u16) = parse_address(parts[1])?;
    let remote: (String, u16) = parse_address(parts[2])?;
    Some((local, remote))
}

#[cfg(target_os = "windows")]
fn parse_address(addr: &str) -> Option<(String, u16)> {
    let colon_pos: usize = addr.rfind(':')?;
    let ip: String = addr[..colon_pos].to_string();
    let port: u16 = addr[colon_pos + 1..].parse::<u16>().ok()?;
    Some((ip, port))
}

#[response_header(CONTENT_TYPE => APPLICATION_JSON)]
pub async fn get_network_capture_data(ctx: Context) {
    let response_data: NetworkStats = get_network_stats().unwrap_or_default();
    if let Ok(json) = serde_json::to_string(&response_data) {
        ctx.set_response_body(&json).await;
    }
}

pub async fn get_network_capture_stream(ctx: Context) {
    let response_data: NetworkStats = get_network_stats().unwrap_or_default();
    if let Ok(json) = serde_json::to_string(&response_data) {
        let event: String = format!("{}{}{}", SSE_DATA_PREFIX, json, SSE_EVENT_SUFFIX);
        ctx.set_response_body(&event).await;
    }
}
