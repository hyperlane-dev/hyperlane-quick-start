use super::*;

/// Database access and validation methods for `UserRepository` (auth module).
impl UserRepository {
    /// Validates whether the given string matches the email regex pattern.
    ///
    /// # Arguments
    ///
    /// - `&str`: The email string to validate.
    ///
    /// # Returns
    ///
    /// - `bool`: `true` if the email is valid, `false` otherwise.
    #[instrument_trace]
    fn validate_email(email: &str) -> bool {
        match EMAIL_REGEX.as_ref() {
            Some(regex) => regex.is_match(email),
            None => false,
        }
    }

    /// Validates whether the given string matches the phone regex pattern.
    ///
    /// # Arguments
    ///
    /// - `&str`: The phone string to validate.
    ///
    /// # Returns
    ///
    /// - `bool`: `true` if the phone is valid, `false` otherwise.
    #[instrument_trace]
    fn validate_phone(phone: &str) -> bool {
        match PHONE_REGEX_OPT.as_ref() {
            Some(regex) => regex.is_match(phone),
            None => false,
        }
    }

    /// Validates the email and phone fields of an `AuthUserActiveModel`.
    ///
    /// # Arguments
    ///
    /// - `&AuthUserActiveModel`: The active model to validate.
    ///
    /// # Returns
    ///
    /// - `Result<(), String>`: Ok if valid, or an error message describing the invalid field.
    #[instrument_trace]
    fn validate_active_model(active_model: &AuthUserActiveModel) -> Result<(), String> {
        if let ActiveValue::Set(Some(ref email)) = active_model.email
            && !email.is_empty()
            && !Self::validate_email(email)
        {
            return Err("Invalid email format".to_string());
        }
        if let ActiveValue::Set(Some(ref phone)) = active_model.phone
            && !phone.is_empty()
            && !Self::validate_phone(phone)
        {
            return Err("Invalid phone format".to_string());
        }
        Ok(())
    }

    /// Finds an auth user by their unique identifier.
    ///
    /// # Arguments
    ///
    /// - `i32`: The user identifier.
    ///
    /// # Returns
    ///
    /// - `Result<Option<AuthUserModel>, String>`: The user model if found, or `None`.
    #[instrument_trace]
    pub async fn find_by_id(user_id: i32) -> Result<Option<AuthUserModel>, String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let result: Option<AuthUserModel> = AuthUserEntity::find_by_id(user_id)
            .one(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(result)
    }

    /// Finds an auth user by their username.
    ///
    /// # Arguments
    ///
    /// - `String`: The username to search for.
    ///
    /// # Returns
    ///
    /// - `Result<Option<AuthUserModel>, String>`: The user model if found, or `None`.
    #[instrument_trace]
    pub async fn find_by_username(username: String) -> Result<Option<AuthUserModel>, String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let result: Option<AuthUserModel> = AuthUserEntity::find()
            .filter(AuthUserColumn::Username.eq(username))
            .one(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(result)
    }

    /// Finds multiple auth users by their identifiers.
    ///
    /// # Arguments
    ///
    /// - `Vec<i32>`: The list of user identifiers.
    ///
    /// # Returns
    ///
    /// - `Result<Vec<AuthUserModel>, String>`: The list of found user models.
    #[instrument_trace]
    pub async fn find_by_ids(user_ids: Vec<i32>) -> Result<Vec<AuthUserModel>, String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let result: Vec<AuthUserModel> = AuthUserEntity::find()
            .filter(AuthUserColumn::Id.is_in(user_ids))
            .all(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(result)
    }

    /// Inserts a new auth user after validating email and phone format.
    ///
    /// # Arguments
    ///
    /// - `AuthUserActiveModel`: The active model containing the user data to insert.
    ///
    /// # Returns
    ///
    /// - `Result<AuthUserModel, String>`: The inserted user model with generated fields.
    #[instrument_trace]
    pub async fn insert(active_model: AuthUserActiveModel) -> Result<AuthUserModel, String> {
        Self::validate_active_model(&active_model)?;
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let result: AuthUserModel = active_model
            .insert(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(result)
    }

    /// Updates an existing auth user after validating email and phone format.
    ///
    /// # Arguments
    ///
    /// - `AuthUserActiveModel`: The active model containing the updated user data.
    ///
    /// # Returns
    ///
    /// - `Result<AuthUserModel, String>`: The updated user model.
    #[instrument_trace]
    pub async fn update(active_model: AuthUserActiveModel) -> Result<AuthUserModel, String> {
        Self::validate_active_model(&active_model)?;
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let result: AuthUserModel = active_model
            .update(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(result)
    }

    /// Queries auth users with keyword search and cursor-based pagination.
    ///
    /// # Arguments
    ///
    /// - `Option<String>`: Optional keyword to search across username, email, phone, and ID.
    /// - `Option<i32>`: Optional last ID for cursor-based pagination.
    /// - `u64`: The page size limit.
    ///
    /// # Returns
    ///
    /// - `Result<(Vec<AuthUserModel>, i64, bool), String>`: The users, total count, and has-more flag.
    #[instrument_trace]
    pub async fn query_with_pagination(
        keyword: Option<String>,
        last_id: Option<i32>,
        limit: u64,
    ) -> Result<(Vec<AuthUserModel>, i64, bool), String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let mut base_select: Select<AuthUserEntity> = AuthUserEntity::find();
        if let Some(keyword) = keyword {
            let keyword_pattern: String = format!("%{keyword}%");
            let mut condition: Condition = Condition::any()
                .add(AuthUserColumn::Username.like(keyword_pattern.clone()))
                .add(AuthUserColumn::Email.like(keyword_pattern.clone()))
                .add(AuthUserColumn::Phone.like(keyword_pattern.clone()));
            if let Ok(user_id) = keyword.parse::<i32>() {
                condition = condition.add(AuthUserColumn::Id.eq(user_id));
            }
            base_select = base_select.filter(condition);
        }
        let total_count_u64: u64 = base_select
            .clone()
            .count(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        let total_count: i64 = total_count_u64 as i64;
        let mut paged_select: Select<AuthUserEntity> = base_select;
        if let Some(last_id) = last_id {
            paged_select = paged_select.filter(AuthUserColumn::Id.lt(last_id));
        }
        paged_select = paged_select.order_by_desc(AuthUserColumn::Id);
        let limit_with_extra: u64 = limit + 1;
        paged_select = paged_select.limit(limit_with_extra);
        let paged_users: Vec<AuthUserModel> = paged_select
            .all(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        let has_more: bool = paged_users.len() > limit as usize;
        let paged_users: Vec<AuthUserModel> =
            paged_users.into_iter().take(limit as usize).collect();
        Ok((paged_users, total_count, has_more))
    }

    /// Counts the number of auth users created within the specified date range.
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
        let count_u64: u64 = AuthUserEntity::find()
            .filter(AuthUserColumn::CreatedAt.gte(start))
            .filter(AuthUserColumn::CreatedAt.lte(end))
            .count(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(count_u64 as i64)
    }

    /// Finds all auth users created within the specified date range.
    ///
    /// # Arguments
    ///
    /// - `NaiveDateTime`: The start of the date range.
    /// - `NaiveDateTime`: The end of the date range.
    ///
    /// # Returns
    ///
    /// - `Result<Vec<AuthUserModel>, String>`: The list of users created in the range.
    #[instrument_trace]
    pub async fn find_by_created_at_range(
        start: NaiveDateTime,
        end: NaiveDateTime,
    ) -> Result<Vec<AuthUserModel>, String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let result: Vec<AuthUserModel> = AuthUserEntity::find()
            .filter(AuthUserColumn::CreatedAt.gte(start))
            .filter(AuthUserColumn::CreatedAt.lte(end))
            .all(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(result)
    }

    /// Deletes an auth user by their unique identifier.
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
        AuthUserEntity::delete_by_id(user_id)
            .exec(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(())
    }
}
