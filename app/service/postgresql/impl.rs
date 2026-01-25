use super::*;

impl PostgresqlService {
    #[instrument_trace]
    pub async fn create_postgresql_record(record: PostgresqlRecord) -> Result<(), String> {
        let db: DatabaseConnection =
            get_postgresql_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME).await?;
        let active_model: ActiveModel = ActiveModel {
            key: ActiveValue::Set(record.get_key().clone()),
            value: ActiveValue::Set(record.get_value().clone()),
            id: ActiveValue::NotSet,
        };
        active_model
            .insert(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(())
    }

    #[instrument_trace]
    pub async fn get_all_postgresql_records() -> Result<Vec<PostgresqlRecord>, String> {
        let db: DatabaseConnection =
            get_postgresql_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME).await?;
        let records: Vec<Model> = Entity::find()
            .all(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        let result: Vec<PostgresqlRecord> = records
            .into_iter()
            .map(|r: Model| {
                let mut record = PostgresqlRecord::default();
                record.set_key(r.key).set_value(r.value);
                record
            })
            .collect();
        Ok(result)
    }

    #[instrument_trace]
    pub async fn update_postgresql_record(record: PostgresqlRecord) -> Result<(), String> {
        let db: DatabaseConnection =
            get_postgresql_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME).await?;
        Entity::update_many()
            .filter(Column::Key.eq(record.get_key()))
            .col_expr(Column::Value, Expr::value(record.get_value().clone()))
            .exec(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(())
    }

    #[instrument_trace]
    pub async fn delete_postgresql_record(key: &str) -> Result<(), String> {
        let db: DatabaseConnection =
            get_postgresql_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME).await?;
        Entity::delete_many()
            .filter(Column::Key.eq(key))
            .exec(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(())
    }
}
