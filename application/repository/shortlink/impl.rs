use super::*;

impl ShortlinkRepository {
    #[instrument_trace]
    pub async fn find_by_url(url: &str) -> Result<Option<ShortlinkModel>, String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let result: Option<ShortlinkModel> = Entity::find()
            .filter(Column::Url.eq(url))
            .one(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(result)
    }

    #[instrument_trace]
    pub async fn find_by_id(id: i32) -> Result<Option<ShortlinkModel>, String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let result: Option<ShortlinkModel> = Entity::find_by_id(id)
            .one(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(result)
    }

    #[instrument_trace]
    pub async fn insert(url: &str) -> Result<ShortlinkModel, String> {
        let db: DatabaseConnection =
            PostgreSqlPlugin::get_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let active_model: ActiveModel = ActiveModel {
            url: ActiveValue::Set(url.to_string()),
            id: ActiveValue::NotSet,
            created_at: ActiveValue::NotSet,
        };
        let result: ShortlinkModel = active_model
            .insert(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(result)
    }
}
