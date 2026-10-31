use super::*;

/// Repository for performing database operations on the `tracking_record` table.
#[derive(Clone, Copy, Debug, Default)]
pub struct TrackingRepository;

/// Query parameters for filtering tracking records by time range with pagination.
#[derive(Clone, Data, Debug, Default)]
pub struct TrackingQuery {
    /// The optional start timestamp (inclusive) for filtering records.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) start_time: Option<i64>,
    /// The optional end timestamp (exclusive) for filtering records.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) end_time: Option<i64>,
    /// The page number for pagination (1-based).
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) page: i64,
    /// The number of records per page.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) page_size: i64,
    /// The optional cache ID for cursor-based pagination.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) cache_id: Option<i64>,
}

/// Query parameters for filtering tracking records by HTTP header key-value pairs with time range and pagination.
#[derive(Clone, Data, Debug, Default)]
pub struct TrackingHeaderQuery {
    /// The optional HTTP header key to search for.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) header_key: Option<String>,
    /// The optional HTTP header value to search for.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) header_value: Option<String>,
    /// The optional start timestamp (inclusive) for filtering records.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) start_time: Option<i64>,
    /// The optional end timestamp (exclusive) for filtering records.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) end_time: Option<i64>,
    /// The page number for pagination (1-based).
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) page: i64,
    /// The number of records per page.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) page_size: i64,
    /// The optional cache ID for cursor-based pagination.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) cache_id: Option<i64>,
}

/// Query parameters for filtering tracking records by HTTP body content with time range and pagination.
#[derive(Clone, Data, Debug, Default)]
pub struct TrackingBodyQuery {
    /// The optional text pattern to search within the HTTP body content.
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) body_content: Option<String>,
    /// The optional start timestamp (inclusive) for filtering records.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) start_time: Option<i64>,
    /// The optional end timestamp (exclusive) for filtering records.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) end_time: Option<i64>,
    /// The page number for pagination (1-based).
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) page: i64,
    /// The number of records per page.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) page_size: i64,
    /// The optional cache ID for cursor-based pagination.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(crate) cache_id: Option<i64>,
}
