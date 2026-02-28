use super::*;

#[derive(Clone, Debug, Default)]
pub struct PasswordUtil;

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
        let existing_user: Option<UserModel> = UserEntity::find()
            .filter(UserColumn::Username.eq(request.get_username().clone()))
            .one(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        if existing_user.is_some() {
            return Err("Username already exists".to_string());
        }
        let password_hash: String = PasswordUtil::hash_password(request.get_password());
        let active_model: UserActiveModel = UserActiveModel {
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
        let result: UserModel = active_model
            .insert(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(Self::model_to_user_response(&result))
    }

    #[instrument_trace]
    pub async fn login_user(request: LoginRequest) -> Result<LoginResponse, String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let user: Option<UserModel> = UserEntity::find()
            .filter(UserColumn::Username.eq(request.get_username().clone()))
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
                let mut response: LoginResponse = LoginResponse::default();
                response
                    .set_user(Self::model_to_user_response(&model))
                    .set_token(format!(
                        "token_{}_{}",
                        model.get_id(),
                        Local::now().timestamp()
                    ));
                Ok(response)
            }
            None => Err("User not found".to_string()),
        }
    }

    #[instrument_trace]
    pub async fn create_user(request: CreateUserRequest) -> Result<UserResponse, String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let existing_user: Option<UserModel> = UserEntity::find()
            .filter(UserColumn::Username.eq(request.get_username().clone()))
            .one(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        if existing_user.is_some() {
            return Err("Username already exists".to_string());
        }
        let password_hash: String = PasswordUtil::hash_password(request.get_password());
        let active_model: UserActiveModel = UserActiveModel {
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
        let result: UserModel = active_model
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
        let user: Option<UserModel> = UserEntity::find_by_id(user_id)
            .one(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        match user {
            Some(model) => {
                let mut active_model: UserActiveModel = model.into();
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
                let result: UserModel = active_model
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
        let user: Option<UserModel> = UserEntity::find_by_id(user_id)
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
                let mut active_model: UserActiveModel = model.into();
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
        let user: Option<UserModel> = UserEntity::find_by_id(user_id)
            .one(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        match user {
            Some(model) => {
                let mut active_model: UserActiveModel = model.into();
                let status: String = if approved {
                    "approved".to_string()
                } else {
                    "rejected".to_string()
                };
                active_model.status = ActiveValue::Set(status);
                active_model.updated_at = ActiveValue::Set(Some(Local::now().naive_local()));
                let result: UserModel = active_model
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
        let users: Vec<UserModel> = UserEntity::find()
            .all(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(users.iter().map(Self::model_to_user_response).collect())
    }

    #[instrument_trace]
    pub async fn get_user(user_id: i32) -> Result<Option<UserResponse>, String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let user: Option<UserModel> = UserEntity::find_by_id(user_id)
            .one(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(user.map(|model: UserModel| Self::model_to_user_response(&model)))
    }

    #[instrument_trace]
    fn model_to_user_response(model: &UserModel) -> UserResponse {
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
        let active_model: RecordActiveModel = RecordActiveModel {
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
        let result: RecordModel = active_model
            .insert(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(Self::model_to_record_response(&result))
    }

    #[instrument_trace]
    pub async fn update_record(
        record_id: i32,
        user_id: i32,
        request: UpdateRecordRequest,
    ) -> Result<RecordResponse, String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let record: Option<RecordModel> = RecordEntity::find_by_id(record_id)
            .one(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        match record {
            Some(model) => {
                if model.get_user_id() != user_id {
                    return Err("Permission denied".to_string());
                }
                let mut active_model: RecordActiveModel = model.into();
                if let Some(amount) = request.try_get_amount() {
                    active_model.amount = ActiveValue::Set(*amount);
                }
                if let Some(category) = request.try_get_category() {
                    active_model.category = ActiveValue::Set(category.clone());
                }
                if let Some(transaction_type) = request.try_get_transaction_type() {
                    active_model.transaction_type = ActiveValue::Set(transaction_type.clone());
                }
                if let Some(description) = request.try_get_description() {
                    active_model.description = ActiveValue::Set(Some(description.clone()));
                }
                if let Some(bill_date) = request.try_get_bill_date() {
                    active_model.bill_date = ActiveValue::Set(*bill_date);
                }
                active_model.updated_at = ActiveValue::Set(Some(Local::now().naive_local()));
                let result: RecordModel = active_model
                    .update(&db)
                    .await
                    .map_err(|error: DbErr| error.to_string())?;
                Ok(Self::model_to_record_response(&result))
            }
            None => Err("Record not found".to_string()),
        }
    }

    #[instrument_trace]
    pub async fn list_records(query: RecordQueryRequest) -> Result<RecordListResponse, String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let mut select: Select<RecordEntity> = RecordEntity::find();
        if let Some(user_id) = query.try_get_user_id() {
            select = select.filter(RecordColumn::UserId.eq(user_id));
        }
        if let Some(start_date) = query.try_get_start_date() {
            select = select.filter(RecordColumn::BillDate.gte(*start_date));
        }
        if let Some(end_date) = query.try_get_end_date() {
            select = select.filter(RecordColumn::BillDate.lte(*end_date));
        }
        if let Some(category) = query.try_get_category() {
            select = select.filter(RecordColumn::Category.eq(category.clone()));
        }
        if let Some(transaction_type) = query.try_get_transaction_type() {
            select = select.filter(RecordColumn::TransactionType.eq(transaction_type.clone()));
        }
        select = select.order_by_desc(RecordColumn::BillDate);
        let records: Vec<RecordModel> = select
            .all(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        let record_responses: Vec<RecordResponse> =
            records.iter().map(Self::model_to_record_response).collect();
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
        let balance: Decimal = total_income - total_expense;
        let mut response: RecordListResponse = RecordListResponse::default();
        response
            .set_records(record_responses)
            .set_total_income(total_income.to_string())
            .set_total_expense(total_expense.to_string())
            .set_balance(balance.to_string());
        Ok(response)
    }

    #[instrument_trace]
    pub async fn get_record(record_id: i32) -> Result<Option<RecordResponse>, String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let record: Option<RecordModel> = RecordEntity::find_by_id(record_id)
            .one(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(record.map(|model: RecordModel| Self::model_to_record_response(&model)))
    }

    #[instrument_trace]
    fn model_to_record_response(model: &RecordModel) -> RecordResponse {
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
}
