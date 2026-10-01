/// Maximum number of records allowed per page in list queries.
pub const MAX_LIMIT: u64 = 100;

/// Time constant for week days.
pub const WEEK_DAYS: &[&str] = &["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];

/// Key for record id key.
pub const RECORD_ID_KEY: &str = "record_id";
/// Transaction type name for income.
pub const TRANSACTION_TYPE_INCOME: &str = "income";

/// Transaction type name for expense.
pub const TRANSACTION_TYPE_EXPENSE: &str = "expense";
