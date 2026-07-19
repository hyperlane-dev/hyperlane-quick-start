use super::*;

/// tracking query request.
#[skip_serializing_none]
#[derive(Clone, Data, Debug, Deserialize, Serialize, ToSchema)]
pub struct TrackingQueryRequest {
    /// The start time.
    #[get(type(copy))]
    pub(super) start_time: Option<i64>,
    /// The end time.
    #[get(type(copy))]
    pub(super) end_time: Option<i64>,
    /// The header key.
    pub(super) header_key: Option<String>,
    /// The header value.
    pub(super) header_value: Option<String>,
    /// The body content.
    pub(super) body_content: Option<String>,
    /// The page.
    #[get(type(copy))]
    pub(super) page: Option<i64>,
    /// The page size.
    #[get(type(copy))]
    pub(super) page_size: Option<i64>,
    /// The cache id.
    #[get(type(copy))]
    pub(super) cache_id: Option<i64>,
}

/// tracking query response.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct TrackingQueryResponse {
    /// The total.
    #[get(type(copy))]
    pub(super) total: i64,
    /// The page.
    #[get(type(copy))]
    pub(super) page: i64,
    /// The page size.
    #[get(type(copy))]
    pub(super) page_size: i64,
    /// The records.
    pub(super) records: Vec<TrackingRecordDTO>,
}

/// Represents a tracking record data transfer object for API responses.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct TrackingRecordDTO {
    /// The id.
    #[get(type(copy))]
    pub(super) id: i64,
    /// The headers.
    pub(super) headers: serde_json::Value,
    /// The body.
    pub(super) body: String,
    /// The timestamp.
    #[get(type(copy))]
    pub(super) timestamp: i64,
    /// The created at.
    pub(super) created_at: i64,
}
