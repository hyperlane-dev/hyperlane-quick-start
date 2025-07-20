use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, ToSchema)]
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub enum PacketDirection {
    Inbound,
    Outbound,
    Local,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub enum CaptureStatus {
    Stopped,
    Running,
    Paused,
    Error(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub enum NetworkInterface {
    Ethernet,
    WiFi,
    Loopback,
    VPN,
    Unknown(String),
}
