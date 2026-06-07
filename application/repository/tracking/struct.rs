use super::*;

/// Repository for performing database operations on the `tracking_record` table.
#[derive(Clone, Copy, Debug, Default)]
pub struct TrackingRepository;

/// Query parameters for filtering tracking records by time range with pagination.
#[derive(Clone, Data, Debug, Default)]
pub struct TrackingQuery {
    /// The optional start timestamp (inclusive) for filtering records.
    #[get(type(copy))]
    pub(super) start_time: Option<i64>,
    /// The optional end timestamp (exclusive) for filtering records.
    #[get(type(copy))]
    pub(super) end_time: Option<i64>,
    /// The page number for pagination (1-based).
    #[get(type(copy))]
    pub(super) page: i64,
    /// The number of records per page.
    #[get(type(copy))]
    pub(super) page_size: i64,
    /// The optional cache ID for cursor-based pagination.
    #[get(type(copy))]
    pub(super) cache_id: Option<i64>,
}

/// Query parameters for filtering tracking records by HTTP header key-value pairs with time range and pagination.
#[derive(Clone, Data, Debug, Default)]
pub struct TrackingHeaderQuery {
    /// The optional HTTP header key to search for.
    pub(super) header_key: Option<String>,
    /// The optional HTTP header value to search for.
    pub(super) header_value: Option<String>,
    /// The optional start timestamp (inclusive) for filtering records.
    #[get(type(copy))]
    pub(super) start_time: Option<i64>,
    /// The optional end timestamp (exclusive) for filtering records.
    #[get(type(copy))]
    pub(super) end_time: Option<i64>,
    /// The page number for pagination (1-based).
    #[get(type(copy))]
    pub(super) page: i64,
    /// The number of records per page.
    #[get(type(copy))]
    pub(super) page_size: i64,
    /// The optional cache ID for cursor-based pagination.
    #[get(type(copy))]
    pub(super) cache_id: Option<i64>,
}

/// Query parameters for filtering tracking records by HTTP body content with time range and pagination.
#[derive(Clone, Data, Debug, Default)]
pub struct TrackingBodyQuery {
    /// The optional text pattern to search within the HTTP body content.
    pub(super) body_content: Option<String>,
    /// The optional start timestamp (inclusive) for filtering records.
    #[get(type(copy))]
    pub(super) start_time: Option<i64>,
    /// The optional end timestamp (exclusive) for filtering records.
    #[get(type(copy))]
    pub(super) end_time: Option<i64>,
    /// The page number for pagination (1-based).
    #[get(type(copy))]
    pub(super) page: i64,
    /// The number of records per page.
    #[get(type(copy))]
    pub(super) page_size: i64,
    /// The optional cache ID for cursor-based pagination.
    #[get(type(copy))]
    pub(super) cache_id: Option<i64>,
}
