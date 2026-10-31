/// Period comparison name for qoq analysis.
pub const PERIOD_NAME_QOQ: &str = "QoQ";

/// Time constant for daily trend days.
pub const DAILY_TREND_DAYS: i64 = 30;

/// Time constant for monthly comparison months.
pub const MONTHLY_COMPARISON_MONTHS: i64 = 6;

/// Limit value for top users limit.
pub const TOP_USERS_LIMIT: i64 = 10;

/// URL hourly distribution days.
pub const HOURLY_DISTRIBUTION_DAYS: i64 = 30;

/// Time constant for weekly trend days.
pub const WEEKLY_TREND_DAYS: i64 = 90;

/// Template string for image download url template.
pub const IMAGE_DOWNLOAD_URL_TEMPLATE: &str = "/api/order/image/download/";

/// Error message when record not found.
pub(crate) const ERROR_RECORD_NOT_FOUND: &str = "Record not found";

/// Error message when only admin can access image data.
pub(crate) const ERROR_ONLY_ADMIN_CAN_ACCESS_IMAGE_DATA: &str = "Only admin can access image data";

/// Prefix for prefix.
pub(crate) const BILL_NO_PREFIX: &str = "BILL";

/// Period comparison name for wow analysis.
pub(crate) const PERIOD_NAME_WOW: &str = "WoW";

/// Period comparison name for mom analysis.
pub(crate) const PERIOD_NAME_MOM: &str = "MoM";

/// Default page number.
pub(crate) const DEFAULT_PAGE_NUMBER: i32 = 1;
