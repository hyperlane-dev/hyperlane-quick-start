use super::*;

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct Tracking {
    pub(super) url: String,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct TrackingRecord {
    pub(super) socket_addr: String,
    pub(super) headers: RequestHeaders,
    pub(super) body: String,
    #[get(type(copy), pub)]
    pub(super) timestamp: i64,
}
