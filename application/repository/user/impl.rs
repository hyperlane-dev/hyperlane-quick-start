use super::*;

impl UserRepository {
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
