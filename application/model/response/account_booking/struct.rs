use super::*;

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct UserResponse {
    #[get(type(copy), pub)]
    pub(super) id: i32,
    #[get(pub)]
    pub(super) username: String,
    #[get(pub)]
    pub(super) nickname: Option<String>,
    #[get(pub)]
    pub(super) email: Option<String>,
    #[get(pub)]
    pub(super) phone: Option<String>,
    #[get(pub)]
    pub(super) role: String,
    #[get(pub)]
    pub(super) status: String,
    #[get(pub)]
    pub(super) created_at: Option<String>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct LoginResponse {
    #[get(pub)]
    pub(super) user: UserResponse,
    #[get(pub)]
    pub(super) token: String,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct RecordResponse {
    #[get(type(copy), pub)]
    pub(super) id: i32,
    #[get(pub)]
    pub(super) bill_no: String,
    #[get(type(copy), pub)]
    pub(super) user_id: i32,
    #[get(pub)]
    pub(super) amount: String,
    #[get(pub)]
    pub(super) category: String,
    #[get(pub)]
    pub(super) transaction_type: String,
    #[get(pub)]
    pub(super) description: Option<String>,
    #[get(pub)]
    pub(super) bill_date: String,
    #[get(pub)]
    pub(super) created_at: Option<String>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct RecordListResponse {
    #[get(pub)]
    pub(super) records: Vec<RecordResponse>,
    #[get(pub)]
    pub(super) total_income: String,
    #[get(pub)]
    pub(super) total_expense: String,
    #[get(pub)]
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
    #[set(pub)]
    pub(super) transactions: i64,
    #[get(pub)]
    #[set(pub)]
    pub(super) income: String,
    #[get(pub)]
    #[set(pub)]
    pub(super) expense: String,
    #[get(type(copy), pub)]
    #[set(pub)]
    pub(super) new_users: i64,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct ChangesStatistics {
    #[get(type(copy), pub)]
    #[set(pub)]
    pub(super) transactions_change: Option<f64>,
    #[get(type(copy), pub)]
    #[set(pub)]
    pub(super) income_change: Option<f64>,
    #[get(type(copy), pub)]
    #[set(pub)]
    pub(super) expense_change: Option<f64>,
    #[get(type(copy), pub)]
    #[set(pub)]
    pub(super) new_users_change: Option<f64>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct DailyTrend {
    #[get(pub)]
    #[set(pub)]
    pub(super) dates: Vec<String>,
    #[get(pub)]
    #[set(pub)]
    pub(super) income: Vec<String>,
    #[get(pub)]
    #[set(pub)]
    pub(super) expense: Vec<String>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct MonthlyComparison {
    #[get(pub)]
    #[set(pub)]
    pub(super) months: Vec<String>,
    #[get(pub)]
    #[set(pub)]
    pub(super) income: Vec<String>,
    #[get(pub)]
    #[set(pub)]
    pub(super) expense: Vec<String>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct CategoryItem {
    #[get(pub)]
    pub(super) name: String,
    #[get(type(copy), pub)]
    pub(super) value: i64,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct UserGrowth {
    #[get(pub)]
    #[set(pub)]
    pub(super) dates: Vec<String>,
    #[get(pub)]
    #[set(pub)]
    pub(super) counts: Vec<i64>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct TransactionTypeDistribution {
    #[get(type(copy), pub)]
    #[set(pub)]
    pub(super) income_count: i64,
    #[get(type(copy), pub)]
    #[set(pub)]
    pub(super) expense_count: i64,
    #[get(pub)]
    #[set(pub)]
    pub(super) income_amount: String,
    #[get(pub)]
    #[set(pub)]
    pub(super) expense_amount: String,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct TransactionCountTrend {
    #[get(pub)]
    #[set(pub)]
    pub(super) dates: Vec<String>,
    #[get(pub)]
    #[set(pub)]
    pub(super) counts: Vec<i64>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct CategoryAmountItem {
    #[get(pub)]
    pub(super) name: String,
    #[get(pub)]
    #[set(pub)]
    pub(super) amount: String,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct UserActivity {
    #[get(pub)]
    #[set(pub)]
    pub(super) dates: Vec<String>,
    #[get(pub)]
    #[set(pub)]
    pub(super) active_users: Vec<i64>,
    #[get(pub)]
    #[set(pub)]
    pub(super) new_records: Vec<i64>,
}

#[derive(Clone, Data, Debug, Default, Deserialize, Serialize)]
pub struct OverviewStatisticsResponse {
    #[get(pub)]
    #[set(pub)]
    pub(super) today: TodayStatistics,
    #[get(pub)]
    #[set(pub)]
    pub(super) changes: ChangesStatistics,
    #[get(pub)]
    #[set(pub)]
    pub(super) daily_trend: DailyTrend,
    #[get(pub)]
    #[set(pub)]
    pub(super) monthly_comparison: MonthlyComparison,
    #[get(pub)]
    #[set(pub)]
    pub(super) category_distribution: Vec<CategoryItem>,
    #[get(pub)]
    #[set(pub)]
    pub(super) user_growth: UserGrowth,
    #[get(pub)]
    #[set(pub)]
    pub(super) transaction_type_distribution: TransactionTypeDistribution,
    #[get(pub)]
    #[set(pub)]
    pub(super) transaction_count_trend: TransactionCountTrend,
    #[get(pub)]
    #[set(pub)]
    pub(super) category_amount_distribution: Vec<CategoryAmountItem>,
    #[get(pub)]
    #[set(pub)]
    pub(super) user_activity: UserActivity,
}
