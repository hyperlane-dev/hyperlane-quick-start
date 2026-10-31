/// Maximum number of records allowed per page in list queries.
pub(crate) const MAX_LIMIT: u64 = 100;

/// Time constant for week days.
pub(crate) const WEEK_DAYS: &[&str] = &["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];

/// Key for record id key.
pub(crate) const RECORD_ID_KEY: &str = "record_id";
/// Transaction type name for income.
pub(crate) const TRANSACTION_TYPE_INCOME: &str = "income";

/// Transaction type name for expense.
pub(crate) const TRANSACTION_TYPE_EXPENSE: &str = "expense";
