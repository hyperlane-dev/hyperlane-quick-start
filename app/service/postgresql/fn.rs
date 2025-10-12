use super::*;

pub async fn create_postgresql_record(record: PostgresqlRecord) -> Result<(), String> {
    let db: DatabaseConnection = get_postgresql_connection().await?;
    let active_model: ActiveModel = ActiveModel {
        key: ActiveValue::Set(record.key),
        value: ActiveValue::Set(record.value),
        id: ActiveValue::NotSet,
    };
    active_model
        .insert(&db)
        .await
        .map_err(|error: DbErr| error.to_string())?;
    Ok(())
}

pub async fn get_all_postgresql_records() -> Result<Vec<PostgresqlRecord>, String> {
    let db: DatabaseConnection = get_postgresql_connection().await?;
    let records: Vec<Model> = Entity::find()
        .all(&db)
        .await
        .map_err(|error: DbErr| error.to_string())?;
    let result: Vec<PostgresqlRecord> = records
        .into_iter()
        .map(|r: Model| PostgresqlRecord {
            key: r.key,
            value: r.value,
        })
        .collect();
    Ok(result)
}

pub async fn update_postgresql_record(record: PostgresqlRecord) -> Result<(), String> {
    let db: DatabaseConnection = get_postgresql_connection().await?;
    Entity::update_many()
        .filter(Column::Key.eq(&record.key))
        .col_expr(Column::Value, Expr::value(record.value))
        .exec(&db)
        .await
        .map_err(|error: DbErr| error.to_string())?;
    Ok(())
}

pub async fn delete_postgresql_record(key: &str) -> Result<(), String> {
    let db: DatabaseConnection = get_postgresql_connection().await?;
    Entity::delete_many()
        .filter(Column::Key.eq(key))
        .exec(&db)
        .await
        .map_err(|error: DbErr| error.to_string())?;
    Ok(())
}
