use super::*;

/// The props of the redis page.
#[derive(Clone, Copy, Debug, Default)]
pub struct RedisPageProps {
    /// The state the page renders from.
    pub state: RedisState,
}

/// The props of one record of the list.
#[derive(Clone, Debug, Default)]
pub struct RecordProps {
    /// The state the page renders from.
    pub state: RedisState,
    /// The record the row shows.
    pub record: RedisRecord,
}
