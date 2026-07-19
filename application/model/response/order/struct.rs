use super::*;

/// Represents an order record response with bill details and associated images.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct RecordResponse {
    /// The id.
    pub(super) id: String,
    /// The bill no.
    pub(super) bill_no: String,
    /// The user id.
    pub(super) user_id: String,
    /// The amount.
    pub(super) amount: String,
    /// The category.
    pub(super) category: String,
    /// The transaction type.
    pub(super) transaction_type: String,
    /// The description.
    pub(super) description: Option<String>,
    /// The bill date.
    pub(super) bill_date: i64,
    /// The created at.
    pub(super) created_at: Option<i64>,
    /// The username.
    pub(super) username: Option<String>,
    /// The email.
    pub(super) email: Option<String>,
    /// The phone.
    pub(super) phone: Option<String>,
    /// The images.
    pub(super) images: Vec<RecordImageResponse>,
}

/// record list response.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct RecordListResponse {
    /// The records.
    pub(super) records: Vec<RecordResponse>,
    /// The total income.
    pub(super) total_income: String,
    /// The total expense.
    pub(super) total_expense: String,
    /// The balance.
    pub(super) balance: String,
    /// The total count.
    #[get(type(copy))]
    pub(super) total_count: i64,
}

/// Represents today's transaction statistics including income, expense, and new user count.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct TodayStatistics {
    /// The transactions.
    #[get(type(copy))]
    pub(super) transactions: i64,
    /// The income.
    pub(super) income: String,
    /// The expense.
    pub(super) expense: String,
    /// The new users.
    #[get(type(copy))]
    pub(super) new_users: i64,
}

/// Represents period-over-period changes in transaction metrics.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct ChangesStatistics {
    /// The transactions change.
    #[get(type(copy))]
    pub(super) transactions_change: Option<f64>,
    /// The income change.
    #[get(type(copy))]
    pub(super) income_change: Option<f64>,
    /// The expense change.
    #[get(type(copy))]
    pub(super) expense_change: Option<f64>,
    /// The new users change.
    #[get(type(copy))]
    pub(super) new_users_change: Option<f64>,
    /// The avg income change.
    #[get(type(copy))]
    pub(super) avg_income_change: Option<f64>,
    /// The avg expense change.
    #[get(type(copy))]
    pub(super) avg_expense_change: Option<f64>,
}

/// Represents daily income and expense trend data.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct DailyTrend {
    /// The dates.
    pub(super) dates: Vec<String>,
    /// The income.
    pub(super) income: Vec<String>,
    /// The expense.
    pub(super) expense: Vec<String>,
}

/// Represents monthly income and expense comparison data.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct MonthlyComparison {
    /// The months.
    pub(super) months: Vec<String>,
    /// The income.
    pub(super) income: Vec<String>,
    /// The expense.
    pub(super) expense: Vec<String>,
}

/// Represents a category breakdown item with name and value.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct CategoryItem {
    /// The name.
    pub(super) name: String,
    /// The value.
    #[get(type(copy))]
    pub(super) value: i64,
}

/// Represents user registration growth trend data.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct UserGrowth {
    /// The dates.
    pub(super) dates: Vec<String>,
    /// The counts.
    pub(super) counts: Vec<i64>,
}

/// transaction type distribution.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct TransactionTypeDistribution {
    /// The income count.
    #[get(type(copy))]
    pub(super) income_count: i64,
    /// The expense count.
    #[get(type(copy))]
    pub(super) expense_count: i64,
    /// The income amount.
    pub(super) income_amount: String,
    /// The expense amount.
    pub(super) expense_amount: String,
}

/// transaction count trend.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct TransactionCountTrend {
    /// The dates.
    pub(super) dates: Vec<String>,
    /// The counts.
    pub(super) counts: Vec<i64>,
}

/// category amount item.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct CategoryAmountItem {
    /// The name.
    pub(super) name: String,
    /// The amount.
    pub(super) amount: String,
}

/// Represents user activity trend data with active users and new records.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct UserActivity {
    /// The dates.
    pub(super) dates: Vec<String>,
    /// The active users.
    pub(super) active_users: Vec<i64>,
    /// The new records.
    pub(super) new_records: Vec<i64>,
}

/// income expense ratio item.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct IncomeExpenseRatioItem {
    /// The date.
    pub(super) date: String,
    /// The ratio.
    pub(super) ratio: f64,
    /// The income.
    pub(super) income: String,
    /// The expense.
    pub(super) expense: String,
}

/// hourly distribution item.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct HourlyDistributionItem {
    /// The hour.
    pub(super) hour: i32,
    /// The count.
    pub(super) count: i64,
    /// The income.
    pub(super) income: String,
    /// The expense.
    pub(super) expense: String,
}

/// Represents a weekly trend item with day-of-week breakdown.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct WeeklyTrendItem {
    /// The day of week.
    pub(super) day_of_week: String,
    /// The income.
    pub(super) income: String,
    /// The expense.
    pub(super) expense: String,
    /// The transaction count.
    pub(super) transaction_count: i64,
}

/// period over period item.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct PeriodOverPeriodItem {
    /// The period.
    pub(super) period: String,
    /// The income change.
    pub(super) income_change: f64,
    /// The expense change.
    pub(super) expense_change: f64,
    /// The transaction change.
    pub(super) transaction_change: f64,
}

/// Represents a category trend item with amounts over time.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct CategoryTrendItem {
    /// The category.
    pub(super) category: String,
    /// The dates.
    pub(super) dates: Vec<String>,
    /// The amounts.
    pub(super) amounts: Vec<String>,
}

/// Represents a user retention data point with rate calculation.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct UserRetentionItem {
    /// The date.
    pub(super) date: String,
    /// The new users.
    pub(super) new_users: i64,
    /// The retained users.
    pub(super) retained_users: i64,
    /// The retention rate.
    pub(super) retention_rate: f64,
}

/// Represents a top user by transaction volume.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct TopUserItem {
    /// The user id.
    pub(super) user_id: String,
    /// The username.
    pub(super) username: String,
    /// The transaction count.
    pub(super) transaction_count: i64,
    /// The total amount.
    pub(super) total_amount: String,
}

/// average transaction stats.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct AverageTransactionStats {
    /// The avg income per transaction.
    pub(super) avg_income_per_transaction: String,
    /// The avg expense per transaction.
    pub(super) avg_expense_per_transaction: String,
    /// The overall avg amount.
    pub(super) overall_avg_amount: String,
    /// The max single income.
    pub(super) max_single_income: String,
    /// The max single expense.
    pub(super) max_single_expense: String,
}

/// record image response.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct RecordImageResponse {
    /// The id.
    pub(super) id: String,
    /// The record id.
    pub(super) record_id: String,
    /// The user id.
    pub(super) user_id: String,
    /// The file name.
    pub(super) file_name: String,
    /// The original name.
    pub(super) original_name: Option<String>,
    /// The mime type.
    pub(super) mime_type: String,
    /// The file size.
    #[get(type(copy))]
    pub(super) file_size: i32,
    /// The created at.
    pub(super) created_at: i64,
    /// The download url.
    pub(super) download_url: String,
    /// The username.
    pub(super) username: String,
}

/// record image list response.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct RecordImageListResponse {
    /// The images.
    pub(super) images: Vec<RecordImageResponse>,
    /// The total count.
    #[get(type(copy))]
    pub(super) total_count: i32,
}

/// record image detail response.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct RecordImageDetailResponse {
    /// The id.
    pub(super) id: String,
    /// The record id.
    pub(super) record_id: String,
    /// The user id.
    pub(super) user_id: String,
    /// The file name.
    pub(super) file_name: String,
    /// The original name.
    pub(super) original_name: Option<String>,
    /// The mime type.
    pub(super) mime_type: String,
    /// The file size.
    #[get(type(copy))]
    pub(super) file_size: i32,
    /// The file data base64.
    pub(super) file_data_base64: String,
    /// The created at.
    pub(super) created_at: i64,
}

/// Represents an image data response with binary file content.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct ImageDataResponse {
    /// The id.
    pub(super) id: String,
    /// The record id.
    pub(super) record_id: String,
    /// The file name.
    pub(super) file_name: String,
    /// The original name.
    pub(super) original_name: Option<String>,
    /// The mime type.
    pub(super) mime_type: String,
    /// The file data.
    pub(super) file_data: Vec<u8>,
}

/// create record with images response.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct CreateRecordWithImagesResponse {
    /// The record.
    pub(super) record: RecordResponse,
    /// The images.
    pub(super) images: Vec<RecordImageResponse>,
}

/// overview statistics response.
#[derive(Clone, Data, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct OverviewStatisticsResponse {
    /// The today.
    pub(super) today: TodayStatistics,
    /// The changes.
    pub(super) changes: ChangesStatistics,
    /// The daily trend.
    pub(super) daily_trend: DailyTrend,
    /// The monthly comparison.
    pub(super) monthly_comparison: MonthlyComparison,
    /// The category distribution.
    pub(super) category_distribution: Vec<CategoryItem>,
    /// The user growth.
    pub(super) user_growth: UserGrowth,
    /// The transaction type distribution.
    pub(super) transaction_type_distribution: TransactionTypeDistribution,
    /// The transaction count trend.
    pub(super) transaction_count_trend: TransactionCountTrend,
    /// The category amount distribution.
    pub(super) category_amount_distribution: Vec<CategoryAmountItem>,
    /// The user activity.
    pub(super) user_activity: UserActivity,
    /// The income expense ratio trend.
    pub(super) income_expense_ratio_trend: Vec<IncomeExpenseRatioItem>,
    /// The hourly distribution.
    pub(super) hourly_distribution: Vec<HourlyDistributionItem>,
    /// The weekly trend.
    pub(super) weekly_trend: Vec<WeeklyTrendItem>,
    /// The period over period.
    pub(super) period_over_period: Vec<PeriodOverPeriodItem>,
    /// The category trends.
    pub(super) category_trends: Vec<CategoryTrendItem>,
    /// The user retention.
    pub(super) user_retention: Vec<UserRetentionItem>,
    /// The top users.
    pub(super) top_users: Vec<TopUserItem>,
    /// The avg transaction stats.
    pub(super) avg_transaction_stats: AverageTransactionStats,
}
