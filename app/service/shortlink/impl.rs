use super::*;

impl ShortlinkService {
    pub async fn insert_shortlink(request: ShortlinkInsertRequest) -> Result<i32, String> {
        if request.url.is_empty() {
            return Err("URL cannot be empty".to_string());
        }
        if !request.url.starts_with("http://") && !request.url.starts_with("https://") {
            return Err("URL must start with http:// or https://".to_string());
        }
        let db: DatabaseConnection = get_postgresql_connection().await?;
        let active_model: ActiveModel = ActiveModel {
            url: ActiveValue::Set(request.url),
            user_cookie: ActiveValue::Set(request.user_cookie),
            id: ActiveValue::NotSet,
            created_at: ActiveValue::NotSet,
        };
        let result = active_model
            .insert(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        Ok(result.id)
    }

    pub async fn query_shortlink(id: i32) -> Result<Option<ShortlinkRecord>, String> {
        let db: DatabaseConnection = get_postgresql_connection().await?;
        let record: Option<Model> = Entity::find_by_id(id)
            .one(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        match record {
            Some(model) => {
                let record = ShortlinkRecord {
                    id: model.id,
                    url: model.url,
                    user_cookie: model.user_cookie,
                    created_at: model
                        .created_at
                        .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
                        .unwrap_or_default(),
                };
                Ok(Some(record))
            }
            None => Ok(None),
        }
    }
}
