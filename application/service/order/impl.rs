use super::*;

impl PasswordUtil {
    #[instrument_trace]
    pub fn hash_password(password: &str) -> String {
        use hyperlane_utils::sha2::{Digest, Sha256};
        let mut hasher: Sha256 = Sha256::new();
        hasher.update(password.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    #[instrument_trace]
    pub fn verify_password(password: &str, hash: &str) -> bool {
        Self::hash_password(password) == hash
    }
}

impl OrderService {
    #[instrument_trace]
    pub fn extract_user_from_cookie(ctx: &Context) -> Result<i32, String> {
        let token: String = match ctx.get_request().try_get_cookie("token") {
            Some(cookie) => cookie,
            None => return Err("Authentication token not found".to_string()),
        };
        let jwt_config: JwtConfig = JwtConfig::new(
            JwtConfigEnum::SecretKey.to_string(),
            JwtConfigEnum::Expiration.expiration_as_u64(),
            JwtConfigEnum::Issuer.to_string(),
        );
        let jwt_service: JwtService = JwtService::from(jwt_config);
        let user_id_value: serde_json::Value = match jwt_service.get_from_token(&token, "user_id") {
            Ok(Some(value)) => value,
            Ok(None) => return Err("user_id not found in token".to_string()),
            Err(_) => return Err("Invalid token".to_string()),
        };
        let user_id: i32 = match user_id_value.as_i64() {
            Some(id) => id as i32,
            None => return Err("Invalid user_id format in token".to_string()),
        };
        Ok(user_id)
    }

    #[instrument_trace]
    fn generate_bill_no() -> String {
        let now: chrono::DateTime<Local> = Local::now();
        let timestamp: i64 = now.timestamp();
        let nanos: u32 = now.timestamp_subsec_nanos() % 10000;
        format!("BILL{}{:04}", timestamp, nanos)
    }

    #[instrument_trace]
    pub async fn register_user(request: RegisterRequest) -> Result<UserResponse, String> {
        let existing_user: Option<OrderUserModel> =
            UserRepository::find_by_username(request.get_username().clone()).await?;
        if existing_user.is_some() {
            return Err("Username already exists".to_string());
        }
        let password_hash: String = PasswordUtil::hash_password(request.get_password());
        let active_model: OrderUserActiveModel = OrderUserActiveModel {
            username: ActiveValue::Set(request.get_username().clone()),
            password_hash: ActiveValue::Set(password_hash),
            email: ActiveValue::Set(request.try_get_email().clone()),
            phone: ActiveValue::Set(request.try_get_phone().clone()),
            role: ActiveValue::Set(UserRole::User.to_i16()),
            status: ActiveValue::Set(UserStatus::Pending.to_i16()),
            id: ActiveValue::NotSet,
            created_at: ActiveValue::NotSet,
            updated_at: ActiveValue::NotSet,
        };
        let result: OrderUserModel = UserRepository::insert(active_model).await?;
        Ok(Self::model_to_user_response(&result))
    }

    #[instrument_trace]
    pub async fn login_user(request: LoginRequest) -> Result<(UserResponse, i32, i16), String> {
        let user: Option<OrderUserModel> =
            UserRepository::find_by_username(request.get_username().clone()).await?;
        match user {
            Some(model) => {
                if model.get_status() != UserStatus::Approved.to_i16() {
                    return Err("User is not approved".to_string());
                }
                let valid: bool = PasswordUtil::verify_password(
                    request.get_password(),
                    model.get_password_hash(),
                );
                if !valid {
                    return Err("Invalid password".to_string());
                }
                let user_response: UserResponse = Self::model_to_user_response(&model);
                let user_id: i32 = model.get_id();
                let role: i16 = model.get_role();
                Ok((user_response, user_id, role))
            }
            None => Err("User not found".to_string()),
        }
    }

    #[instrument_trace]
    pub async fn create_user(request: CreateUserRequest) -> Result<UserResponse, String> {
        let existing_user: Option<OrderUserModel> =
            UserRepository::find_by_username(request.get_username().clone()).await?;
        if existing_user.is_some() {
            return Err("Username already exists".to_string());
        }
        let password_hash: String = PasswordUtil::hash_password(request.get_password());
        let active_model: OrderUserActiveModel = OrderUserActiveModel {
            username: ActiveValue::Set(request.get_username().clone()),
            password_hash: ActiveValue::Set(password_hash),
            email: ActiveValue::Set(request.try_get_email().clone()),
            phone: ActiveValue::Set(request.try_get_phone().clone()),
            role: ActiveValue::Set(*request.get_role()),
            status: ActiveValue::Set(UserStatus::Approved.to_i16()),
            id: ActiveValue::NotSet,
            created_at: ActiveValue::NotSet,
            updated_at: ActiveValue::NotSet,
        };
        let result: OrderUserModel = UserRepository::insert(active_model).await?;
        Ok(Self::model_to_user_response(&result))
    }

    #[instrument_trace]
    pub async fn update_user(
        user_id: i32,
        request: UpdateUserRequest,
    ) -> Result<UserResponse, String> {
        match UserRepository::find_by_id(user_id).await? {
            Some(model) => {
                let mut active_model: OrderUserActiveModel = model.into();
                if let Some(email) = request.try_get_email() {
                    active_model.email = ActiveValue::Set(Some(email.clone()));
                }
                if let Some(phone) = request.try_get_phone() {
                    active_model.phone = ActiveValue::Set(Some(phone.clone()));
                }
                active_model.updated_at = ActiveValue::Set(Some(Local::now().naive_local()));
                let result: OrderUserModel = UserRepository::update(active_model).await?;
                Ok(Self::model_to_user_response(&result))
            }
            None => Err("User not found".to_string()),
        }
    }

    #[instrument_trace]
    pub async fn change_password(
        user_id: i32,
        request: ChangePasswordRequest,
    ) -> Result<(), String> {
        match UserRepository::find_by_id(user_id).await? {
            Some(model) => {
                let valid: bool = PasswordUtil::verify_password(
                    request.get_old_password(),
                    model.get_password_hash(),
                );
                if !valid {
                    return Err("Old password is incorrect".to_string());
                }
                let new_password_hash: String =
                    PasswordUtil::hash_password(request.get_new_password());
                let mut active_model: OrderUserActiveModel = model.into();
                active_model.password_hash = ActiveValue::Set(new_password_hash);
                active_model.updated_at = ActiveValue::Set(Some(Local::now().naive_local()));
                UserRepository::update(active_model).await?;
                Ok(())
            }
            None => Err("User not found".to_string()),
        }
    }

    #[instrument_trace]
    pub async fn approve_user(user_id: i32, approved: bool) -> Result<UserResponse, String> {
        match UserRepository::find_by_id(user_id).await? {
            Some(model) => {
                let mut active_model: OrderUserActiveModel = model.into();
                let status: i16 = if approved {
                    UserStatus::Approved.to_i16()
                } else {
                    UserStatus::Rejected.to_i16()
                };
                active_model.status = ActiveValue::Set(status);
                active_model.updated_at = ActiveValue::Set(Some(Local::now().naive_local()));
                let result: OrderUserModel = UserRepository::update(active_model).await?;
                Ok(Self::model_to_user_response(&result))
            }
            None => Err("User not found".to_string()),
        }
    }

    #[instrument_trace]
    pub async fn list_users(query: UserListQueryRequest) -> Result<UserListResponse, String> {
        let limit: u64 = query.get_limit().unwrap_or(20);
        let keyword: Option<String> = query.try_get_keyword().clone();
        let last_id: Option<i32> = query.get_last_id();
        let (paged_users, total_count, has_more) =
            UserRepository::query_with_pagination(keyword, last_id, limit).await?;
        let last_id: Option<i32> = paged_users.last().map(|u: &OrderUserModel| u.get_id());
        let user_responses: Vec<UserResponse> = paged_users
            .iter()
            .map(Self::model_to_user_response)
            .collect();
        let mut response: UserListResponse = UserListResponse::default();
        response
            .set_users(user_responses)
            .set_has_more(has_more)
            .set_last_id(last_id)
            .set_total_count(total_count);
        Ok(response)
    }

    #[instrument_trace]
    pub async fn get_user(user_id: i32) -> Result<Option<UserResponse>, String> {
        Ok(UserRepository::find_by_id(user_id)
            .await?
            .map(|model: OrderUserModel| Self::model_to_user_response(&model)))
    }

    #[instrument_trace]
    fn model_to_user_response(model: &OrderUserModel) -> UserResponse {
        let mut response: UserResponse = UserResponse::default();
        let created_at: Option<String> = model
            .try_get_created_at()
            .as_ref()
            .map(|dt: &NaiveDateTime| dt.format("%Y-%m-%d %H:%M:%S").to_string());
        let role: UserRole = UserRole::try_from(model.get_role()).unwrap_or_default();
        let status: UserStatus = UserStatus::try_from(model.get_status()).unwrap_or_default();
        response
            .set_id(model.get_id())
            .set_username(model.get_username().clone())
            .set_email(model.try_get_email().clone())
            .set_phone(model.try_get_phone().clone())
            .set_role(role.as_str().to_string())
            .set_status(status.as_str().to_string())
            .set_created_at(created_at);
        response
    }

    #[instrument_trace]
    pub async fn create_record(
        user_id: i32,
        request: CreateRecordRequest,
    ) -> Result<RecordResponse, String> {
        let bill_no: String = Self::generate_bill_no();
        let bill_date: NaiveDate = request
            .try_get_bill_date()
            .unwrap_or_else(|| Local::now().naive_local().date());
        let active_model: OrderRecordActiveModel = OrderRecordActiveModel {
            bill_no: ActiveValue::Set(bill_no.clone()),
            user_id: ActiveValue::Set(user_id),
            amount: ActiveValue::Set(request.get_amount()),
            category: ActiveValue::Set(request.get_category().clone()),
            transaction_type: ActiveValue::Set(request.get_transaction_type().clone()),
            description: ActiveValue::Set(request.try_get_description().clone()),
            bill_date: ActiveValue::Set(bill_date),
            id: ActiveValue::NotSet,
            created_at: ActiveValue::NotSet,
            updated_at: ActiveValue::NotSet,
        };
        let result: OrderRecordModel = RecordRepository::insert(active_model).await?;
        Ok(Self::model_to_record_response(&result))
    }

    #[instrument_trace]
    pub async fn list_records(query: RecordQueryRequest) -> Result<RecordListResponse, String> {
        let page: i32 = query.get_page().unwrap_or(1);
        let limit: u64 = query.get_limit().unwrap_or(20);
        let user_id: Option<i32> = query.get_user_id();
        let start_date: Option<NaiveDate> = *query.try_get_start_date();
        let end_date: Option<NaiveDate> = *query.try_get_end_date();
        let category: Option<String> = query.try_get_category().clone();
        let transaction_type: Option<String> = query.try_get_transaction_type().clone();
        let cache_id: Option<i32> = query.get_cache_id();
        let mut pagination_query: RecordPaginationQuery = RecordPaginationQuery::default();
        pagination_query
            .set_user_id(user_id)
            .set_start_date(start_date)
            .set_end_date(end_date)
            .set_category(category.clone())
            .set_transaction_type(transaction_type.clone())
            .set_cache_id(cache_id)
            .set_page(page)
            .set_limit(limit);
        let (paged_records, total_count) =
            RecordRepository::query_with_pagination(pagination_query).await?;
        let total_income: Decimal = RecordRepository::sum_amount_by_transaction_type(
            user_id,
            start_date,
            end_date,
            category.clone(),
            cache_id,
            TransactionType::Income.as_str().to_string(),
        )
        .await?;
        let total_expense: Decimal = RecordRepository::sum_amount_by_transaction_type(
            user_id,
            start_date,
            end_date,
            category,
            cache_id,
            TransactionType::Expense.as_str().to_string(),
        )
        .await?;
        let balance: Decimal = total_income - total_expense;
        let record_responses: Vec<RecordResponse> =
            Self::enrich_records_with_users(paged_records).await?;
        let mut response: RecordListResponse = RecordListResponse::default();
        response
            .set_records(record_responses)
            .set_total_income(total_income.to_string())
            .set_total_expense(total_expense.to_string())
            .set_balance(balance.to_string())
            .set_total_count(total_count);
        Ok(response)
    }

    #[instrument_trace]
    pub async fn get_record(record_id: i32) -> Result<Option<RecordResponse>, String> {
        match RecordRepository::find_by_id(record_id).await? {
            Some(model) => {
                let mut response: RecordResponse = Self::model_to_record_response(&model);
                Self::enrich_record_with_user(&mut response).await?;
                Ok(Some(response))
            }
            None => Ok(None),
        }
    }

    #[instrument_trace]
    async fn enrich_records_with_users(
        records: Vec<OrderRecordModel>,
    ) -> Result<Vec<RecordResponse>, String> {
        let user_ids: Vec<i32> = records
            .iter()
            .map(|r: &OrderRecordModel| r.get_user_id())
            .collect::<HashSet<i32>>()
            .into_iter()
            .collect();
        let users: Vec<OrderUserModel> = UserRepository::find_by_ids(user_ids).await?;
        let user_map: HashMap<i32, OrderUserModel> = users
            .into_iter()
            .map(|u: OrderUserModel| (u.get_id(), u))
            .collect();
        let responses: Vec<RecordResponse> = records
            .iter()
            .map(|record: &OrderRecordModel| {
                let mut response: RecordResponse = Self::model_to_record_response(record);
                if let Some(user) = user_map.get(&response.get_user_id()) {
                    response
                        .set_username(Some(user.get_username().clone()))
                        .set_email(user.try_get_email().clone())
                        .set_phone(user.try_get_phone().clone());
                }
                response
            })
            .collect();
        Ok(responses)
    }

    #[instrument_trace]
    async fn enrich_record_with_user(response: &mut RecordResponse) -> Result<(), String> {
        let user: Option<OrderUserModel> =
            UserRepository::find_by_id(response.get_user_id()).await?;
        if let Some(user) = user {
            response
                .set_username(Some(user.get_username().clone()))
                .set_email(user.try_get_email().clone())
                .set_phone(user.try_get_phone().clone());
        }
        Ok(())
    }

    #[instrument_trace]
    fn model_to_record_response(model: &OrderRecordModel) -> RecordResponse {
        let mut response: RecordResponse = RecordResponse::default();
        let created_at: Option<String> = model
            .try_get_created_at()
            .as_ref()
            .map(|dt: &NaiveDateTime| dt.format("%Y-%m-%d %H:%M:%S").to_string());
        response
            .set_id(model.get_id())
            .set_bill_no(model.get_bill_no().clone())
            .set_user_id(model.get_user_id())
            .set_amount(model.get_amount().to_string())
            .set_category(model.get_category().clone())
            .set_transaction_type(model.get_transaction_type().clone())
            .set_description(model.try_get_description().clone())
            .set_bill_date(model.get_bill_date().to_string())
            .set_created_at(created_at);
        response
    }

    #[instrument_trace]
    pub async fn get_overview_statistics() -> Result<OverviewStatisticsResponse, String> {
        let today: NaiveDate = Local::now().naive_local().date();
        let yesterday: NaiveDate = today - chrono::Duration::days(1);
        let today_stats: (i64, Decimal, Decimal) = Self::get_date_statistics(today).await?;
        let yesterday_stats: (i64, Decimal, Decimal) = Self::get_date_statistics(yesterday).await?;
        let daily_trend: DailyTrend = Self::get_daily_trend(30).await?;
        let monthly_comparison: MonthlyComparison = Self::get_monthly_comparison(6).await?;
        let category_distribution: Vec<CategoryItem> = Self::get_category_distribution().await?;
        let user_growth: UserGrowth = Self::get_user_growth(30).await?;
        let transaction_type_distribution: TransactionTypeDistribution =
            Self::get_transaction_type_distribution().await?;
        let transaction_count_trend: TransactionCountTrend =
            Self::get_transaction_count_trend(30).await?;
        let category_amount_distribution: Vec<CategoryAmountItem> =
            Self::get_category_amount_distribution().await?;
        let user_activity: UserActivity = Self::get_user_activity(30).await?;
        let income_expense_ratio_trend: Vec<IncomeExpenseRatioItem> =
            Self::get_income_expense_ratio_trend(30).await?;
        let hourly_distribution: Vec<HourlyDistributionItem> =
            Self::get_hourly_distribution().await?;
        let weekly_trend: Vec<WeeklyTrendItem> = Self::get_weekly_trend().await?;
        let period_over_period: Vec<PeriodOverPeriodItem> =
            Self::get_period_over_period_analysis().await?;
        let category_trends: Vec<CategoryTrendItem> = Self::get_category_trends(30).await?;
        let user_retention: Vec<UserRetentionItem> = Self::get_user_retention(30).await?;
        let top_users: Vec<TopUserItem> = Self::get_top_users(10).await?;
        let avg_transaction_stats: AverageTransactionStats =
            Self::get_average_transaction_stats().await?;
        let transactions_change: Option<f64> =
            Self::calculate_change_percentage(today_stats.0 as f64, yesterday_stats.0 as f64);
        let income_change: Option<f64> = Self::calculate_change_percentage(
            today_stats.1.to_f64().unwrap_or(0.0),
            yesterday_stats.1.to_f64().unwrap_or(0.0),
        );
        let expense_change: Option<f64> = Self::calculate_change_percentage(
            today_stats.2.to_f64().unwrap_or(0.0),
            yesterday_stats.2.to_f64().unwrap_or(0.0),
        );
        let today_new_users: i64 = Self::get_new_users_count(today).await?;
        let yesterday_new_users: i64 = Self::get_new_users_count(yesterday).await?;
        let new_users_change: Option<f64> =
            Self::calculate_change_percentage(today_new_users as f64, yesterday_new_users as f64);
        let (today_avg_income, today_avg_expense): (Decimal, Decimal) =
            Self::get_date_avg_stats(today).await?;
        let (yesterday_avg_income, yesterday_avg_expense): (Decimal, Decimal) =
            Self::get_date_avg_stats(yesterday).await?;
        let avg_income_change: Option<f64> = Self::calculate_change_percentage(
            today_avg_income.to_f64().unwrap_or(0.0),
            yesterday_avg_income.to_f64().unwrap_or(0.0),
        );
        let avg_expense_change: Option<f64> = Self::calculate_change_percentage(
            today_avg_expense.to_f64().unwrap_or(0.0),
            yesterday_avg_expense.to_f64().unwrap_or(0.0),
        );
        let mut today_statistics: TodayStatistics = TodayStatistics::default();
        today_statistics
            .set_transactions(today_stats.0)
            .set_income(today_stats.1.to_string())
            .set_expense(today_stats.2.to_string())
            .set_new_users(today_new_users);
        let mut changes_statistics: ChangesStatistics = ChangesStatistics::default();
        changes_statistics
            .set_transactions_change(transactions_change)
            .set_income_change(income_change)
            .set_expense_change(expense_change)
            .set_new_users_change(new_users_change)
            .set_avg_income_change(avg_income_change)
            .set_avg_expense_change(avg_expense_change);
        let mut response: OverviewStatisticsResponse = OverviewStatisticsResponse::default();
        response
            .set_today(today_statistics)
            .set_changes(changes_statistics)
            .set_daily_trend(daily_trend)
            .set_monthly_comparison(monthly_comparison)
            .set_category_distribution(category_distribution)
            .set_user_growth(user_growth)
            .set_transaction_type_distribution(transaction_type_distribution)
            .set_transaction_count_trend(transaction_count_trend)
            .set_category_amount_distribution(category_amount_distribution)
            .set_user_activity(user_activity)
            .set_income_expense_ratio_trend(income_expense_ratio_trend)
            .set_hourly_distribution(hourly_distribution)
            .set_weekly_trend(weekly_trend)
            .set_period_over_period(period_over_period)
            .set_category_trends(category_trends)
            .set_user_retention(user_retention)
            .set_top_users(top_users)
            .set_avg_transaction_stats(avg_transaction_stats);
        Ok(response)
    }

    #[instrument_trace]
    async fn get_date_statistics(date: NaiveDate) -> Result<(i64, Decimal, Decimal), String> {
        let records: Vec<OrderRecordModel> = RecordRepository::find_by_bill_date(date).await?;
        let transaction_count: i64 = records.len() as i64;
        let mut total_income: Decimal = Decimal::ZERO;
        let mut total_expense: Decimal = Decimal::ZERO;
        for record in &records {
            let amount: Decimal = *record.get_amount();
            if record.get_transaction_type() == TransactionType::Income.as_str() {
                total_income += amount;
            } else {
                total_expense += amount;
            }
        }
        Ok((transaction_count, total_income, total_expense))
    }

    #[instrument_trace]
    async fn get_new_users_count(date: NaiveDate) -> Result<i64, String> {
        let start_of_day: NaiveDateTime = date.and_hms_opt(0, 0, 0).unwrap();
        let end_of_day: NaiveDateTime = date.and_hms_opt(23, 59, 59).unwrap();
        UserRepository::count_by_created_at_range(start_of_day, end_of_day).await
    }

    #[instrument_trace]
    async fn get_daily_trend(days: i64) -> Result<DailyTrend, String> {
        let end_date: NaiveDate = Local::now().naive_local().date();
        let start_date: NaiveDate = end_date - chrono::Duration::days(days);
        let records: Vec<OrderRecordModel> =
            RecordRepository::find_by_bill_date_range(start_date, end_date).await?;
        let mut dates: Vec<String> = Vec::new();
        let mut income: Vec<String> = Vec::new();
        let mut expense: Vec<String> = Vec::new();
        let mut current_date: NaiveDate = start_date;
        while current_date <= end_date {
            dates.push(current_date.to_string());
            let day_records: Vec<&OrderRecordModel> = records
                .iter()
                .filter(|r| *r.get_bill_date() == current_date)
                .collect();
            let mut day_income: Decimal = Decimal::ZERO;
            let mut day_expense: Decimal = Decimal::ZERO;
            for record in day_records {
                let amount: Decimal = *record.get_amount();
                if record.get_transaction_type() == TransactionType::Income.as_str() {
                    day_income += amount;
                } else {
                    day_expense += amount;
                }
            }
            income.push(day_income.to_string());
            expense.push(day_expense.to_string());
            current_date += chrono::Duration::days(1);
        }
        let mut trend: DailyTrend = DailyTrend::default();
        trend.set_dates(dates);
        trend.set_income(income);
        trend.set_expense(expense);
        Ok(trend)
    }

    #[instrument_trace]
    async fn get_monthly_comparison(months: i64) -> Result<MonthlyComparison, String> {
        let now: chrono::DateTime<Local> = Local::now();
        let mut months_list: Vec<String> = Vec::new();
        let mut income_list: Vec<String> = Vec::new();
        let mut expense_list: Vec<String> = Vec::new();
        for i in (0..months).rev() {
            let target_date: chrono::DateTime<Local> = now - chrono::Duration::days(i * 30);
            let year: i32 = target_date.year();
            let month: u32 = target_date.month();
            let month_key: String = format!("{year}-{month:02}");
            months_list.push(month_key.clone());
            let start_date: NaiveDate = NaiveDate::from_ymd_opt(year, month, 1).unwrap_or_default();
            let end_date: NaiveDate = if month == 12 {
                NaiveDate::from_ymd_opt(year + 1, 1, 1).unwrap_or_default()
                    - chrono::Duration::days(1)
            } else {
                NaiveDate::from_ymd_opt(year, month + 1, 1).unwrap_or_default()
                    - chrono::Duration::days(1)
            };
            let records: Vec<OrderRecordModel> =
                RecordRepository::find_by_bill_date_range(start_date, end_date).await?;
            let mut total_income: Decimal = Decimal::ZERO;
            let mut total_expense: Decimal = Decimal::ZERO;
            for record in &records {
                let amount: Decimal = *record.get_amount();
                if record.get_transaction_type() == TransactionType::Income.as_str() {
                    total_income += amount;
                } else {
                    total_expense += amount;
                }
            }
            income_list.push(total_income.to_string());
            expense_list.push(total_expense.to_string());
        }
        let mut comparison: MonthlyComparison = MonthlyComparison::default();
        comparison.set_months(months_list);
        comparison.set_income(income_list);
        comparison.set_expense(expense_list);
        Ok(comparison)
    }

    #[instrument_trace]
    async fn get_category_distribution() -> Result<Vec<CategoryItem>, String> {
        let records: Vec<OrderRecordModel> = RecordRepository::find_all().await?;
        let mut category_map: HashMap<String, i64> = HashMap::new();
        for record in &records {
            if record.get_transaction_type() == TransactionType::Expense.as_str() {
                let category: String = record.get_category().clone();
                *category_map.entry(category).or_insert(0) += 1;
            }
        }
        let mut result: Vec<CategoryItem> = category_map
            .into_iter()
            .map(|(name, value)| {
                let mut item: CategoryItem = CategoryItem::default();
                item.set_name(name);
                item.set_value(value);
                item
            })
            .collect();
        result.sort_by(|a, b| {
            let a_val: i64 = a.get_value();
            let b_val: i64 = b.get_value();
            b_val.cmp(&a_val)
        });
        Ok(result)
    }

    #[instrument_trace]
    async fn get_user_growth(days: i64) -> Result<UserGrowth, String> {
        let end_date: NaiveDate = Local::now().naive_local().date();
        let start_date: NaiveDate = end_date - chrono::Duration::days(days);
        let mut dates: Vec<String> = Vec::new();
        let mut counts: Vec<i64> = Vec::new();
        let mut current_date: NaiveDate = start_date;
        while current_date <= end_date {
            dates.push(current_date.to_string());
            let start_of_day: NaiveDateTime = current_date.and_hms_opt(0, 0, 0).unwrap();
            let end_of_day: NaiveDateTime = current_date.and_hms_opt(23, 59, 59).unwrap();
            let count: i64 =
                UserRepository::count_by_created_at_range(start_of_day, end_of_day).await?;
            counts.push(count);
            current_date += chrono::Duration::days(1);
        }
        let mut growth: UserGrowth = UserGrowth::default();
        growth.set_dates(dates);
        growth.set_counts(counts);
        Ok(growth)
    }

    #[instrument_trace]
    async fn get_transaction_type_distribution() -> Result<TransactionTypeDistribution, String> {
        let records: Vec<OrderRecordModel> = RecordRepository::find_all().await?;
        let mut income_count: i64 = 0;
        let mut expense_count: i64 = 0;
        let mut income_amount: Decimal = Decimal::ZERO;
        let mut expense_amount: Decimal = Decimal::ZERO;
        for record in &records {
            let amount: Decimal = *record.get_amount();
            if record.get_transaction_type() == TransactionType::Income.as_str() {
                income_count += 1;
                income_amount += amount;
            } else {
                expense_count += 1;
                expense_amount += amount;
            }
        }
        let mut distribution: TransactionTypeDistribution = TransactionTypeDistribution::default();
        distribution.set_income_count(income_count);
        distribution.set_expense_count(expense_count);
        distribution.set_income_amount(income_amount.to_string());
        distribution.set_expense_amount(expense_amount.to_string());
        Ok(distribution)
    }

    #[instrument_trace]
    async fn get_transaction_count_trend(days: i64) -> Result<TransactionCountTrend, String> {
        let end_date: NaiveDate = Local::now().naive_local().date();
        let start_date: NaiveDate = end_date - chrono::Duration::days(days);
        let records: Vec<OrderRecordModel> =
            RecordRepository::find_by_bill_date_range(start_date, end_date).await?;
        let mut dates: Vec<String> = Vec::new();
        let mut counts: Vec<i64> = Vec::new();
        let mut current_date: NaiveDate = start_date;
        while current_date <= end_date {
            dates.push(current_date.to_string());
            let day_count: i64 = records
                .iter()
                .filter(|r| *r.get_bill_date() == current_date)
                .count() as i64;
            counts.push(day_count);
            current_date += chrono::Duration::days(1);
        }
        let mut trend: TransactionCountTrend = TransactionCountTrend::default();
        trend.set_dates(dates);
        trend.set_counts(counts);
        Ok(trend)
    }

    #[instrument_trace]
    async fn get_category_amount_distribution() -> Result<Vec<CategoryAmountItem>, String> {
        let records: Vec<OrderRecordModel> = RecordRepository::find_all().await?;
        let mut category_map: HashMap<String, Decimal> = HashMap::new();
        for record in &records {
            if record.get_transaction_type() == TransactionType::Expense.as_str() {
                let category: String = record.get_category().clone();
                let amount: Decimal = *record.get_amount();
                *category_map.entry(category).or_insert(Decimal::ZERO) += amount;
            }
        }
        let mut result: Vec<CategoryAmountItem> = category_map
            .into_iter()
            .map(|(name, amount)| {
                let mut item: CategoryAmountItem = CategoryAmountItem::default();
                item.set_name(name);
                item.set_amount(amount.to_string());
                item
            })
            .collect();
        result.sort_by(|a, b| {
            let a_amt: Decimal = a.get_amount().parse().unwrap_or(Decimal::ZERO);
            let b_amt: Decimal = b.get_amount().parse().unwrap_or(Decimal::ZERO);
            b_amt.cmp(&a_amt)
        });
        Ok(result)
    }

    #[instrument_trace]
    async fn get_user_activity(days: i64) -> Result<UserActivity, String> {
        let end_date: NaiveDate = Local::now().naive_local().date();
        let start_date: NaiveDate = end_date - chrono::Duration::days(days);
        let mut dates: Vec<String> = Vec::new();
        let mut active_users: Vec<i64> = Vec::new();
        let mut new_records: Vec<i64> = Vec::new();
        let records: Vec<OrderRecordModel> =
            RecordRepository::find_by_bill_date_range(start_date, end_date).await?;
        let mut current_date: NaiveDate = start_date;
        while current_date <= end_date {
            dates.push(current_date.to_string());
            let day_records: Vec<&OrderRecordModel> = records
                .iter()
                .filter(|r| *r.get_bill_date() == current_date)
                .collect();
            let unique_users: HashSet<i32> = day_records.iter().map(|r| r.get_user_id()).collect();
            active_users.push(unique_users.len() as i64);
            new_records.push(day_records.len() as i64);
            current_date += chrono::Duration::days(1);
        }
        let mut activity: UserActivity = UserActivity::default();
        activity.set_dates(dates);
        activity.set_active_users(active_users);
        activity.set_new_records(new_records);
        Ok(activity)
    }
}

impl OrderService {
    #[instrument_trace]
    fn calculate_change_percentage(current: f64, previous: f64) -> Option<f64> {
        if previous == 0.0 {
            if current > 0.0 {
                return Some(100.0);
            }
            return None;
        }
        Some(((current - previous) / previous) * 100.0)
    }

    #[instrument_trace]
    async fn get_income_expense_ratio_trend(
        days: i64,
    ) -> Result<Vec<IncomeExpenseRatioItem>, String> {
        let end_date: NaiveDate = Local::now().naive_local().date();
        let start_date: NaiveDate = end_date - chrono::Duration::days(days);
        let records: Vec<OrderRecordModel> =
            RecordRepository::find_by_bill_date_range(start_date, end_date).await?;
        let mut result: Vec<IncomeExpenseRatioItem> = Vec::new();
        let mut current_date: NaiveDate = start_date;
        while current_date <= end_date {
            let day_records: Vec<&OrderRecordModel> = records
                .iter()
                .filter(|r| *r.get_bill_date() == current_date)
                .collect();
            let mut day_income: Decimal = Decimal::ZERO;
            let mut day_expense: Decimal = Decimal::ZERO;
            for record in &day_records {
                let amount: Decimal = *record.get_amount();
                if record.get_transaction_type() == TransactionType::Income.as_str() {
                    day_income += amount;
                } else {
                    day_expense += amount;
                }
            }
            let ratio: f64 = if day_expense > Decimal::ZERO {
                (day_income / day_expense).to_f64().unwrap_or(0.0)
            } else {
                0.0
            };
            let mut item: IncomeExpenseRatioItem = IncomeExpenseRatioItem::default();
            item.set_date(current_date.to_string());
            item.set_ratio(ratio);
            item.set_income(day_income.to_string());
            item.set_expense(day_expense.to_string());
            result.push(item);
            current_date += chrono::Duration::days(1);
        }
        Ok(result)
    }

    #[instrument_trace]
    async fn get_hourly_distribution() -> Result<Vec<HourlyDistributionItem>, String> {
        let today: NaiveDate = Local::now().naive_local().date();
        let start_date: NaiveDate = today - chrono::Duration::days(30);
        let records: Vec<OrderRecordModel> =
            RecordRepository::find_by_bill_date_range(start_date, today).await?;
        let mut hourly_data: HashMap<i32, (i64, Decimal, Decimal)> = HashMap::new();
        for record in &records {
            let hour: i32 = record
                .try_get_created_at()
                .map(|dt| dt.hour() as i32)
                .unwrap_or(0);
            let entry: &mut (i64, Decimal, Decimal) =
                hourly_data
                    .entry(hour)
                    .or_insert((0, Decimal::ZERO, Decimal::ZERO));
            entry.0 += 1;
            let amount: Decimal = *record.get_amount();
            if record.get_transaction_type() == TransactionType::Income.as_str() {
                entry.1 += amount;
            } else {
                entry.2 += amount;
            }
        }
        let result: Vec<HourlyDistributionItem> = (0..24)
            .map(|hour| {
                let (count, income, expense) =
                    hourly_data
                        .get(&hour)
                        .copied()
                        .unwrap_or((0, Decimal::ZERO, Decimal::ZERO));
                let mut item: HourlyDistributionItem = HourlyDistributionItem::default();
                item.set_hour(hour);
                item.set_count(count);
                item.set_income(income.to_string());
                item.set_expense(expense.to_string());
                item
            })
            .collect();
        Ok(result)
    }

    #[instrument_trace]
    async fn get_weekly_trend() -> Result<Vec<WeeklyTrendItem>, String> {
        let today: NaiveDate = Local::now().naive_local().date();
        let start_date: NaiveDate = today - chrono::Duration::days(90);
        let records: Vec<OrderRecordModel> =
            RecordRepository::find_by_bill_date_range(start_date, today).await?;
        let days: Vec<&str> = WEEK_DAYS.to_vec();
        let mut weekly_data: Vec<(Decimal, Decimal, i64)> =
            vec![(Decimal::ZERO, Decimal::ZERO, 0); 7];
        for record in &records {
            let weekday: usize = record.get_bill_date().weekday().num_days_from_monday() as usize;
            let amount: Decimal = *record.get_amount();
            if record.get_transaction_type() == TransactionType::Income.as_str() {
                weekly_data[weekday].0 += amount;
            } else {
                weekly_data[weekday].1 += amount;
            }
            weekly_data[weekday].2 += 1;
        }
        let result: Vec<WeeklyTrendItem> = days
            .into_iter()
            .enumerate()
            .map(|(idx, day)| {
                let (income, expense, count) = weekly_data[idx];
                let mut item: WeeklyTrendItem = WeeklyTrendItem::default();
                item.set_day_of_week(day.to_string());
                item.set_income(income.to_string());
                item.set_expense(expense.to_string());
                item.set_transaction_count(count);
                item
            })
            .collect();
        Ok(result)
    }

    #[instrument_trace]
    async fn get_period_over_period_analysis() -> Result<Vec<PeriodOverPeriodItem>, String> {
        let today: NaiveDate = Local::now().naive_local().date();
        let mut result: Vec<PeriodOverPeriodItem> = Vec::new();
        let periods: Vec<(String, NaiveDate, NaiveDate, NaiveDate, NaiveDate)> = vec![
            (
                "WoW".to_string(),
                today - chrono::Duration::days(6),
                today,
                today - chrono::Duration::days(13),
                today - chrono::Duration::days(7),
            ),
            (
                "MoM".to_string(),
                today - chrono::Duration::days(29),
                today,
                today - chrono::Duration::days(59),
                today - chrono::Duration::days(30),
            ),
            (
                "QoQ".to_string(),
                today - chrono::Duration::days(89),
                today,
                today - chrono::Duration::days(179),
                today - chrono::Duration::days(90),
            ),
        ];
        for (period_name, curr_start, curr_end, prev_start, prev_end) in periods {
            let curr_stats: (Decimal, Decimal, i64) =
                Self::get_period_stats(curr_start, curr_end).await?;
            let prev_stats: (Decimal, Decimal, i64) =
                Self::get_period_stats(prev_start, prev_end).await?;
            let income_change: f64 = Self::calculate_change_percentage(
                curr_stats.0.to_f64().unwrap_or(0.0),
                prev_stats.0.to_f64().unwrap_or(0.0),
            )
            .unwrap_or(0.0);
            let expense_change: f64 = Self::calculate_change_percentage(
                curr_stats.1.to_f64().unwrap_or(0.0),
                prev_stats.1.to_f64().unwrap_or(0.0),
            )
            .unwrap_or(0.0);
            let transaction_change: f64 =
                Self::calculate_change_percentage(curr_stats.2 as f64, prev_stats.2 as f64)
                    .unwrap_or(0.0);
            let mut item: PeriodOverPeriodItem = PeriodOverPeriodItem::default();
            item.set_period(period_name);
            item.set_income_change(income_change);
            item.set_expense_change(expense_change);
            item.set_transaction_change(transaction_change);
            result.push(item);
        }
        Ok(result)
    }

    #[instrument_trace]
    async fn get_period_stats(
        start: NaiveDate,
        end: NaiveDate,
    ) -> Result<(Decimal, Decimal, i64), String> {
        let records: Vec<OrderRecordModel> =
            RecordRepository::find_by_bill_date_range(start, end).await?;
        let mut income: Decimal = Decimal::ZERO;
        let mut expense: Decimal = Decimal::ZERO;
        for record in &records {
            let amount: Decimal = *record.get_amount();
            if record.get_transaction_type() == TransactionType::Income.as_str() {
                income += amount;
            } else {
                expense += amount;
            }
        }
        Ok((income, expense, records.len() as i64))
    }

    #[instrument_trace]
    async fn get_category_trends(days: i64) -> Result<Vec<CategoryTrendItem>, String> {
        let end_date: NaiveDate = Local::now().naive_local().date();
        let start_date: NaiveDate = end_date - chrono::Duration::days(days);
        let records: Vec<OrderRecordModel> =
            RecordRepository::find_by_bill_date_range_and_transaction_type(
                start_date,
                end_date,
                TransactionType::Expense.as_str().to_string(),
            )
            .await?;
        let mut category_data: HashMap<String, Vec<(String, Decimal)>> = HashMap::new();
        for record in &records {
            let category: String = record.get_category().clone();
            let date: String = record.get_bill_date().to_string();
            let amount: Decimal = *record.get_amount();
            category_data
                .entry(category)
                .or_default()
                .push((date, amount));
        }
        let dates: Vec<String> = (0..=days)
            .map(|i| (start_date + chrono::Duration::days(i)).to_string())
            .collect();
        let result: Vec<CategoryTrendItem> = category_data
            .into_iter()
            .map(|(category, data)| {
                let amounts: Vec<String> = dates
                    .iter()
                    .map(|date| {
                        let total: Decimal = data
                            .iter()
                            .filter(|(d, _)| d == date)
                            .map(|(_, amt)| amt)
                            .sum();
                        total.to_string()
                    })
                    .collect();
                let mut item: CategoryTrendItem = CategoryTrendItem::default();
                item.set_category(category);
                item.set_dates(dates.clone());
                item.set_amounts(amounts);
                item
            })
            .collect();
        Ok(result)
    }

    #[instrument_trace]
    async fn get_user_retention(days: i64) -> Result<Vec<UserRetentionItem>, String> {
        let end_date: NaiveDate = Local::now().naive_local().date();
        let start_date: NaiveDate = end_date - chrono::Duration::days(days);
        let users: Vec<OrderUserModel> = UserRepository::find_by_created_at_range(
            start_date.and_hms_opt(0, 0, 0).unwrap(),
            end_date.and_hms_opt(23, 59, 59).unwrap(),
        )
        .await?;
        let mut result: Vec<UserRetentionItem> = Vec::new();
        let records: Vec<OrderRecordModel> =
            RecordRepository::find_by_bill_date_range(start_date, end_date).await?;
        for day_offset in 0..=days {
            let date: NaiveDate = start_date + chrono::Duration::days(day_offset);
            let new_users: Vec<i32> = users
                .iter()
                .filter(|u| u.get_created_at().date() == date)
                .map(|u| u.get_id())
                .collect();
            let new_users_count: i64 = new_users.len() as i64;
            let retained: i64 = if new_users_count > 0 {
                let next_day: NaiveDate = date + chrono::Duration::days(1);
                let retained_users: HashSet<i32> = records
                    .iter()
                    .filter(|r| {
                        *r.get_bill_date() == next_day && new_users.contains(&r.get_user_id())
                    })
                    .map(|r| r.get_user_id())
                    .collect();
                retained_users.len() as i64
            } else {
                0
            };
            let retention_rate: f64 = if new_users_count > 0 {
                (retained as f64 / new_users_count as f64) * 100.0
            } else {
                0.0
            };
            let mut item: UserRetentionItem = UserRetentionItem::default();
            item.set_date(date.to_string());
            item.set_new_users(new_users_count);
            item.set_retained_users(retained);
            item.set_retention_rate(retention_rate);
            result.push(item);
        }
        Ok(result)
    }

    #[instrument_trace]
    async fn get_top_users(limit: i64) -> Result<Vec<TopUserItem>, String> {
        let today: NaiveDate = Local::now().naive_local().date();
        let start_date: NaiveDate = today - chrono::Duration::days(30);
        let records: Vec<OrderRecordModel> =
            RecordRepository::find_by_bill_date_range(start_date, today).await?;
        let mut user_stats: HashMap<i32, (String, i64, Decimal)> = HashMap::new();
        for record in &records {
            let user_id: i32 = record.get_user_id();
            let entry: &mut (String, i64, Decimal) = user_stats
                .entry(user_id)
                .or_insert_with(|| (String::new(), 0, Decimal::ZERO));
            entry.1 += 1;
            entry.2 += record.get_amount();
        }
        for (user_id, entry) in &mut user_stats {
            if let Ok(Some(user)) = UserRepository::find_by_id(*user_id).await {
                entry.0 = user.get_username().clone();
            }
        }
        let mut result: Vec<TopUserItem> = user_stats
            .into_iter()
            .map(|(user_id, (username, count, total))| {
                let mut item: TopUserItem = TopUserItem::default();
                item.set_user_id(user_id);
                item.set_username(username);
                item.set_transaction_count(count);
                item.set_total_amount(total.to_string());
                item
            })
            .collect();
        result.sort_by(|a, b| {
            let a_amt: Decimal = a.get_total_amount().parse().unwrap_or(Decimal::ZERO);
            let b_amt: Decimal = b.get_total_amount().parse().unwrap_or(Decimal::ZERO);
            b_amt.cmp(&a_amt)
        });
        result.truncate(limit as usize);
        Ok(result)
    }

    #[instrument_trace]
    async fn get_average_transaction_stats() -> Result<AverageTransactionStats, String> {
        let today: NaiveDate = Local::now().naive_local().date();
        let start_date: NaiveDate = today - chrono::Duration::days(30);
        let records: Vec<OrderRecordModel> =
            RecordRepository::find_by_bill_date_range(start_date, today).await?;
        let mut income_count: i64 = 0;
        let mut expense_count: i64 = 0;
        let mut total_income: Decimal = Decimal::ZERO;
        let mut total_expense: Decimal = Decimal::ZERO;
        let mut max_income: Decimal = Decimal::ZERO;
        let mut max_expense: Decimal = Decimal::ZERO;
        for record in &records {
            let amount: Decimal = *record.get_amount();
            if record.get_transaction_type() == TransactionType::Income.as_str() {
                income_count += 1;
                total_income += amount;
                if amount > max_income {
                    max_income = amount;
                }
            } else {
                expense_count += 1;
                total_expense += amount;
                if amount > max_expense {
                    max_expense = amount;
                }
            }
        }
        let avg_income: Decimal = if income_count > 0 {
            total_income / Decimal::from(income_count)
        } else {
            Decimal::ZERO
        };
        let avg_expense: Decimal = if expense_count > 0 {
            total_expense / Decimal::from(expense_count)
        } else {
            Decimal::ZERO
        };
        let overall_avg: Decimal = if !records.is_empty() {
            (total_income + total_expense) / Decimal::from(records.len() as i64)
        } else {
            Decimal::ZERO
        };
        let mut stats: AverageTransactionStats = AverageTransactionStats::default();
        stats.set_avg_income_per_transaction(avg_income.to_string());
        stats.set_avg_expense_per_transaction(avg_expense.to_string());
        stats.set_overall_avg_amount(overall_avg.to_string());
        stats.set_max_single_income(max_income.to_string());
        stats.set_max_single_expense(max_expense.to_string());
        Ok(stats)
    }

    #[instrument_trace]
    async fn get_date_avg_stats(date: NaiveDate) -> Result<(Decimal, Decimal), String> {
        let records: Vec<OrderRecordModel> = RecordRepository::find_by_bill_date(date).await?;
        let mut income_count: i64 = 0;
        let mut expense_count: i64 = 0;
        let mut total_income: Decimal = Decimal::ZERO;
        let mut total_expense: Decimal = Decimal::ZERO;
        for record in &records {
            let amount: Decimal = *record.get_amount();
            if record.get_transaction_type() == TransactionType::Income.as_str() {
                income_count += 1;
                total_income += amount;
            } else {
                expense_count += 1;
                total_expense += amount;
            }
        }
        let avg_income: Decimal = if income_count > 0 {
            total_income / Decimal::from(income_count)
        } else {
            Decimal::ZERO
        };
        let avg_expense: Decimal = if expense_count > 0 {
            total_expense / Decimal::from(expense_count)
        } else {
            Decimal::ZERO
        };
        Ok((avg_income, avg_expense))
    }
}

impl OrderService {
    #[instrument_trace]
    pub async fn create_record_with_images(
        user_id: i32,
        request: CreateRecordWithImagesRequest,
    ) -> Result<CreateRecordWithImagesResponse, String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let bill_no: String = Self::generate_bill_no();
        let bill_date: NaiveDate = request
            .try_get_bill_date()
            .unwrap_or_else(|| Local::now().naive_local().date());
        let txn: DatabaseTransaction = db.begin().await.map_err(|e: DbErr| e.to_string())?;
        let record_active_model: OrderRecordActiveModel = OrderRecordActiveModel {
            bill_no: ActiveValue::Set(bill_no.clone()),
            user_id: ActiveValue::Set(user_id),
            amount: ActiveValue::Set(request.get_amount()),
            category: ActiveValue::Set(request.get_category().clone()),
            transaction_type: ActiveValue::Set(request.get_transaction_type().clone()),
            description: ActiveValue::Set(request.try_get_description().clone()),
            bill_date: ActiveValue::Set(bill_date),
            id: ActiveValue::NotSet,
            created_at: ActiveValue::NotSet,
            updated_at: ActiveValue::NotSet,
        };
        let record_result: OrderRecordModel =
            RecordRepository::insert_with_transaction(&txn, record_active_model).await?;
        let record_id: i32 = record_result.get_id();
        let mut saved_images: Vec<RecordImageResponse> = Vec::new();
        for image_req in request.get_images() {
            let file_data: Vec<u8> = image_req.get_file_data().clone();
            let file_size: i32 = image_req.get_file_size();
            let image_active_model: OrderRecordImageActiveModel = OrderRecordImageActiveModel {
                record_id: ActiveValue::Set(record_id),
                user_id: ActiveValue::Set(user_id),
                file_name: ActiveValue::Set(image_req.get_file_name().clone()),
                original_name: ActiveValue::Set(image_req.try_get_original_name().clone()),
                mime_type: ActiveValue::Set(image_req.get_mime_type().clone()),
                file_size: ActiveValue::Set(file_size),
                file_data: ActiveValue::Set(file_data),
                id: ActiveValue::NotSet,
                created_at: ActiveValue::NotSet,
            };
            let image_result: OrderRecordImageModel =
                match RecordImageRepository::insert_with_transaction(&txn, image_active_model).await
                {
                    Ok(result) => result,
                    Err(error) => {
                        let _: Result<(), DbErr> = txn.rollback().await;
                        return Err(error);
                    }
                };
            let image_response: RecordImageResponse =
                Self::model_to_image_response(&image_result, record_id);
            saved_images.push(image_response);
        }
        txn.commit().await.map_err(|e: DbErr| e.to_string())?;
        let record_response: RecordResponse = Self::model_to_record_response(&record_result);
        let mut response: CreateRecordWithImagesResponse =
            CreateRecordWithImagesResponse::default();
        response
            .set_record(record_response)
            .set_images(saved_images);
        Ok(response)
    }

    #[instrument_trace]
    pub async fn create_record_with_single_image(
        user_id: i32,
        record_request: CreateRecordRequest,
        image_request: ImageUploadRequest,
    ) -> Result<CreateRecordWithImagesResponse, String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let bill_no: String = Self::generate_bill_no();
        let bill_date: NaiveDate = record_request
            .try_get_bill_date()
            .unwrap_or_else(|| Local::now().naive_local().date());
        let txn: DatabaseTransaction = db.begin().await.map_err(|e: DbErr| e.to_string())?;
        let record_active_model: OrderRecordActiveModel = OrderRecordActiveModel {
            bill_no: ActiveValue::Set(bill_no.clone()),
            user_id: ActiveValue::Set(user_id),
            amount: ActiveValue::Set(record_request.get_amount()),
            category: ActiveValue::Set(record_request.get_category().clone()),
            transaction_type: ActiveValue::Set(record_request.get_transaction_type().clone()),
            description: ActiveValue::Set(record_request.try_get_description().clone()),
            bill_date: ActiveValue::Set(bill_date),
            id: ActiveValue::NotSet,
            created_at: ActiveValue::NotSet,
            updated_at: ActiveValue::NotSet,
        };
        let record_result: OrderRecordModel =
            RecordRepository::insert_with_transaction(&txn, record_active_model).await?;
        let record_id: i32 = record_result.get_id();
        let image_active_model: OrderRecordImageActiveModel = OrderRecordImageActiveModel {
            record_id: ActiveValue::Set(record_id),
            user_id: ActiveValue::Set(user_id),
            file_name: ActiveValue::Set(image_request.get_file_name().clone()),
            original_name: ActiveValue::Set(image_request.try_get_original_name().clone()),
            mime_type: ActiveValue::Set(image_request.get_mime_type().clone()),
            file_size: ActiveValue::Set(image_request.get_file_size()),
            file_data: ActiveValue::Set(image_request.get_file_data().clone()),
            id: ActiveValue::NotSet,
            created_at: ActiveValue::NotSet,
        };
        let image_result: OrderRecordImageModel =
            match RecordImageRepository::insert_with_transaction(&txn, image_active_model).await {
                Ok(result) => result,
                Err(error) => {
                    let _: Result<(), DbErr> = txn.rollback().await;
                    return Err(error);
                }
            };
        txn.commit().await.map_err(|e: DbErr| e.to_string())?;
        let record_response: RecordResponse = Self::model_to_record_response(&record_result);
        let image_response: RecordImageResponse =
            Self::model_to_image_response(&image_result, record_id);
        let mut response: CreateRecordWithImagesResponse =
            CreateRecordWithImagesResponse::default();
        response
            .set_record(record_response)
            .set_images(vec![image_response]);
        Ok(response)
    }

    #[instrument_trace]
    pub async fn add_image_to_record(
        record_id: i32,
        user_id: i32,
        image_request: ImageUploadRequest,
    ) -> Result<CreateRecordWithImagesResponse, String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let record_model: OrderRecordModel = match RecordRepository::find_by_id(record_id).await? {
            Some(model) => model,
            None => return Err("Record not found".to_string()),
        };
        let txn: DatabaseTransaction = db.begin().await.map_err(|e: DbErr| e.to_string())?;
        let image_active_model: OrderRecordImageActiveModel = OrderRecordImageActiveModel {
            record_id: ActiveValue::Set(record_id),
            user_id: ActiveValue::Set(user_id),
            file_name: ActiveValue::Set(image_request.get_file_name().clone()),
            original_name: ActiveValue::Set(image_request.try_get_original_name().clone()),
            mime_type: ActiveValue::Set(image_request.get_mime_type().clone()),
            file_size: ActiveValue::Set(image_request.get_file_size()),
            file_data: ActiveValue::Set(image_request.get_file_data().clone()),
            id: ActiveValue::NotSet,
            created_at: ActiveValue::NotSet,
        };
        let image_result: OrderRecordImageModel =
            match RecordImageRepository::insert_with_transaction(&txn, image_active_model).await {
                Ok(result) => result,
                Err(error) => {
                    let _: Result<(), DbErr> = txn.rollback().await;
                    return Err(error);
                }
            };
        txn.commit().await.map_err(|e: DbErr| e.to_string())?;
        let record_response: RecordResponse = Self::model_to_record_response(&record_model);
        let image_response: RecordImageResponse =
            Self::model_to_image_response(&image_result, record_id);
        let mut response: CreateRecordWithImagesResponse =
            CreateRecordWithImagesResponse::default();
        response
            .set_record(record_response)
            .set_images(vec![image_response]);
        Ok(response)
    }

    #[instrument_trace]
    pub async fn get_record_images(record_id: i32) -> Result<RecordImageListResponse, String> {
        let images: Vec<OrderRecordImageModel> =
            RecordImageRepository::find_by_record_id(record_id).await?;
        let total_count: i32 = images.len() as i32;
        let user_ids: Vec<i32> = images
            .iter()
            .map(|img: &OrderRecordImageModel| img.get_user_id())
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();
        let users: Vec<OrderUserModel> = UserRepository::find_by_ids(user_ids).await?;
        let user_map: HashMap<i32, String> = users
            .into_iter()
            .map(|user: OrderUserModel| (user.get_id(), user.get_username().clone()))
            .collect();
        let mut image_responses: Vec<RecordImageResponse> = Vec::new();
        for img in &images {
            let mut response: RecordImageResponse = Self::model_to_image_response(img, record_id);
            let username: String = user_map
                .get(&img.get_user_id())
                .cloned()
                .unwrap_or_default();
            response.set_username(username);
            image_responses.push(response);
        }
        let mut response: RecordImageListResponse = RecordImageListResponse::default();
        response
            .set_images(image_responses)
            .set_total_count(total_count);
        Ok(response)
    }

    #[instrument_trace]
    pub async fn get_image_data(
        image_id: i32,
        user_id: i32,
    ) -> Result<Option<ImageDataResponse>, String> {
        let user_role: UserRole = match UserRepository::find_by_id(user_id).await? {
            Some(ref model) => UserRole::try_from(model.get_role()).unwrap_or_default(),
            None => return Err("User not found".to_string()),
        };
        if !user_role.is_admin() {
            return Err("Only admin can access image data".to_string());
        }
        match RecordImageRepository::find_by_id(image_id).await? {
            Some(model) => {
                let mut response: ImageDataResponse = ImageDataResponse::default();
                response
                    .set_id(model.get_id())
                    .set_record_id(model.get_record_id())
                    .set_file_name(model.get_file_name().clone())
                    .set_original_name(model.try_get_original_name().clone())
                    .set_mime_type(model.get_mime_type().clone())
                    .set_file_data(model.get_file_data().clone());
                Ok(Some(response))
            }
            None => Ok(None),
        }
    }

    #[instrument_trace]
    fn model_to_image_response(
        model: &OrderRecordImageModel,
        record_id: i32,
    ) -> RecordImageResponse {
        let created_at: String = model
            .try_get_created_at()
            .map(|dt: NaiveDateTime| dt.format("%Y-%m-%d %H:%M:%S").to_string())
            .unwrap_or_default();
        let mut response: RecordImageResponse = RecordImageResponse::default();
        let download_url: String = format!("/api/order/image/download/{}", model.get_id());
        response
            .set_id(model.get_id())
            .set_record_id(record_id)
            .set_user_id(model.get_user_id())
            .set_file_name(model.get_file_name().clone())
            .set_original_name(model.try_get_original_name().clone())
            .set_mime_type(model.get_mime_type().clone())
            .set_file_size(model.get_file_size())
            .set_created_at(created_at)
            .set_download_url(download_url);
        response
    }
}
