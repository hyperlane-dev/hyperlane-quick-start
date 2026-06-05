use super::*;

impl GithubPagesRepository {
    #[instrument_trace]
    pub async fn find_all() -> Result<Vec<GithubPagesModel>, String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let results: Vec<GithubPagesModel> = Entity::find()
            .all(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(results)
    }

    #[instrument_trace]
    pub async fn find_by_owner_and_repository(
        owner: &str,
        repository: &str,
    ) -> Result<Option<GithubPagesModel>, String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let result: Option<GithubPagesModel> = Entity::find()
            .filter(Column::Owner.eq(owner))
            .filter(Column::Repository.eq(repository))
            .one(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(result)
    }

    #[instrument_trace]
    pub async fn insert(
        owner: &str,
        repository: &str,
        base_url: &str,
    ) -> Result<GithubPagesModel, String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let active_model: ActiveModel = ActiveModel {
            owner: ActiveValue::Set(owner.to_string()),
            repository: ActiveValue::Set(repository.to_string()),
            base_url: ActiveValue::Set(base_url.to_string()),
            id: ActiveValue::NotSet,
            last_synced_at: ActiveValue::NotSet,
            created_at: ActiveValue::NotSet,
        };
        let result: GithubPagesModel = active_model
            .insert(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(result)
    }

    #[instrument_trace]
    pub async fn update_last_synced_at(id: i32) -> Result<(), String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let active_model: ActiveModel = ActiveModel {
            id: ActiveValue::Set(id),
            last_synced_at: ActiveValue::Set(Some(Utc::now().naive_utc())),
            ..Default::default()
        };
        active_model
            .update(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(())
    }

    #[instrument_trace]
    pub async fn delete_by_id(id: i32) -> Result<(), String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        Entity::delete_by_id(id)
            .exec(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(())
    }
}
