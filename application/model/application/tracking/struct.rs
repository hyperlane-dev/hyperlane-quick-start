use super::*;

/// Application-level tracking configuration containing the tracking endpoint URL.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct Tracking {
    /// The URL endpoint where tracking data is reported.
    pub(super) url: String,
}

/// Application-level model representing a tracking record with parsed request headers and body.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct TrackingRecord {
    /// The parsed HTTP request headers.
    pub(super) headers: RequestHeaders,
    /// The serialized HTTP request body content.
    pub(super) body: String,
    /// The Unix timestamp (in milliseconds) when the request was received.
    #[get(type(copy))]
    pub(super) timestamp: i64,
}
