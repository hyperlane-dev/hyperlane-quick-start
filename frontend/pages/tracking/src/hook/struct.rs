use super::*;

/// The reactive state the tracking page renders from.
#[derive(Clone, Copy, CustomDebug, Data, Default, New)]
pub struct TrackingState {
    /// The start of the window the query looks at.
    #[get(type(copy))]
    pub start_time: Signal<String>,
    /// The end of the window the query looks at.
    #[get(type(copy))]
    pub end_time: Signal<String>,
    /// The header the query filters on.
    #[get(type(copy))]
    pub header_key: Signal<String>,
    /// The value of the header the query filters on.
    #[get(type(copy))]
    pub header_value: Signal<String>,
    /// The content the query filters on.
    #[get(type(copy))]
    pub body_content: Signal<String>,
    /// The page being displayed.
    #[get(type(copy))]
    pub page: Signal<i64>,
    /// The number of the records one page holds.
    #[get(type(copy))]
    pub page_size: Signal<i64>,
    /// The identifier of the record the query is anchored on.
    #[get(type(copy))]
    pub cache_id: Signal<i64>,
    /// The records of the page being displayed.
    #[get(type(copy))]
    pub records: Signal<Vec<TrackingRecord>>,
    /// The number of the records the filter holds.
    #[get(type(copy))]
    pub total: Signal<i64>,
    /// Whether a request is in flight.
    #[get(type(copy))]
    pub loading: Signal<bool>,
    /// The message of the last failure.
    #[get(type(copy))]
    pub error: Signal<String>,
    /// The record whose detail is open.
    #[get(type(copy))]
    pub detail: Signal<Option<TrackingRecord>>,
}
