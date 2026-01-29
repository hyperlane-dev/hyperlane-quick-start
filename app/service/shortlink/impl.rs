use super::*;

impl ShortlinkService {
    #[instrument_trace]
    fn decrypt_id(encoded_id: &str) -> Result<i32, String> {
        let decoded: String = hyperlane_utils::Decode::execute(CHARSETS, encoded_id)
            .map_err(|_| "Invalid shortlink ID format".to_string())?;
        decoded
            .parse::<i32>()
            .map_err(|_| "Invalid shortlink ID format".to_string())
    }

    #[instrument_trace]
    fn encrypt_id(id: i32) -> Result<String, String> {
        hyperlane_utils::Encode::execute(CHARSETS, &id.to_string())
            .map_err(|_| "Failed to encrypt shortlink ID".to_string())
    }

    #[instrument_trace]
    pub async fn insert_shortlink(request: ShortlinkInsertRequest) -> Result<String, String> {
        if request.url.is_empty() {
            return Err("URL cannot be empty".to_string());
        }
        let db: DatabaseConnection =
            get_postgresql_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let existing_record: Option<Model> = Entity::find()
            .filter(Column::Url.eq(&request.url))
            .one(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        let record_id: i32 = if let Some(record) = existing_record {
            record.id
        } else {
            let active_model: ActiveModel = ActiveModel {
                url: ActiveValue::Set(request.url),
                id: ActiveValue::NotSet,
                created_at: ActiveValue::NotSet,
            };
            let result = active_model
                .insert(&db)
                .await
                .map_err(|error: DbErr| error.to_string())?;
            result.id
        };
        Self::encrypt_id(record_id)
    }

    #[instrument_trace]
    pub async fn query_shortlink(encrypted_id: String) -> Result<Option<ShortlinkRecord>, String> {
        let id: i32 = Self::decrypt_id(&encrypted_id)?;
        let db: DatabaseConnection =
            get_postgresql_connection(DEFAULT_POSTGRESQL_INSTANCE_NAME, None).await?;
        let record: Option<Model> = Entity::find_by_id(id)
            .one(&db)
            .await
            .map_err(|error: DbErr| error.to_string())?;
        match record {
            Some(model) => {
                let record = ShortlinkRecord {
                    id: model.id,
                    url: model.url,
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
