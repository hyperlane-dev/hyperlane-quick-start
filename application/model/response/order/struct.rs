use super::*;

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct UserResponse {
    #[get(type(copy), pub)]
    pub(super) id: i32,
    pub(super) username: String,
    pub(super) nickname: Option<String>,
    pub(super) email: Option<String>,
    pub(super) phone: Option<String>,
    pub(super) role: String,
    pub(super) status: String,
    pub(super) created_at: Option<String>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct LoginResponse {
    pub(super) user: UserResponse,
    pub(super) token: String,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct UserListResponse {
    pub(super) users: Vec<UserResponse>,
    #[get(type(copy), pub)]
    pub(super) has_more: bool,
    #[get(type(copy), pub)]
    pub(super) last_id: Option<i32>,
    #[get(type(copy), pub)]
    pub(super) total_count: i64,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct RecordResponse {
    #[get(type(copy), pub)]
    pub(super) id: i32,
    pub(super) bill_no: String,
    #[get(type(copy), pub)]
    pub(super) user_id: i32,
    pub(super) amount: String,
    pub(super) category: String,
    pub(super) transaction_type: String,
    pub(super) description: Option<String>,
    pub(super) bill_date: String,
    pub(super) created_at: Option<String>,
    pub(super) username: Option<String>,
    pub(super) nickname: Option<String>,
    pub(super) email: Option<String>,
    pub(super) phone: Option<String>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct RecordListResponse {
    pub(super) records: Vec<RecordResponse>,
    pub(super) total_income: String,
    pub(super) total_expense: String,
    pub(super) balance: String,
    #[get(type(copy), pub)]
    pub(super) has_more: bool,
    #[get(type(copy), pub)]
    pub(super) last_id: Option<i32>,
    #[get(type(copy), pub)]
    pub(super) total_count: i64,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct TodayStatistics {
    #[get(type(copy), pub)]
    pub(super) transactions: i64,
    pub(super) income: String,
    pub(super) expense: String,
    #[get(type(copy), pub)]
    pub(super) new_users: i64,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct ChangesStatistics {
    #[get(type(copy), pub)]
    pub(super) transactions_change: Option<f64>,
    #[get(type(copy), pub)]
    pub(super) income_change: Option<f64>,
    #[get(type(copy), pub)]
    pub(super) expense_change: Option<f64>,
    #[get(type(copy), pub)]
    pub(super) new_users_change: Option<f64>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct DailyTrend {
    pub(super) dates: Vec<String>,
    pub(super) income: Vec<String>,
    pub(super) expense: Vec<String>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct MonthlyComparison {
    pub(super) months: Vec<String>,
    pub(super) income: Vec<String>,
    pub(super) expense: Vec<String>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct CategoryItem {
    pub(super) name: String,
    #[get(type(copy), pub)]
    pub(super) value: i64,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct UserGrowth {
    pub(super) dates: Vec<String>,
    pub(super) counts: Vec<i64>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct TransactionTypeDistribution {
    #[get(type(copy), pub)]
    pub(super) income_count: i64,
    #[get(type(copy), pub)]
    pub(super) expense_count: i64,
    pub(super) income_amount: String,
    pub(super) expense_amount: String,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct TransactionCountTrend {
    pub(super) dates: Vec<String>,
    pub(super) counts: Vec<i64>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct CategoryAmountItem {
    pub(super) name: String,
    pub(super) amount: String,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct UserActivity {
    pub(super) dates: Vec<String>,
    pub(super) active_users: Vec<i64>,
    pub(super) new_records: Vec<i64>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct OverviewStatisticsResponse {
    pub(super) today: TodayStatistics,
    pub(super) changes: ChangesStatistics,
    pub(super) daily_trend: DailyTrend,
    pub(super) monthly_comparison: MonthlyComparison,
    pub(super) category_distribution: Vec<CategoryItem>,
    pub(super) user_growth: UserGrowth,
    pub(super) transaction_type_distribution: TransactionTypeDistribution,
    pub(super) transaction_count_trend: TransactionCountTrend,
    pub(super) category_amount_distribution: Vec<CategoryAmountItem>,
    pub(super) user_activity: UserActivity,
}
