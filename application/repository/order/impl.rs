use super::*;

/// Database access methods for `RecordRepository`.
impl RecordRepository {
    /// Finds an order record by its unique identifier.
    ///
    /// # Arguments
    ///
    /// - `i32`: The record identifier.
    ///
    /// # Returns
    ///
    /// - `Result<Option<OrderRecordModel>, String>`: The record model if found, or `None`.
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

    /// Inserts a new order record into the database.
    ///
    /// # Arguments
    ///
    /// - `OrderRecordActiveModel`: The active model containing the record data to insert.
    ///
    /// # Returns
    ///
    /// - `Result<OrderRecordModel, String>`: The inserted record model.
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

    /// Inserts a new order record within an existing database transaction.
    ///
    /// # Arguments
    ///
    /// - `&DatabaseTransaction`: The active database transaction.
    /// - `OrderRecordActiveModel`: The active model containing the record data to insert.
    ///
    /// # Returns
    ///
    /// - `Result<OrderRecordModel, String>`: The inserted record model.
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

    /// Queries order records with multiple optional filters for user, date range, category, and type.
    ///
    /// # Arguments
    ///
    /// - `Option<i32>`: Optional user identifier filter.
    /// - `Option<NaiveDate>`: Optional start date filter.
    /// - `Option<NaiveDate>`: Optional end date filter.
    /// - `Option<String>`: Optional category filter.
    /// - `Option<String>`: Optional transaction type filter.
    /// - `Option<i32>`: Optional cache ID for cursor-based pagination.
    ///
    /// # Returns
    ///
    /// - `Result<Vec<OrderRecordModel>, String>`: The filtered list of order records.
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

    /// Queries order records with pagination and multiple filter criteria.
    ///
    /// # Arguments
    ///
    /// - `RecordPaginationQuery`: The pagination query parameters including filters.
    ///
    /// # Returns
    ///
    /// - `Result<(Vec<OrderRecordModel>, i64), String>`: The paginated records and total count.
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

    /// Sums the amount of order records matching the given filters and transaction type.
    ///
    /// # Arguments
    ///
    /// - `Option<i32>`: Optional user identifier filter.
    /// - `Option<NaiveDate>`: Optional start date filter.
    /// - `Option<NaiveDate>`: Optional end date filter.
    /// - `Option<String>`: Optional category filter.
    /// - `Option<i32>`: Optional cache ID for cursor-based pagination.
    /// - `String`: The transaction type to sum (e.g., "income" or "expense").
    ///
    /// # Returns
    ///
    /// - `Result<Decimal, String>`: The sum of amounts, or zero if no records match.
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

    /// Finds all order records within the specified bill date range.
    ///
    /// # Arguments
    ///
    /// - `NaiveDate`: The start date of the range.
    /// - `NaiveDate`: The end date of the range.
    ///
    /// # Returns
    ///
    /// - `Result<Vec<OrderRecordModel>, String>`: The list of records in the date range.
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

    /// Finds all order records within the specified bill date range and transaction type.
    ///
    /// # Arguments
    ///
    /// - `NaiveDate`: The start date of the range.
    /// - `NaiveDate`: The end date of the range.
    /// - `String`: The transaction type to filter by.
    ///
    /// # Returns
    ///
    /// - `Result<Vec<OrderRecordModel>, String>`: The filtered list of records.
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

    /// Finds all order records for a specific bill date.
    ///
    /// # Arguments
    ///
    /// - `NaiveDate`: The bill date to filter by.
    ///
    /// # Returns
    ///
    /// - `Result<Vec<OrderRecordModel>, String>`: The list of records for the specified date.
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

    /// Retrieves all order records from the database.
    ///
    /// # Returns
    ///
    /// - `Result<Vec<OrderRecordModel>, String>`: The complete list of order records.
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

/// Database access methods for `RecordImageRepository`.
impl RecordImageRepository {
    /// Finds a record image by its unique identifier.
    ///
    /// # Arguments
    ///
    /// - `i32`: The image identifier.
    ///
    /// # Returns
    ///
    /// - `Result<Option<OrderRecordImageModel>, String>`: The image model if found, or `None`.
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

    /// Finds all images associated with the given record identifier.
    ///
    /// # Arguments
    ///
    /// - `i32`: The record identifier.
    ///
    /// # Returns
    ///
    /// - `Result<Vec<OrderRecordImageModel>, String>`: The list of images for the record.
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

    /// Finds all images associated with the given record identifiers.
    ///
    /// # Arguments
    ///
    /// - `Vec<i32>`: The list of record identifiers.
    ///
    /// # Returns
    ///
    /// - `Result<Vec<OrderRecordImageModel>, String>`: The list of images for the specified records.
    #[instrument_trace]
    pub async fn find_by_record_ids(
        record_ids: Vec<i32>,
    ) -> Result<Vec<OrderRecordImageModel>, String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let result: Vec<OrderRecordImageModel> = OrderRecordImageEntity::find()
            .filter(OrderRecordImageColumn::RecordId.is_in(record_ids))
            .order_by_desc(OrderRecordImageColumn::Id)
            .all(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(result)
    }

    /// Inserts a new record image into the database.
    ///
    /// # Arguments
    ///
    /// - `OrderRecordImageActiveModel`: The active model containing the image data to insert.
    ///
    /// # Returns
    ///
    /// - `Result<OrderRecordImageModel, String>`: The inserted image model.
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

    /// Inserts a new record image within an existing database transaction.
    ///
    /// # Arguments
    ///
    /// - `&DatabaseTransaction`: The active database transaction.
    /// - `OrderRecordImageActiveModel`: The active model containing the image data to insert.
    ///
    /// # Returns
    ///
    /// - `Result<OrderRecordImageModel, String>`: The inserted image model.
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

    /// Finds all images belonging to the specified user identifiers.
    ///
    /// # Arguments
    ///
    /// - `Vec<i32>`: The list of user identifiers.
    ///
    /// # Returns
    ///
    /// - `Result<Vec<OrderRecordImageModel>, String>`: The list of images for the specified users.
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

    /// Updates an existing record image in the database.
    ///
    /// # Arguments
    ///
    /// - `OrderRecordImageActiveModel`: The active model containing the updated image data.
    ///
    /// # Returns
    ///
    /// - `Result<OrderRecordImageModel, String>`: The updated image model.
    #[instrument_trace]
    pub async fn update(
        active_model: OrderRecordImageActiveModel,
    ) -> Result<OrderRecordImageModel, String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let result: OrderRecordImageModel = active_model
            .update(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(result)
    }
}
