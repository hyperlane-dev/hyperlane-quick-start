use super::*;

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

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema)]
pub enum PacketDirection {
    Inbound,
    Outbound,
    Local,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema)]
pub enum CaptureStatus {
    Stopped,
    Running,
    Paused,
    Error(String),
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, ToSchema)]
pub enum NetworkInterface {
    Ethernet,
    WiFi,
    Loopback,
    VPN,
    Unknown(String),
}
