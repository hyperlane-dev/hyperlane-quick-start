use super::*;

#[derive(Data, DisplayDebug, CustomDebug, Deserialize, Default)]
pub struct Tracking {
    url: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TrackingRecord {
    pub socket_addr: String,
    pub headers: RequestHeaders,
    pub body: String,
    pub timestamp: i64,
}
