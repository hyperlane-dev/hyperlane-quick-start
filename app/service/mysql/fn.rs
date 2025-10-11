use super::*;

async fn get_mysql_connection() -> Result<DatabaseConnection, String> {
    let env: &'static EnvConfig = get_global_env_config();
    let db_url: String = format!(
        "mysql://{}:{}@{}:{}/{}",
        env.mysql_username, env.mysql_password, env.mysql_host, env.mysql_port, env.mysql_database
    );
    Database::connect(&db_url)
        .await
        .map_err(|error| error.to_string())
}

pub async fn create_mysql_record(record: MysqlRecord) -> Result<(), String> {
    let db: DatabaseConnection = get_mysql_connection().await?;
    let active_model: ActiveModel = ActiveModel {
        key: sea_orm::ActiveValue::Set(record.key),
        value: sea_orm::ActiveValue::Set(record.value),
        id: sea_orm::ActiveValue::NotSet,
    };
    active_model
        .insert(&db)
        .await
        .map_err(|error| error.to_string())?;
    Ok(())
}

pub async fn get_all_mysql_records() -> Result<Vec<MysqlRecord>, String> {
    let db: DatabaseConnection = get_mysql_connection().await?;
    let records: Vec<Model> = Entity::find()
        .all(&db)
        .await
        .map_err(|error| error.to_string())?;
    Ok(records
        .into_iter()
        .map(|r| MysqlRecord {
            key: r.key,
            value: r.value,
        })
        .collect())
}

pub async fn update_mysql_record(record: MysqlRecord) -> Result<(), String> {
    let db: DatabaseConnection = get_mysql_connection().await?;
    Entity::update_many()
        .filter(Column::Key.eq(&record.key))
        .col_expr(Column::Value, Expr::value(record.value))
        .exec(&db)
        .await
        .map_err(|error| error.to_string())?;
    Ok(())
}

pub async fn delete_mysql_record(key: &str) -> Result<(), String> {
    let db: DatabaseConnection = get_mysql_connection().await?;
    Entity::delete_many()
        .filter(Column::Key.eq(key))
        .exec(&db)
        .await
        .map_err(|error| error.to_string())?;
    Ok(())
}
