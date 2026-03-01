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

impl AccountBookingService {
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
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let existing_user: Option<AccountBookingUserModel> = AccountBookingUserEntity::find()
            .filter(AccountBookingUserColumn::Username.eq(request.get_username().clone()))
            .one(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        if existing_user.is_some() {
            return Err("Username already exists".to_string());
        }
        let password_hash: String = PasswordUtil::hash_password(request.get_password());
        let active_model: AccountBookingUserActiveModel = AccountBookingUserActiveModel {
            username: ActiveValue::Set(request.get_username().clone()),
            password_hash: ActiveValue::Set(password_hash),
            nickname: ActiveValue::Set(request.try_get_nickname().clone()),
            email: ActiveValue::Set(request.try_get_email().clone()),
            phone: ActiveValue::Set(request.try_get_phone().clone()),
            role: ActiveValue::Set("user".to_string()),
            status: ActiveValue::Set("pending".to_string()),
            id: ActiveValue::NotSet,
            created_at: ActiveValue::NotSet,
            updated_at: ActiveValue::NotSet,
        };
        let result: AccountBookingUserModel = active_model
            .insert(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(Self::model_to_user_response(&result))
    }

    #[instrument_trace]
    pub async fn login_user(request: LoginRequest) -> Result<(UserResponse, i32, String), String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let user: Option<AccountBookingUserModel> = AccountBookingUserEntity::find()
            .filter(AccountBookingUserColumn::Username.eq(request.get_username().clone()))
            .one(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        match user {
            Some(model) => {
                if model.get_status() != "approved" {
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
                let role: String = model.get_role().clone();
                Ok((user_response, user_id, role))
            }
            None => Err("User not found".to_string()),
        }
    }

    #[instrument_trace]
    pub async fn create_user(request: CreateUserRequest) -> Result<UserResponse, String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let existing_user: Option<AccountBookingUserModel> = AccountBookingUserEntity::find()
            .filter(AccountBookingUserColumn::Username.eq(request.get_username().clone()))
            .one(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        if existing_user.is_some() {
            return Err("Username already exists".to_string());
        }
        let password_hash: String = PasswordUtil::hash_password(request.get_password());
        let active_model: AccountBookingUserActiveModel = AccountBookingUserActiveModel {
            username: ActiveValue::Set(request.get_username().clone()),
            password_hash: ActiveValue::Set(password_hash),
            nickname: ActiveValue::Set(request.try_get_nickname().clone()),
            email: ActiveValue::Set(request.try_get_email().clone()),
            phone: ActiveValue::Set(request.try_get_phone().clone()),
            role: ActiveValue::Set(request.get_role().clone()),
            status: ActiveValue::Set("approved".to_string()),
            id: ActiveValue::NotSet,
            created_at: ActiveValue::NotSet,
            updated_at: ActiveValue::NotSet,
        };
        let result: AccountBookingUserModel = active_model
            .insert(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(Self::model_to_user_response(&result))
    }

    #[instrument_trace]
    pub async fn update_user(
        user_id: i32,
        request: UpdateUserRequest,
    ) -> Result<UserResponse, String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let user: Option<AccountBookingUserModel> = AccountBookingUserEntity::find_by_id(user_id)
            .one(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        match user {
            Some(model) => {
                let mut active_model: AccountBookingUserActiveModel = model.into();
                if let Some(nickname) = request.try_get_nickname() {
                    active_model.nickname = ActiveValue::Set(Some(nickname.clone()));
                }
                if let Some(email) = request.try_get_email() {
                    active_model.email = ActiveValue::Set(Some(email.clone()));
                }
                if let Some(phone) = request.try_get_phone() {
                    active_model.phone = ActiveValue::Set(Some(phone.clone()));
                }
                if let Some(role) = request.try_get_role() {
                    active_model.role = ActiveValue::Set(role.clone());
                }
                active_model.updated_at = ActiveValue::Set(Some(Local::now().naive_local()));
                let result: AccountBookingUserModel = active_model
                    .update(&db)
                    .await
                    .map_err(|error: DbErr| error.to_string())?;
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
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let user: Option<AccountBookingUserModel> = AccountBookingUserEntity::find_by_id(user_id)
            .one(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        match user {
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
                let mut active_model: AccountBookingUserActiveModel = model.into();
                active_model.password_hash = ActiveValue::Set(new_password_hash);
                active_model.updated_at = ActiveValue::Set(Some(Local::now().naive_local()));
                active_model
                    .update(&db)
                    .await
                    .map_err(|error: DbErr| error.to_string())?;
                Ok(())
            }
            None => Err("User not found".to_string()),
        }
    }

    #[instrument_trace]
    pub async fn approve_user(user_id: i32, approved: bool) -> Result<UserResponse, String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let user: Option<AccountBookingUserModel> = AccountBookingUserEntity::find_by_id(user_id)
            .one(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        match user {
            Some(model) => {
                let mut active_model: AccountBookingUserActiveModel = model.into();
                let status: String = if approved {
                    "approved".to_string()
                } else {
                    "rejected".to_string()
                };
                active_model.status = ActiveValue::Set(status);
                active_model.updated_at = ActiveValue::Set(Some(Local::now().naive_local()));
                let result: AccountBookingUserModel = active_model
                    .update(&db)
                    .await
                    .map_err(|error: DbErr| error.to_string())?;
                Ok(Self::model_to_user_response(&result))
            }
            None => Err("User not found".to_string()),
        }
    }

    #[instrument_trace]
    pub async fn list_users() -> Result<Vec<UserResponse>, String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let users: Vec<AccountBookingUserModel> = AccountBookingUserEntity::find()
            .all(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(users.iter().map(Self::model_to_user_response).collect())
    }

    #[instrument_trace]
    pub async fn get_user(user_id: i32) -> Result<Option<UserResponse>, String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let user: Option<AccountBookingUserModel> = AccountBookingUserEntity::find_by_id(user_id)
            .one(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(user.map(|model: AccountBookingUserModel| Self::model_to_user_response(&model)))
    }

    #[instrument_trace]
    fn model_to_user_response(model: &AccountBookingUserModel) -> UserResponse {
        let mut response: UserResponse = UserResponse::default();
        let created_at: Option<String> = model
            .try_get_created_at()
            .as_ref()
            .map(|dt: &NaiveDateTime| dt.format("%Y-%m-%d %H:%M:%S").to_string());
        response
            .set_id(model.get_id())
            .set_username(model.get_username().clone())
            .set_nickname(model.try_get_nickname().clone())
            .set_email(model.try_get_email().clone())
            .set_phone(model.try_get_phone().clone())
            .set_role(model.get_role().clone())
            .set_status(model.get_status().clone())
            .set_created_at(created_at);
        response
    }

    #[instrument_trace]
    pub async fn create_record(
        user_id: i32,
        request: CreateRecordRequest,
    ) -> Result<RecordResponse, String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let bill_no: String = Self::generate_bill_no();
        let active_model: AccountBookingRecordActiveModel = AccountBookingRecordActiveModel {
            bill_no: ActiveValue::Set(bill_no.clone()),
            user_id: ActiveValue::Set(user_id),
            amount: ActiveValue::Set(request.get_amount()),
            category: ActiveValue::Set(request.get_category().clone()),
            transaction_type: ActiveValue::Set(request.get_transaction_type().clone()),
            description: ActiveValue::Set(request.try_get_description().clone()),
            bill_date: ActiveValue::Set(request.get_bill_date()),
            id: ActiveValue::NotSet,
            created_at: ActiveValue::NotSet,
            updated_at: ActiveValue::NotSet,
        };
        let result: AccountBookingRecordModel = active_model
            .insert(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(Self::model_to_record_response(&result))
    }

    #[instrument_trace]
    pub async fn list_records(query: RecordQueryRequest) -> Result<RecordListResponse, String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let mut base_select: Select<AccountBookingRecordEntity> =
            AccountBookingRecordEntity::find();
        if let Some(user_id) = query.try_get_user_id() {
            base_select = base_select.filter(AccountBookingRecordColumn::UserId.eq(user_id));
        }
        if let Some(start_date) = query.try_get_start_date() {
            base_select = base_select.filter(AccountBookingRecordColumn::BillDate.gte(*start_date));
        }
        if let Some(end_date) = query.try_get_end_date() {
            base_select = base_select.filter(AccountBookingRecordColumn::BillDate.lte(*end_date));
        }
        if let Some(category) = query.try_get_category() {
            base_select =
                base_select.filter(AccountBookingRecordColumn::Category.eq(category.clone()));
        }
        if let Some(transaction_type) = query.try_get_transaction_type() {
            base_select = base_select
                .filter(AccountBookingRecordColumn::TransactionType.eq(transaction_type.clone()));
        }
        let total_count_u64: u64 = base_select
            .clone()
            .count(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        let total_count: i64 = total_count_u64 as i64;
        let all_records: Vec<AccountBookingRecordModel> = base_select
            .clone()
            .all(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        let mut total_income: Decimal = Decimal::ZERO;
        let mut total_expense: Decimal = Decimal::ZERO;
        for record in &all_records {
            let amount: Decimal = *record.get_amount();
            if record.get_transaction_type() == "income" {
                total_income += amount;
            } else {
                total_expense += amount;
            }
        }
        let balance: Decimal = total_income - total_expense;
        let mut paged_select: Select<AccountBookingRecordEntity> = base_select;
        if let Some(last_id) = query.try_get_last_id() {
            paged_select = paged_select.filter(AccountBookingRecordColumn::Id.lt(last_id));
        }
        paged_select = paged_select.order_by_desc(AccountBookingRecordColumn::Id);
        let limit: i32 = query.try_get_limit().unwrap_or(20);
        let limit_with_extra: i32 = limit + 1;
        paged_select = paged_select.limit(limit_with_extra as u64);
        let paged_records: Vec<AccountBookingRecordModel> = paged_select
            .all(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        let has_more: bool = paged_records.len() > limit as usize;
        let paged_records: Vec<AccountBookingRecordModel> =
            paged_records.into_iter().take(limit as usize).collect();
        let last_id: Option<i32> = paged_records
            .last()
            .map(|r: &AccountBookingRecordModel| r.get_id());
        let record_responses: Vec<RecordResponse> = paged_records
            .iter()
            .map(Self::model_to_record_response)
            .collect();
        let mut response: RecordListResponse = RecordListResponse::default();
        response
            .set_records(record_responses)
            .set_total_income(total_income.to_string())
            .set_total_expense(total_expense.to_string())
            .set_balance(balance.to_string())
            .set_has_more(has_more)
            .set_last_id(last_id)
            .set_total_count(total_count);
        Ok(response)
    }

    #[instrument_trace]
    pub async fn get_record(record_id: i32) -> Result<Option<RecordResponse>, String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let record: Option<AccountBookingRecordModel> =
            AccountBookingRecordEntity::find_by_id(record_id)
                .one(&db)
                .await
                .map_err(|error: DbErr| error.to_string())?;
        Ok(record.map(|model: AccountBookingRecordModel| Self::model_to_record_response(&model)))
    }

    #[instrument_trace]
    fn model_to_record_response(model: &AccountBookingRecordModel) -> RecordResponse {
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
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let today: NaiveDate = Local::now().naive_local().date();
        let yesterday: NaiveDate = today - chrono::Duration::days(1);
        let today_stats: (i64, Decimal, Decimal) = Self::get_date_statistics(&db, today).await?;
        let yesterday_stats: (i64, Decimal, Decimal) =
            Self::get_date_statistics(&db, yesterday).await?;
        let daily_trend: DailyTrend = Self::get_daily_trend(&db, 30).await?;
        let monthly_comparison: MonthlyComparison = Self::get_monthly_comparison(&db, 6).await?;
        let category_distribution: Vec<CategoryItem> = Self::get_category_distribution(&db).await?;
        let user_growth: UserGrowth = Self::get_user_growth(&db, 30).await?;
        let transaction_type_distribution: TransactionTypeDistribution =
            Self::get_transaction_type_distribution(&db).await?;
        let transaction_count_trend: TransactionCountTrend =
            Self::get_transaction_count_trend(&db, 30).await?;
        let category_amount_distribution: Vec<CategoryAmountItem> =
            Self::get_category_amount_distribution(&db).await?;
        let user_activity: UserActivity = Self::get_user_activity(&db, 30).await?;
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
        let today_new_users: i64 = Self::get_new_users_count(&db, today).await?;
        let yesterday_new_users: i64 = Self::get_new_users_count(&db, yesterday).await?;
        let new_users_change: Option<f64> =
            Self::calculate_change_percentage(today_new_users as f64, yesterday_new_users as f64);
        let mut today_statistics: TodayStatistics = TodayStatistics::default();
        today_statistics.set_transactions(today_stats.0);
        today_statistics.set_income(today_stats.1.to_string());
        today_statistics.set_expense(today_stats.2.to_string());
        today_statistics.set_new_users(today_new_users);
        let mut changes_statistics: ChangesStatistics = ChangesStatistics::default();
        changes_statistics.set_transactions_change(transactions_change);
        changes_statistics.set_income_change(income_change);
        changes_statistics.set_expense_change(expense_change);
        changes_statistics.set_new_users_change(new_users_change);
        let mut response: OverviewStatisticsResponse = OverviewStatisticsResponse::default();
        response.set_today(today_statistics);
        response.set_changes(changes_statistics);
        response.set_daily_trend(daily_trend);
        response.set_monthly_comparison(monthly_comparison);
        response.set_category_distribution(category_distribution);
        response.set_user_growth(user_growth);
        response.set_transaction_type_distribution(transaction_type_distribution);
        response.set_transaction_count_trend(transaction_count_trend);
        response.set_category_amount_distribution(category_amount_distribution);
        response.set_user_activity(user_activity);
        Ok(response)
    }

    #[instrument_trace]
    async fn get_date_statistics(
        db: &DatabaseConnection,
        date: NaiveDate,
    ) -> Result<(i64, Decimal, Decimal), String> {
        let records: Vec<AccountBookingRecordModel> = AccountBookingRecordEntity::find()
            .filter(AccountBookingRecordColumn::BillDate.eq(date))
            .all(db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        let transaction_count: i64 = records.len() as i64;
        let mut total_income: Decimal = Decimal::ZERO;
        let mut total_expense: Decimal = Decimal::ZERO;
        for record in &records {
            let amount: Decimal = *record.get_amount();
            if record.get_transaction_type() == "income" {
                total_income += amount;
            } else {
                total_expense += amount;
            }
        }
        Ok((transaction_count, total_income, total_expense))
    }

    #[instrument_trace]
    async fn get_new_users_count(db: &DatabaseConnection, date: NaiveDate) -> Result<i64, String> {
        let start_of_day: NaiveDateTime = date.and_hms_opt(0, 0, 0).unwrap();
        let end_of_day: NaiveDateTime = date.and_hms_opt(23, 59, 59).unwrap();
        let count_u64: u64 = AccountBookingUserEntity::find()
            .filter(AccountBookingUserColumn::CreatedAt.gte(start_of_day))
            .filter(AccountBookingUserColumn::CreatedAt.lte(end_of_day))
            .count(db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(count_u64 as i64)
    }

    #[instrument_trace]
    async fn get_daily_trend(db: &DatabaseConnection, days: i64) -> Result<DailyTrend, String> {
        let end_date: NaiveDate = Local::now().naive_local().date();
        let start_date: NaiveDate = end_date - chrono::Duration::days(days);
        let records: Vec<AccountBookingRecordModel> = AccountBookingRecordEntity::find()
            .filter(AccountBookingRecordColumn::BillDate.gte(start_date))
            .filter(AccountBookingRecordColumn::BillDate.lte(end_date))
            .order_by_asc(AccountBookingRecordColumn::BillDate)
            .all(db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        let mut dates: Vec<String> = Vec::new();
        let mut income: Vec<String> = Vec::new();
        let mut expense: Vec<String> = Vec::new();
        let mut current_date: NaiveDate = start_date;
        while current_date <= end_date {
            dates.push(current_date.to_string());
            let day_records: Vec<&AccountBookingRecordModel> = records
                .iter()
                .filter(|r| *r.get_bill_date() == current_date)
                .collect();
            let mut day_income: Decimal = Decimal::ZERO;
            let mut day_expense: Decimal = Decimal::ZERO;
            for record in day_records {
                let amount: Decimal = *record.get_amount();
                if record.get_transaction_type() == "income" {
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
    async fn get_monthly_comparison(
        db: &DatabaseConnection,
        months: i64,
    ) -> Result<MonthlyComparison, String> {
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
            let records: Vec<AccountBookingRecordModel> = AccountBookingRecordEntity::find()
                .filter(AccountBookingRecordColumn::BillDate.gte(start_date))
                .filter(AccountBookingRecordColumn::BillDate.lte(end_date))
                .all(db)
                .await
                .map_err(|error: DbErr| error.to_string())?;
            let mut total_income: Decimal = Decimal::ZERO;
            let mut total_expense: Decimal = Decimal::ZERO;
            for record in &records {
                let amount: Decimal = *record.get_amount();
                if record.get_transaction_type() == "income" {
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
    async fn get_category_distribution(
        db: &DatabaseConnection,
    ) -> Result<Vec<CategoryItem>, String> {
        let records: Vec<AccountBookingRecordModel> = AccountBookingRecordEntity::find()
            .filter(AccountBookingRecordColumn::TransactionType.eq("expense"))
            .all(db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        let mut category_map: std::collections::HashMap<String, i64> =
            std::collections::HashMap::new();
        for record in &records {
            let category: String = record.get_category().clone();
            *category_map.entry(category).or_insert(0) += 1;
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
    async fn get_user_growth(db: &DatabaseConnection, days: i64) -> Result<UserGrowth, String> {
        let end_date: NaiveDate = Local::now().naive_local().date();
        let start_date: NaiveDate = end_date - chrono::Duration::days(days);
        let mut dates: Vec<String> = Vec::new();
        let mut counts: Vec<i64> = Vec::new();
        let mut current_date: NaiveDate = start_date;
        while current_date <= end_date {
            dates.push(current_date.to_string());
            let start_of_day: NaiveDateTime = current_date.and_hms_opt(0, 0, 0).unwrap();
            let end_of_day: NaiveDateTime = current_date.and_hms_opt(23, 59, 59).unwrap();
            let count_u64: u64 = AccountBookingUserEntity::find()
                .filter(AccountBookingUserColumn::CreatedAt.gte(start_of_day))
                .filter(AccountBookingUserColumn::CreatedAt.lte(end_of_day))
                .count(db)
                .await
                .map_err(|error: DbErr| error.to_string())?;
            counts.push(count_u64 as i64);
            current_date += chrono::Duration::days(1);
        }
        let mut growth: UserGrowth = UserGrowth::default();
        growth.set_dates(dates);
        growth.set_counts(counts);
        Ok(growth)
    }

    #[instrument_trace]
    async fn get_transaction_type_distribution(
        db: &DatabaseConnection,
    ) -> Result<TransactionTypeDistribution, String> {
        let records: Vec<AccountBookingRecordModel> = AccountBookingRecordEntity::find()
            .all(db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        let mut income_count: i64 = 0;
        let mut expense_count: i64 = 0;
        let mut income_amount: Decimal = Decimal::ZERO;
        let mut expense_amount: Decimal = Decimal::ZERO;
        for record in &records {
            let amount: Decimal = *record.get_amount();
            if record.get_transaction_type() == "income" {
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
    async fn get_transaction_count_trend(
        db: &DatabaseConnection,
        days: i64,
    ) -> Result<TransactionCountTrend, String> {
        let end_date: NaiveDate = Local::now().naive_local().date();
        let start_date: NaiveDate = end_date - chrono::Duration::days(days);
        let records: Vec<AccountBookingRecordModel> = AccountBookingRecordEntity::find()
            .filter(AccountBookingRecordColumn::BillDate.gte(start_date))
            .filter(AccountBookingRecordColumn::BillDate.lte(end_date))
            .order_by_asc(AccountBookingRecordColumn::BillDate)
            .all(db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
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
    async fn get_category_amount_distribution(
        db: &DatabaseConnection,
    ) -> Result<Vec<CategoryAmountItem>, String> {
        let records: Vec<AccountBookingRecordModel> = AccountBookingRecordEntity::find()
            .filter(AccountBookingRecordColumn::TransactionType.eq("expense"))
            .all(db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        let mut category_map: std::collections::HashMap<String, Decimal> =
            std::collections::HashMap::new();
        for record in &records {
            let category: String = record.get_category().clone();
            let amount: Decimal = *record.get_amount();
            *category_map.entry(category).or_insert(Decimal::ZERO) += amount;
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
    async fn get_user_activity(db: &DatabaseConnection, days: i64) -> Result<UserActivity, String> {
        let end_date: NaiveDate = Local::now().naive_local().date();
        let start_date: NaiveDate = end_date - chrono::Duration::days(days);
        let mut dates: Vec<String> = Vec::new();
        let mut active_users: Vec<i64> = Vec::new();
        let mut new_records: Vec<i64> = Vec::new();
        let records: Vec<AccountBookingRecordModel> = AccountBookingRecordEntity::find()
            .filter(AccountBookingRecordColumn::BillDate.gte(start_date))
            .filter(AccountBookingRecordColumn::BillDate.lte(end_date))
            .all(db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        let mut current_date: NaiveDate = start_date;
        while current_date <= end_date {
            dates.push(current_date.to_string());
            let day_records: Vec<&AccountBookingRecordModel> = records
                .iter()
                .filter(|r| *r.get_bill_date() == current_date)
                .collect();
            let unique_users: std::collections::HashSet<i32> = day_records
                .iter()
                .map(|r| r.get_user_id())
                .collect();
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
}
