/// Default capture duration.
pub const DEFAULT_CAPTURE_DURATION: u64 = 60;

/// Maximum capture duration.
pub const MAX_CAPTURE_DURATION: u64 = 3600;

/// Default packet buffer size.
pub const DEFAULT_PACKET_BUFFER_SIZE: usize = 1000;

/// Maximum connections display.
pub const MAX_CONNECTIONS_DISPLAY: usize = 50;

/// Interval value for monitor seconds.
pub const MONITOR_INTERVAL_SECONDS: u64 = 60;

/// Cache or TTL value for network stats ttl.
pub const NETWORK_STATS_CACHE_TTL: u64 = 30;

/// Limit value for top connections limit.
pub const TOP_CONNECTIONS_LIMIT: usize = 10;

/// Sample packet count.
pub const SAMPLE_PACKET_COUNT: u64 = 10;

/// Sample packet base size.
pub const SAMPLE_PACKET_BASE_SIZE: u32 = 1024;

/// Sample packet size multiplier.
pub const SAMPLE_PACKET_SIZE_MULTIPLIER: u32 = 100;

/// Sample ip prefix a.
pub const SAMPLE_IP_PREFIX_A: &str = "192.168.1.";

/// Sample ip prefix b.
pub const SAMPLE_IP_PREFIX_B: &str = "8.8.8.";

/// Sample base src port.
pub const SAMPLE_BASE_SRC_PORT: usize = 50000;

/// Sample dst port a.
pub const SAMPLE_DST_PORT_A: usize = 80;

/// Sample dst port b.
pub const SAMPLE_DST_PORT_B: usize = 443;

/// Protocol identifier for tcp.
pub const PROTOCOL_TCP: &str = "TCP";

/// Protocol identifier for udp.
pub const PROTOCOL_UDP: &str = "UDP";

/// Protocol identifier for icmp.
pub const PROTOCOL_ICMP: &str = "ICMP";

/// Direction label for in network traffic.
pub const DIRECTION_IN: &str = "in";

/// Direction label for out network traffic.
pub const DIRECTION_OUT: &str = "out";

/// SSE data prefix.
pub const SSE_DATA_PREFIX: &str = "data: ";

/// Maximum history seconds.
pub const MAX_HISTORY_SECONDS: usize = 60;
