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
        if request.get_url().is_empty() {
            return Err("URL cannot be empty".to_string());
        }
        let existing_record: Option<ShortlinkModel> =
            ShortlinkRepository::find_by_url(request.get_url()).await?;
        let record_id: i32 = if let Some(record) = existing_record {
            record.get_id()
        } else {
            let result: ShortlinkModel = ShortlinkRepository::insert(request.get_url()).await?;
            result.get_id()
        };
        Self::encrypt_id(record_id)
    }

    #[instrument_trace]
    pub async fn query_shortlink(encrypted_id: String) -> Result<Option<ShortlinkRecord>, String> {
        let id: i32 = Self::decrypt_id(&encrypted_id)?;
        match ShortlinkRepository::find_by_id(id).await? {
            Some(model) => {
                let mut record: ShortlinkRecord = ShortlinkRecord::default();
                record
                    .set_id(model.get_id())
                    .set_url(model.get_url().clone())
                    .set_created_at(
                        model
                            .try_get_created_at()
                            .map(|dt: DateTime<FixedOffset>| {
                                dt.format("%Y-%m-%d %H:%M:%S").to_string()
                            })
                            .unwrap_or_default(),
                    );
                Ok(Some(record))
            }
            None => Ok(None),
        }
    }
}
