pub const TOP_CONNECTIONS_LIMIT: usize = 10;

pub const WIN_NETSTAT_COMMAND: &'static str = "netstat";
pub const WIN_NETSTAT_ARGS: &[&'static str] = &["-an", "-p", "TCP"];
pub const WIN_NETSTAT_SKIP_LINES: usize = 4;
pub const WIN_POWERSHELL_COMMAND: &'static str = "powershell";
pub const WIN_POWERSHELL_ARG: &'static str = "-Command";
pub const WIN_PERF_COUNTER_SCRIPT: &'static str = r#"\
Get-Counter '\\Network Interface(*)\\Packets/sec', '\\Network Interface(*)\\Bytes Total/sec' -SampleInterval 1 -MaxSamples 1 |
ForEach-Object { $_.CounterSamples } |
Where-Object { $_.InstanceName -notlike '*Loopback*' -and $_.InstanceName -ne '_Total' } |
Measure-Object -Property CookedValue -Sum |
Select-Object -ExpandProperty Sum
"#;
pub const WIN_DEFAULT_PACKET_BYTES: u64 = 1024;

pub const LINUX_SS_COMMAND: &'static str = "ss";
pub const LINUX_SS_ARGS: &[&'static str] = &["-tuln"];
pub const LINUX_NET_DEV_PATH: &'static str = "/proc/net/dev";
pub const LINUX_NET_DEV_SKIP_LINES: usize = 2;

pub const SAMPLE_PACKET_COUNT: u64 = 10;
pub const SAMPLE_PACKET_BASE_SIZE: u32 = 1024;
pub const SAMPLE_PACKET_SIZE_MULTIPLIER: u32 = 100;
pub const SAMPLE_IP_PREFIX_A: &'static str = "192.168.1.";
pub const SAMPLE_IP_PREFIX_B: &'static str = "8.8.8.";
pub const SAMPLE_BASE_SRC_PORT: u16 = 50000;
pub const SAMPLE_DST_PORT_A: u16 = 80;
pub const SAMPLE_DST_PORT_B: u16 = 443;
pub const PROTOCOL_TCP: &'static str = "TCP";
pub const PROTOCOL_UDP: &'static str = "UDP";
pub const PROTOCOL_ICMP: &'static str = "ICMP";
pub const DIRECTION_IN: &'static str = "in";
pub const DIRECTION_OUT: &'static str = "out";

pub const SSE_DATA_PREFIX: &'static str = "data: ";
pub const SSE_EVENT_SUFFIX: &'static str = "\n\n";
