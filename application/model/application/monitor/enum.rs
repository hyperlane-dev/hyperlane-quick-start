use super::*;

/// Enumeration of network protocol.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize, ToSchema)]
pub enum NetworkProtocol {
    TCP,
    UDP,
    ICMP,
    HTTP,
    HTTPS,
    FTP,
    SSH,
    DNS,
    DHCP,
    Unknown(String),
}

/// Enumeration of packet direction.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema)]
pub enum PacketDirection {
    Inbound,
    Outbound,
    Local,
}

/// Enumeration of capture status.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema)]
pub enum CaptureStatus {
    Stopped,
    Running,
    Paused,
    Error(String),
}

/// Enumeration of network interface.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema)]
pub enum NetworkInterface {
    Ethernet,
    WiFi,
    Loopback,
    VPN,
    Unknown(String),
}
