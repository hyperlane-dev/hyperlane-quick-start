use super::*;

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct Tracking {
    #[get(pub)]
    pub(super) url: String,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct TrackingRecord {
    #[get(pub)]
    pub(super) socket_addr: String,
    #[get(pub)]
    pub(super) headers: RequestHeaders,
    #[get(pub)]
    pub(super) body: String,
    #[get(type(copy), pub)]
    pub(super) timestamp: i64,
}
