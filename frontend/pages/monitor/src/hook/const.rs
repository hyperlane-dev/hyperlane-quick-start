/// The endpoint of the static information of the server.
pub(crate) const PATH_SERVER_INFO: &str = "/api/server/info";

/// The endpoint of the live status stream of the server.
pub(crate) const PATH_SERVER_STATUS: &str = "/api/server/status";

/// The endpoint of the performance history of the server.
pub(crate) const PATH_SERVER_HISTORY: &str = "/api/server/performance/history";

/// The number of the history samples the page shows.
pub(crate) const HISTORY_SAMPLES: usize = 12;

/// The empty text of a field.
pub(crate) const TEXT_EMPTY: &str = "";

/// The message reported when the live status stream cannot be opened.
pub(crate) const MESSAGE_STREAM_FAILED: &str = "the live status stream could not be opened";

/// The number of the bytes in a kibibyte.
pub(crate) const BYTES_PER_KIBIBYTE: f64 = 1024.0;

/// The number of the bytes in a mebibyte.
pub(crate) const BYTES_PER_MEBIBYTE: f64 = 1048576.0;

/// The number of the bytes in a gibibyte.
pub(crate) const BYTES_PER_GIBIBYTE: f64 = 1073741824.0;

/// The number of the seconds in a minute.
pub(crate) const SECONDS_PER_MINUTE: i64 = 60;

/// The number of the seconds in an hour.
pub(crate) const SECONDS_PER_HOUR: i64 = 3600;

/// The number of the seconds in a day.
pub(crate) const SECONDS_PER_DAY: i64 = 86400;

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
