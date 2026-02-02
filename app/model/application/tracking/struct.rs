use super::*;

#[derive(CustomDebug, Data, Default, Deserialize, DisplayDebug)]
pub struct Tracking {
    url: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TrackingRecord {
    pub socket_addr: String,
    pub headers: RequestHeaders,
    pub body: String,
    pub timestamp: i64,
}
