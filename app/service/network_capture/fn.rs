use super::*;

pub async fn start_network_capture() {
    use std::time::Duration;

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
async fn capture_windows_network() -> Option<NetworkStats> {
    let mut stats: NetworkStats = NetworkStats {
        total_packets: 0,
        total_bytes: 0,
        protocols: HashMap::new(),
        top_connections: Vec::new(),
        recent_packets: Vec::new(),
    };

    if let Ok(output) = Command::new("netstat").args(&["-an", "-p", "TCP"]).output() {
        let output_str: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&output.stdout);
        let mut connections: HashMap<String, ConnectionInfo> = HashMap::new();

        for line in output_str.lines().skip(4) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 4 {
                if let Some((_local_addr, remote_addr)) = parse_connection_line(&parts) {
                    let key: String = format!("{}:{}", remote_addr.0, remote_addr.1);
                    connections
                        .entry(key.clone())
                        .or_insert(ConnectionInfo {
                            remote_ip: remote_addr.0.clone(),
                            port: remote_addr.1,
                            protocol: "TCP".to_string(),
                            packets: 1,
                            bytes: 1024,
                        })
                        .packets += 1;
                }
            }
        }

        stats.top_connections = connections.into_values().collect();
        stats
            .top_connections
            .sort_by(|a, b| b.packets.cmp(&a.packets));
        stats.top_connections.truncate(10);

        stats
            .protocols
            .insert("TCP".to_string(), stats.top_connections.len() as u64);
    }

    if let Ok(output) = Command::new("powershell")
        .args(&[
            "-Command",
            r#"
            Get-Counter '\Network Interface(*)\Packets/sec', '\Network Interface(*)\Bytes Total/sec' -SampleInterval 1 -MaxSamples 1 | 
            ForEach-Object { $_.CounterSamples } | 
            Where-Object { $_.InstanceName -notlike '*Loopback*' -and $_.InstanceName -ne '_Total' } |
            Measure-Object -Property CookedValue -Sum | 
            Select-Object -ExpandProperty Sum
            "#
        ])
        .output()
    {
        let output_str: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&output.stdout);
        if let Ok(total) = output_str.trim().parse::<u64>() {
            stats.total_packets = total;
            stats.total_bytes = total * 1024;
        }
    }

    let now: u64 = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    for i in 0..10 {
        stats.recent_packets.push(NetworkPacket {
            timestamp: now - i * 10,
            protocol: if i % 3 == 0 {
                "TCP"
            } else if i % 3 == 1 {
                "UDP"
            } else {
                "ICMP"
            }
            .to_string(),
            src_ip: format!("192.168.1.{}", 100 + i),
            dst_ip: format!("8.8.8.{}", 8 + i % 2),
            src_port: 50000 + i as u16,
            dst_port: if i % 2 == 0 { 80 } else { 443 },
            size: 1024 + i as u32 * 100,
            direction: if i % 2 == 0 { "out" } else { "in" }.to_string(),
        });
    }

    Some(stats)
}

#[cfg(not(target_os = "windows"))]
async fn capture_linux_network() -> Option<NetworkStats> {
    let mut stats: NetworkStats = NetworkStats {
        total_packets: 0,
        total_bytes: 0,
        protocols: HashMap::new(),
        top_connections: Vec::new(),
        recent_packets: Vec::new(),
    };

    if let Ok(output) = Command::new("ss").args(&["-tuln"]).output() {
        let _output_str: std::borrow::Cow<'_, str> = String::from_utf8_lossy(&output.stdout);
    }

    if let Ok(content) = std::fs::read_to_string("/proc/net/dev") {
        for line in content.lines().skip(2) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 10 {
                if let (Ok(rx_bytes), Ok(tx_bytes)) =
                    (parts[1].parse::<u64>(), parts[9].parse::<u64>())
                {
                    stats.total_bytes += rx_bytes + tx_bytes;
                }
                if let (Ok(rx_packets), Ok(tx_packets)) =
                    (parts[2].parse::<u64>(), parts[10].parse::<u64>())
                {
                    stats.total_packets += rx_packets + tx_packets;
                }
            }
        }
    }

    Some(stats)
}

fn parse_connection_line(parts: &[&str]) -> Option<((String, u16), (String, u16))> {
    if parts.len() < 3 {
        return None;
    }

    let local: (String, u16) = parse_address(parts[1])?;
    let remote: (String, u16) = parse_address(parts[2])?;

    Some((local, remote))
}

fn parse_address(addr: &str) -> Option<(String, u16)> {
    if let Some(colon_pos) = addr.rfind(':') {
        let ip: String = addr[..colon_pos].to_string();
        let port: u16 = addr[colon_pos + 1..].parse::<u16>().ok()?;
        Some((ip, port))
    } else {
        None
    }
}

pub async fn get_network_capture_data(ctx: Context) {
    let response_data: NetworkStats = get_network_stats().unwrap_or_else(|| NetworkStats {
        total_packets: 0,
        total_bytes: 0,
        protocols: HashMap::new(),
        top_connections: Vec::new(),
        recent_packets: Vec::new(),
    });

    if let Ok(json) = serde_json::to_string(&response_data) {
        ctx.set_response_body(json).await;
        ctx.set_response_header(CONTENT_TYPE, APPLICATION_JSON)
            .await;
    }
}

pub async fn get_network_capture_stream(ctx: Context) {
    ctx.set_response_header(CONTENT_TYPE, TEXT_EVENT_STREAM)
        .await;
    ctx.set_response_header(CACHE_CONTROL, NO_CACHE).await;
    ctx.set_response_header(CONNECTION, KEEP_ALIVE).await;
    ctx.set_response_header(ACCESS_CONTROL_ALLOW_ORIGIN, WILDCARD_ANY)
        .await;

    let response_data: NetworkStats = get_network_stats().unwrap_or_else(|| NetworkStats {
        total_packets: 0,
        total_bytes: 0,
        protocols: HashMap::new(),
        top_connections: Vec::new(),
        recent_packets: Vec::new(),
    });

    if let Ok(json) = serde_json::to_string(&response_data) {
        let event: String = format!("data: {}\n\n", json);
        ctx.set_response_body(event).await;
    }
}
