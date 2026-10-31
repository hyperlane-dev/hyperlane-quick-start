use super::*;

/// Application-level tracking configuration containing the tracking endpoint URL.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct Tracking {
    /// The URL endpoint where tracking data is reported.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) url: String,
}

/// Application-level model representing a tracking record with parsed request headers and body.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct TrackingRecord {
    /// The parsed HTTP request headers.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) headers: RequestHeaders,
    /// The serialized HTTP request body content.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) body: String,
    /// The Unix timestamp (in milliseconds) when the request was received.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) timestamp: i64,
}
