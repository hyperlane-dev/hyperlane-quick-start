use super::*;

impl UserRepository {
    #[instrument_trace]
    pub async fn find_by_id(user_id: i32) -> Result<Option<OrderUserModel>, String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let result: Option<OrderUserModel> = OrderUserEntity::find_by_id(user_id)
            .one(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(result)
    }

    #[instrument_trace]
    pub async fn find_by_username(username: String) -> Result<Option<OrderUserModel>, String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let result: Option<OrderUserModel> = OrderUserEntity::find()
            .filter(OrderUserColumn::Username.eq(username))
            .one(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(result)
    }

    #[instrument_trace]
    pub async fn find_by_ids(user_ids: Vec<i32>) -> Result<Vec<OrderUserModel>, String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let result: Vec<OrderUserModel> = OrderUserEntity::find()
            .filter(OrderUserColumn::Id.is_in(user_ids))
            .all(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(result)
    }

    #[instrument_trace]
    pub async fn insert(active_model: OrderUserActiveModel) -> Result<OrderUserModel, String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let result: OrderUserModel = active_model
            .insert(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(result)
    }

    #[instrument_trace]
    pub async fn update(active_model: OrderUserActiveModel) -> Result<OrderUserModel, String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let result: OrderUserModel = active_model
            .update(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(result)
    }

    #[instrument_trace]
    pub async fn query_with_pagination(
        keyword: Option<String>,
        last_id: Option<i32>,
        limit: u64,
    ) -> Result<(Vec<OrderUserModel>, i64, bool), String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let mut base_select: Select<OrderUserEntity> = OrderUserEntity::find();
        if let Some(keyword) = keyword {
            let keyword_pattern: String = format!("%{keyword}%");
            let mut condition: Condition = Condition::any()
                .add(OrderUserColumn::Username.like(keyword_pattern.clone()))
                .add(OrderUserColumn::Email.like(keyword_pattern.clone()))
                .add(OrderUserColumn::Phone.like(keyword_pattern.clone()));
            if let Ok(user_id) = keyword.parse::<i32>() {
                condition = condition.add(OrderUserColumn::Id.eq(user_id));
            }
            base_select = base_select.filter(condition);
        }
        let total_count_u64: u64 = base_select
            .clone()
            .count(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        let total_count: i64 = total_count_u64 as i64;
        let mut paged_select: Select<OrderUserEntity> = base_select;
        if let Some(last_id) = last_id {
            paged_select = paged_select.filter(OrderUserColumn::Id.lt(last_id));
        }
        paged_select = paged_select.order_by_desc(OrderUserColumn::Id);
        let limit_with_extra: u64 = limit + 1;
        paged_select = paged_select.limit(limit_with_extra);
        let paged_users: Vec<OrderUserModel> = paged_select
            .all(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        let has_more: bool = paged_users.len() > limit as usize;
        let paged_users: Vec<OrderUserModel> =
            paged_users.into_iter().take(limit as usize).collect();
        Ok((paged_users, total_count, has_more))
    }

    #[instrument_trace]
    pub async fn count_by_created_at_range(
        start: NaiveDateTime,
        end: NaiveDateTime,
    ) -> Result<i64, String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let count_u64: u64 = OrderUserEntity::find()
            .filter(OrderUserColumn::CreatedAt.gte(start))
            .filter(OrderUserColumn::CreatedAt.lte(end))
            .count(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(count_u64 as i64)
    }

    #[instrument_trace]
    pub async fn find_by_created_at_range(
        start: NaiveDateTime,
        end: NaiveDateTime,
    ) -> Result<Vec<OrderUserModel>, String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let result: Vec<OrderUserModel> = OrderUserEntity::find()
            .filter(OrderUserColumn::CreatedAt.gte(start))
            .filter(OrderUserColumn::CreatedAt.lte(end))
            .all(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(result)
    }
}

impl RecordRepository {
    #[instrument_trace]
    pub async fn find_by_id(record_id: i32) -> Result<Option<OrderRecordModel>, String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let result: Option<OrderRecordModel> = OrderRecordEntity::find_by_id(record_id)
            .one(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(result)
    }

    #[instrument_trace]
    pub async fn insert(active_model: OrderRecordActiveModel) -> Result<OrderRecordModel, String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let result: OrderRecordModel = active_model
            .insert(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(result)
    }

    #[instrument_trace]
    pub async fn insert_with_transaction(
        txn: &DatabaseTransaction,
        active_model: OrderRecordActiveModel,
    ) -> Result<OrderRecordModel, String> {
        let result: OrderRecordModel = active_model
            .insert(txn)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(result)
    }

    #[instrument_trace]
    pub async fn query_with_filters(
        user_id: Option<i32>,
        start_date: Option<NaiveDate>,
        end_date: Option<NaiveDate>,
        category: Option<String>,
        transaction_type: Option<String>,
        cache_id: Option<i32>,
    ) -> Result<Vec<OrderRecordModel>, String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let mut base_select: sea_orm::Select<OrderRecordEntity> = OrderRecordEntity::find();
        if let Some(user_id) = user_id {
            base_select = base_select.filter(OrderRecordColumn::UserId.eq(user_id));
        }
        if let Some(start_date) = start_date {
            base_select = base_select.filter(OrderRecordColumn::BillDate.gte(start_date));
        }
        if let Some(end_date) = end_date {
            base_select = base_select.filter(OrderRecordColumn::BillDate.lte(end_date));
        }
        if let Some(category) = category {
            base_select = base_select.filter(OrderRecordColumn::Category.eq(category));
        }
        if let Some(transaction_type) = transaction_type {
            base_select =
                base_select.filter(OrderRecordColumn::TransactionType.eq(transaction_type));
        }
        if let Some(cache_id) = cache_id {
            base_select = base_select.filter(OrderRecordColumn::Id.lte(cache_id));
        }
        let result: Vec<OrderRecordModel> = base_select
            .all(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(result)
    }

    #[instrument_trace]
    pub async fn query_with_pagination(
        query: RecordPaginationQuery,
    ) -> Result<(Vec<OrderRecordModel>, i64), String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let mut base_select: sea_orm::Select<OrderRecordEntity> = OrderRecordEntity::find();
        if let Some(user_id) = query.try_get_user_id() {
            base_select = base_select.filter(OrderRecordColumn::UserId.eq(user_id));
        }
        if let Some(start_date) = query.try_get_start_date() {
            base_select = base_select.filter(OrderRecordColumn::BillDate.gte(start_date));
        }
        if let Some(end_date) = query.try_get_end_date() {
            base_select = base_select.filter(OrderRecordColumn::BillDate.lte(end_date));
        }
        if let Some(category) = query.try_get_category() {
            base_select = base_select.filter(OrderRecordColumn::Category.eq(category));
        }
        if let Some(transaction_type) = query.try_get_transaction_type() {
            base_select =
                base_select.filter(OrderRecordColumn::TransactionType.eq(transaction_type));
        }
        if let Some(cache_id) = query.try_get_cache_id() {
            base_select = base_select.filter(OrderRecordColumn::Id.lte(cache_id));
        }
        let total_count: i64 = base_select
            .clone()
            .count(&db)
            .await
            .map_err(|error: DbErr| error.to_string())? as i64;
        let offset: u64 = ((query.get_page() - 1) as u64) * query.get_limit();
        let paged_records: Vec<OrderRecordModel> = base_select
            .order_by_desc(OrderRecordColumn::Id)
            .limit(query.get_limit())
            .offset(offset)
            .all(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok((paged_records, total_count))
    }

    #[instrument_trace]
    pub async fn sum_amount_by_transaction_type(
        user_id: Option<i32>,
        start_date: Option<NaiveDate>,
        end_date: Option<NaiveDate>,
        category: Option<String>,
        cache_id: Option<i32>,
        transaction_type: String,
    ) -> Result<Decimal, String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let mut base_select: sea_orm::Select<OrderRecordEntity> = OrderRecordEntity::find();
        if let Some(user_id) = user_id {
            base_select = base_select.filter(OrderRecordColumn::UserId.eq(user_id));
        }
        if let Some(start_date) = start_date {
            base_select = base_select.filter(OrderRecordColumn::BillDate.gte(start_date));
        }
        if let Some(end_date) = end_date {
            base_select = base_select.filter(OrderRecordColumn::BillDate.lte(end_date));
        }
        if let Some(category) = category {
            base_select = base_select.filter(OrderRecordColumn::Category.eq(category));
        }
        if let Some(cache_id) = cache_id {
            base_select = base_select.filter(OrderRecordColumn::Id.lte(cache_id));
        }
        let result: Option<(Option<Decimal>,)> = base_select
            .filter(OrderRecordColumn::TransactionType.eq(transaction_type))
            .select_only()
            .column_as(OrderRecordColumn::Amount.sum(), "sum")
            .into_tuple()
            .one(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        let sum: Decimal = match result {
            Some((Some(decimal),)) => decimal,
            _ => Decimal::ZERO,
        };
        Ok(sum)
    }

    #[instrument_trace]
    pub async fn find_by_bill_date_range(
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<Vec<OrderRecordModel>, String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let result: Vec<OrderRecordModel> = OrderRecordEntity::find()
            .filter(OrderRecordColumn::BillDate.gte(start_date))
            .filter(OrderRecordColumn::BillDate.lte(end_date))
            .order_by_asc(OrderRecordColumn::BillDate)
            .all(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(result)
    }

    #[instrument_trace]
    pub async fn find_by_bill_date_range_and_transaction_type(
        start_date: NaiveDate,
        end_date: NaiveDate,
        transaction_type: String,
    ) -> Result<Vec<OrderRecordModel>, String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let result: Vec<OrderRecordModel> = OrderRecordEntity::find()
            .filter(OrderRecordColumn::BillDate.gte(start_date))
            .filter(OrderRecordColumn::BillDate.lte(end_date))
            .filter(OrderRecordColumn::TransactionType.eq(transaction_type))
            .all(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(result)
    }

    #[instrument_trace]
    pub async fn find_by_bill_date(bill_date: NaiveDate) -> Result<Vec<OrderRecordModel>, String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let result: Vec<OrderRecordModel> = OrderRecordEntity::find()
            .filter(OrderRecordColumn::BillDate.eq(bill_date))
            .all(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(result)
    }

    #[instrument_trace]
    pub async fn find_all() -> Result<Vec<OrderRecordModel>, String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let result: Vec<OrderRecordModel> = OrderRecordEntity::find()
            .all(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(result)
    }
}

impl RecordImageRepository {
    #[instrument_trace]
    pub async fn find_by_id(image_id: i32) -> Result<Option<OrderRecordImageModel>, String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let result: Option<OrderRecordImageModel> = OrderRecordImageEntity::find_by_id(image_id)
            .one(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(result)
    }

    #[instrument_trace]
    pub async fn find_by_record_id(record_id: i32) -> Result<Vec<OrderRecordImageModel>, String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let result: Vec<OrderRecordImageModel> = OrderRecordImageEntity::find()
            .filter(OrderRecordImageColumn::RecordId.eq(record_id))
            .order_by_desc(OrderRecordImageColumn::Id)
            .all(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(result)
    }

    #[instrument_trace]
    pub async fn insert(
        active_model: OrderRecordImageActiveModel,
    ) -> Result<OrderRecordImageModel, String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let result: OrderRecordImageModel = active_model
            .insert(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(result)
    }

    #[instrument_trace]
    pub async fn insert_with_transaction(
        txn: &DatabaseTransaction,
        active_model: OrderRecordImageActiveModel,
    ) -> Result<OrderRecordImageModel, String> {
        let result: OrderRecordImageModel = active_model
            .insert(txn)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(result)
    }

    #[instrument_trace]
    pub async fn find_by_user_ids(
        user_ids: Vec<i32>,
    ) -> Result<Vec<OrderRecordImageModel>, String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let result: Vec<OrderRecordImageModel> = OrderRecordImageEntity::find()
            .filter(OrderRecordImageColumn::UserId.is_in(user_ids))
            .all(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(result)
    }
}
