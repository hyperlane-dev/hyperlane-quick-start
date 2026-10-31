use super::*;

/// tracking query request.
#[skip_serializing_none]
#[derive(Clone, Data, Debug, Deserialize, Serialize, ToSchema)]
pub struct TrackingQueryRequest {
    /// The start time.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) start_time: Option<i64>,
    /// The end time.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) end_time: Option<i64>,
    /// The header key.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) header_key: Option<String>,
    /// The header value.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) header_value: Option<String>,
    /// The body content.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) body_content: Option<String>,
    /// The page.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) page: Option<i64>,
    /// The page size.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) page_size: Option<i64>,
    /// The cache id.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) cache_id: Option<i64>,
}

/// tracking query response.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct TrackingQueryResponse {
    /// The total.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) total: i64,
    /// The page.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) page: i64,
    /// The page size.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) page_size: i64,
    /// The records.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) records: Vec<TrackingRecordDTO>,
}

/// Represents a tracking record data transfer object for API responses.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct TrackingRecordDTO {
    /// The id.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) id: i64,
    /// The headers.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) headers: Value,
    /// The body.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) body: String,
    /// The timestamp.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) timestamp: i64,
    /// The created at.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) created_at: i64,
}
