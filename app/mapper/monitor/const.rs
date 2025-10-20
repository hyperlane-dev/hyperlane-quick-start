pub const DEFAULT_CAPTURE_DURATION: u64 = 60;
pub const MAX_CAPTURE_DURATION: u64 = 3600;
pub const DEFAULT_PACKET_BUFFER_SIZE: usize = 1000;
pub const MAX_CONNECTIONS_DISPLAY: usize = 50;
pub const CAPTURE_INTERVAL_SECONDS: u64 = 2;
pub const NETWORK_STATS_CACHE_TTL: u64 = 30;
pub const TOP_CONNECTIONS_LIMIT: usize = 10;
pub const WIN_NETSTAT_COMMAND: &str = "netstat";
pub const WIN_NETSTAT_ARGS: &[&str] = &["-an", "-p", "TCP"];
pub const WIN_NETSTAT_SKIP_LINES: usize = 4;
pub const WIN_POWERSHELL_COMMAND: &str = "powershell";
pub const WIN_POWERSHELL_ARG: &str = "-Command";
pub const WIN_PERF_COUNTER_SCRIPT: &str = r#"\
Get-Counter '\\Network Interface(*)\\Packets/sec', '\\Network Interface(*)\\Bytes Total/sec' -SampleInterval 1 -MaxSamples 1 |
ForEach-Object { $_.CounterSamples } |
Where-Object { $_.InstanceName -notlike '*Loopback*' -and $_.InstanceName -ne '_Total' } |
Measure-Object -Property CookedValue -Sum |
Select-Object -ExpandProperty Sum
"#;
pub const WIN_DEFAULT_PACKET_BYTES: u64 = 1024;
pub const LINUX_SS_COMMAND: &str = "ss";
pub const LINUX_SS_ARGS: &[&str] = &["-tuln"];
pub const LINUX_NET_DEV_PATH: &str = "/proc/net/dev";
pub const LINUX_NET_DEV_SKIP_LINES: usize = 2;
pub const SAMPLE_PACKET_COUNT: u64 = 10;
pub const SAMPLE_PACKET_BASE_SIZE: u32 = 1024;
pub const SAMPLE_PACKET_SIZE_MULTIPLIER: u32 = 100;
pub const SAMPLE_IP_PREFIX_A: &str = "192.168.1.";
pub const SAMPLE_IP_PREFIX_B: &str = "8.8.8.";
pub const SAMPLE_BASE_SRC_PORT: usize = 50000;
pub const SAMPLE_DST_PORT_A: usize = 80;
pub const SAMPLE_DST_PORT_B: usize = 443;
pub const PROTOCOL_TCP: &str = "TCP";
pub const PROTOCOL_UDP: &str = "UDP";
pub const PROTOCOL_ICMP: &str = "ICMP";
pub const DIRECTION_IN: &str = "in";
pub const DIRECTION_OUT: &str = "out";
pub const SSE_DATA_PREFIX: &str = "data: ";
