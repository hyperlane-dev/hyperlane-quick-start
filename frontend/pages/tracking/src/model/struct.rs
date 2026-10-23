use super::*;

/// One stored request of the tracking service.
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(default)]
pub struct TrackingRecord {
    /// The identifier of the record.
    pub id: i64,
    /// The headers of the request, as the server stored them.
    pub headers: serde_json::Value,
    /// The body of the request.
    pub body: String,
    /// The moment the request was received, in milliseconds since the Unix epoch.
    pub timestamp: i64,
    /// The moment the record was written, in milliseconds since the Unix epoch.
    pub created_at: i64,
}

/// The answer of a tracking query.
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(default)]
pub struct TrackingQueryResult {
    /// The number of the records the filter holds.
    pub total: i64,
    /// The page the answer holds.
    pub page: i64,
    /// The number of the records per page.
    pub page_size: i64,
    /// The records of the page.
    pub records: Vec<TrackingRecord>,
}

/// The body of a tracking query.
#[derive(Clone, Debug, Default, Serialize)]
pub struct TrackingQuery {
    /// The start of the window, in milliseconds since the Unix epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    /// The end of the window, in milliseconds since the Unix epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
    /// The header the filter looks for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header_key: Option<String>,
    /// The value the filter looks for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header_value: Option<String>,
    /// The content the filter looks for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_content: Option<String>,
    /// The page the query asks for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    /// The number of the records per page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// The identifier of the record the query is anchored on.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_id: Option<i64>,
}
