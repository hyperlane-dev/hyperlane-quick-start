use super::*;

/// The reactive state the redis page renders from.
#[derive(Clone, Copy, CustomDebug, Data, Default, New)]
pub struct RedisState {
    /// The records of the cache.
    #[get(type(copy))]
    pub records: Signal<Vec<RedisRecord>>,
    /// Whether a request is in flight.
    #[get(type(copy))]
    pub loading: Signal<bool>,
    /// The message of the last failure.
    #[get(type(copy))]
    pub error: Signal<String>,
    /// The message of the last success.
    #[get(type(copy))]
    pub notice: Signal<String>,
    /// The key of the record being created.
    #[get(type(copy))]
    pub create_key: Signal<String>,
    /// The value of the record being created.
    #[get(type(copy))]
    pub create_value: Signal<String>,
    /// Whether the editor of a record is open.
    #[get(type(copy))]
    pub edit_open: Signal<bool>,
    /// The key of the record being edited.
    #[get(type(copy))]
    pub edit_key: Signal<String>,
    /// The value of the record being edited.
    #[get(type(copy))]
    pub edit_value: Signal<String>,
}
