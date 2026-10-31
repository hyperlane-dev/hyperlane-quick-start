/// The endpoint of the tracking query.
pub(crate) const PATH_TRACKING_QUERY: &str = "/api/tracking/query";

/// The number of the records one page holds.
pub(crate) const PAGE_SIZE: i64 = 50;

/// The first page of the records.
pub(crate) const PAGE_FIRST: i64 = 1;

/// The empty text of a field.
pub(crate) const TEXT_EMPTY: &str = "";

/// The number of the milliseconds in a minute.
pub(crate) const MILLIS_PER_MINUTE: i64 = 60000;

/// The number of the milliseconds in an hour.
pub(crate) const MILLIS_PER_HOUR: i64 = 3600000;

/// The number of the milliseconds in a second.
pub(crate) const MILLIS_PER_SECOND: i64 = 1000;

/// The number of the milliseconds in a day.
pub(crate) const MILLIS_PER_DAY: i64 = 86400000;

/// The number of the days between the epoch and the start of the era.
pub(crate) const DAYS_SHIFT: i64 = 719468;

/// The number of the days in an era of four hundred years.
pub(crate) const DAYS_PER_ERA: i64 = 146097;

/// The number of the years in an era of four hundred years.
pub(crate) const YEARS_PER_ERA: i64 = 400;
