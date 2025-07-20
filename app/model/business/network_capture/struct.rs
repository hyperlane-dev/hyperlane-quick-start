use super::*;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct NetworkPacket {
    pub timestamp: u64,
    pub protocol: String,
    pub src_ip: String,
    pub dst_ip: String,
    pub src_port: u16,
    pub dst_port: u16,
    pub size: u32,
    pub direction: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct NetworkStats {
    pub total_packets: u64,
    pub total_bytes: u64,
    pub protocols: HashMap<String, u64>,
    pub top_connections: Vec<ConnectionInfo>,
    pub recent_packets: Vec<NetworkPacket>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ConnectionInfo {
    pub remote_ip: String,
    pub port: u16,
    pub protocol: String,
    pub packets: u64,
    pub bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct NetworkCaptureRequest {
    pub duration_seconds: Option<u64>,
    pub filter_protocol: Option<String>,
    pub filter_port: Option<u16>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct NetworkCaptureResponse {
    pub status: String,
    pub message: String,
    pub data: Option<NetworkStats>,
}
