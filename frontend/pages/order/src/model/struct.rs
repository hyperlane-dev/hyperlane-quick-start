use super::*;

/// The public key the fields are encrypted with.
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(default)]
pub struct RsaPublicKey {
    /// The base64 encoded modulus of the key.
    pub modulus: String,
    /// The base64 encoded public exponent of the key.
    pub exponent: String,
}

/// One record of the ledger.
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(default)]
pub struct RecordItem {
    /// The opaque identifier of the record.
    pub id: String,
    /// The number of the bill of the record.
    pub bill_no: String,
    /// The identifier of the user the record belongs to.
    pub user_id: String,
    /// The amount of the record.
    pub amount: String,
    /// The category of the record.
    pub category: String,
    /// The kind of the record.
    pub transaction_type: String,
    /// The description of the record.
    pub description: Option<String>,
    /// The day the bill of the record was written on.
    pub bill_date: i64,
    /// The creation time in milliseconds since the Unix epoch.
    pub created_at: Option<i64>,
    /// The name of the user the record belongs to.
    pub username: Option<String>,
    /// The address of the user the record belongs to.
    pub email: Option<String>,
    /// The phone number of the user the record belongs to.
    pub phone: Option<String>,
}

/// One page of the records of the ledger.
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(default)]
pub struct RecordList {
    /// The records of the page.
    pub records: Vec<RecordItem>,
    /// The income the records of the filter hold.
    pub total_income: String,
    /// The expense the records of the filter hold.
    pub total_expense: String,
    /// The balance the records of the filter hold.
    pub balance: String,
    /// The number of records the filter holds.
    pub total_count: i64,
}

/// The statistics of the current day.
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(default)]
pub struct TodayStatistics {
    /// The number of the transactions of the day.
    pub transactions: i64,
    /// The income of the day.
    pub income: String,
    /// The expense of the day.
    pub expense: String,
    /// The number of the users that joined the day.
    pub new_users: i64,
}

/// The change of the metrics against the day before.
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(default)]
pub struct ChangesStatistics {
    /// The change of the number of transactions.
    pub transactions_change: Option<f64>,
    /// The change of the income.
    pub income_change: Option<f64>,
    /// The change of the expense.
    pub expense_change: Option<f64>,
    /// The change of the number of new users.
    pub new_users_change: Option<f64>,
    /// The change of the average income.
    pub avg_income_change: Option<f64>,
    /// The change of the average expense.
    pub avg_expense_change: Option<f64>,
}

/// The averages of the transactions of the ledger.
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(default)]
pub struct AverageTransactionStats {
    /// The average income of one transaction.
    pub avg_income_per_transaction: String,
    /// The average expense of one transaction.
    pub avg_expense_per_transaction: String,
    /// The average amount of every transaction.
    pub overall_avg_amount: String,
    /// The largest income of one transaction.
    pub max_single_income: String,
    /// The largest expense of one transaction.
    pub max_single_expense: String,
}

/// The statistics of the ledger of the server.
///
/// The page renders the headline numbers of the statistics, the charts the
/// server offers with them are left out.
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(default)]
pub struct OverviewStatistics {
    /// The statistics of the current day.
    pub today: TodayStatistics,
    /// The change of the metrics against the day before.
    pub changes: ChangesStatistics,
    /// The averages of the transactions of the ledger.
    pub avg_transaction_stats: AverageTransactionStats,
}

/// The profile of a user.
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(default)]
pub struct UserProfile {
    /// The opaque identifier of the user.
    pub id: String,
    /// The name of the user.
    pub username: String,
    /// The address of the user.
    pub email: Option<String>,
    /// The phone number of the user.
    pub phone: Option<String>,
    /// The role of the user.
    pub role: String,
    /// The status of the user.
    pub status: String,
    /// The creation time in milliseconds since the Unix epoch.
    pub created_at: Option<i64>,
}

/// One page of the users of the server.
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(default)]
pub struct UserList {
    /// The users of the page.
    pub users: Vec<UserProfile>,
    /// Whether the server holds more users than the page holds.
    pub has_more: bool,
    /// The identifier the next page starts after.
    pub last_id: Option<String>,
    /// The number of users the server holds.
    pub total_count: i64,
}

/// The record a visitor writes.
#[derive(Clone, Debug, Default, Serialize)]
pub struct CreateRecordRequest {
    /// The amount of the record.
    pub amount: String,
    /// The category of the record.
    pub category: String,
    /// The kind of the record.
    pub transaction_type: String,
    /// The description of the record.
    pub description: Option<String>,
    /// The day the bill of the record was written on.
    pub bill_date: Option<String>,
    /// The identifier of the user the record belongs to.
    pub target_user_id: Option<String>,
    /// The identifiers of the images of the record.
    pub image_ids: Vec<String>,
}

/// The contact fields of a user.
#[derive(Clone, Debug, Default, Serialize)]
pub struct UpdateUserRequest {
    /// The new address of the user.
    pub email: Option<String>,
    /// The new phone number of the user.
    pub phone: Option<String>,
}

/// The passwords of a user.
#[derive(Clone, Debug, Default, Serialize)]
pub struct ChangePasswordRequest {
    /// The current password of the user.
    pub old_password: String,
    /// The new password of the user.
    pub new_password: String,
}
