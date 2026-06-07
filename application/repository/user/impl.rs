use super::*;

/// Database access methods for `UserRepository`.
impl UserRepository {
    /// Finds a user by their unique identifier.
    ///
    /// # Arguments
    ///
    /// - `i32`: The user identifier.
    ///
    /// # Returns
    ///
    /// - `Result<Option<UserModel>, String>`: The user model if found, or `None`.
    #[instrument_trace]
    pub async fn find_by_id(user_id: i32) -> Result<Option<UserModel>, String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let result: Option<UserModel> = UserEntity::find_by_id(user_id)
            .one(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(result)
    }

    /// Finds a user by their username.
    ///
    /// # Arguments
    ///
    /// - `String`: The username to search for.
    ///
    /// # Returns
    ///
    /// - `Result<Option<UserModel>, String>`: The user model if found, or `None`.
    #[instrument_trace]
    pub async fn find_by_username(username: String) -> Result<Option<UserModel>, String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let result: Option<UserModel> = UserEntity::find()
            .filter(UserColumn::Username.eq(username))
            .one(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(result)
    }

    /// Finds multiple users by their identifiers.
    ///
    /// # Arguments
    ///
    /// - `Vec<i32>`: The list of user identifiers.
    ///
    /// # Returns
    ///
    /// - `Result<Vec<UserModel>, String>`: The list of found user models.
    #[instrument_trace]
    pub async fn find_by_ids(user_ids: Vec<i32>) -> Result<Vec<UserModel>, String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let result: Vec<UserModel> = UserEntity::find()
            .filter(UserColumn::Id.is_in(user_ids))
            .all(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(result)
    }

    /// Inserts a new user record into the database.
    ///
    /// # Arguments
    ///
    /// - `UserActiveModel`: The active model containing the user data to insert.
    ///
    /// # Returns
    ///
    /// - `Result<UserModel, String>`: The inserted user model with generated fields.
    #[instrument_trace]
    pub async fn insert(active_model: UserActiveModel) -> Result<UserModel, String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let result: UserModel = active_model
            .insert(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(result)
    }

    /// Updates an existing user record in the database.
    ///
    /// # Arguments
    ///
    /// - `UserActiveModel`: The active model containing the updated user data.
    ///
    /// # Returns
    ///
    /// - `Result<UserModel, String>`: The updated user model.
    #[instrument_trace]
    pub async fn update(active_model: UserActiveModel) -> Result<UserModel, String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let result: UserModel = active_model
            .update(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(result)
    }

    /// Queries users with keyword search and cursor-based pagination.
    ///
    /// # Arguments
    ///
    /// - `Option<String>`: Optional keyword to search across username, email, phone, and ID.
    /// - `Option<i32>`: Optional last ID for cursor-based pagination.
    /// - `u64`: The page size limit.
    ///
    /// # Returns
    ///
    /// - `Result<(Vec<UserModel>, i64, bool), String>`: The users, total count, and has-more flag.
    #[instrument_trace]
    pub async fn query_with_pagination(
        keyword: Option<String>,
        last_id: Option<i32>,
        limit: u64,
    ) -> Result<(Vec<UserModel>, i64, bool), String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let mut base_select: Select<UserEntity> = UserEntity::find();
        if let Some(keyword) = keyword {
            let keyword_pattern: String = format!("%{keyword}%");
            let mut condition: Condition = Condition::any()
                .add(UserColumn::Username.like(keyword_pattern.clone()))
                .add(UserColumn::Email.like(keyword_pattern.clone()))
                .add(UserColumn::Phone.like(keyword_pattern.clone()));
            if let Ok(user_id) = keyword.parse::<i32>() {
                condition = condition.add(UserColumn::Id.eq(user_id));
            }
            base_select = base_select.filter(condition);
        }
        let total_count_u64: u64 = base_select
            .clone()
            .count(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        let total_count: i64 = total_count_u64 as i64;
        let mut paged_select: Select<UserEntity> = base_select;
        if let Some(last_id) = last_id {
            paged_select = paged_select.filter(UserColumn::Id.lt(last_id));
        }
        paged_select = paged_select.order_by_desc(UserColumn::Id);
        let limit_with_extra: u64 = limit + 1;
        paged_select = paged_select.limit(limit_with_extra);
        let paged_users: Vec<UserModel> = paged_select
            .all(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        let has_more: bool = paged_users.len() > limit as usize;
        let paged_users: Vec<UserModel> = paged_users.into_iter().take(limit as usize).collect();
        Ok((paged_users, total_count, has_more))
    }

    /// Counts the number of users created within the specified date range.
    ///
    /// # Arguments
    ///
    /// - `NaiveDateTime`: The start of the date range.
    /// - `NaiveDateTime`: The end of the date range.
    ///
    /// # Returns
    ///
    /// - `Result<i64, String>`: The count of users created in the range.
    #[instrument_trace]
    pub async fn count_by_created_at_range(
        start: NaiveDateTime,
        end: NaiveDateTime,
    ) -> Result<i64, String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let count_u64: u64 = UserEntity::find()
            .filter(UserColumn::CreatedAt.gte(start))
            .filter(UserColumn::CreatedAt.lte(end))
            .count(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(count_u64 as i64)
    }

    /// Finds all users created within the specified date range.
    ///
    /// # Arguments
    ///
    /// - `NaiveDateTime`: The start of the date range.
    /// - `NaiveDateTime`: The end of the date range.
    ///
    /// # Returns
    ///
    /// - `Result<Vec<UserModel>, String>`: The list of users created in the range.
    #[instrument_trace]
    pub async fn find_by_created_at_range(
        start: NaiveDateTime,
        end: NaiveDateTime,
    ) -> Result<Vec<UserModel>, String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let result: Vec<UserModel> = UserEntity::find()
            .filter(UserColumn::CreatedAt.gte(start))
            .filter(UserColumn::CreatedAt.lte(end))
            .all(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(result)
    }

    /// Deletes a user by their unique identifier.
    ///
    /// # Arguments
    ///
    /// - `i32`: The user identifier to delete.
    ///
    /// # Returns
    ///
    /// - `Result<(), String>`: Ok on success, or an error message.
    #[instrument_trace]
    pub async fn delete_by_id(user_id: i32) -> Result<(), String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        UserEntity::delete_by_id(user_id)
            .exec(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(())
    }
}
