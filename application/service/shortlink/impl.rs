use super::*;

/// Implementation of methods for `ShortlinkService`.
impl ShortlinkService {
    /// Decodes an obfuscated shortlink ID back to its numeric value.
    ///
    /// # Arguments
    ///
    /// - `&str`: The encoded shortlink ID string.
    ///
    /// # Returns
    ///
    /// - `Result<i32, String>`: The decoded numeric ID, or an error if the format is invalid.
    #[instrument_trace]
    fn decrypt_id(encoded_id: &str) -> Result<i32, String> {
        let decoded: String = hyperlane_utils::Decode::execute(CHARSETS, encoded_id)
            .map_err(|_: DecodeError| ERROR_INVALID_SHORTLINK_ID_FORMAT.to_string())?;
        decoded
            .parse::<i32>()
            .map_err(|_: std::num::ParseIntError| ERROR_INVALID_SHORTLINK_ID_FORMAT.to_string())
    }

    /// Encodes a numeric ID into an obfuscated string for use in short URLs.
    ///
    /// # Arguments
    ///
    /// - `i32`: The numeric ID to encode.
    ///
    /// # Returns
    ///
    /// - `Result<String, String>`: The encoded string, or an error if encoding fails.
    #[instrument_trace]
    fn encrypt_id(id: i32) -> Result<String, String> {
        hyperlane_utils::Encode::execute(CHARSETS, &id.to_string())
            .map_err(|_: EncodeError| ERROR_FAILED_TO_ENCRYPT_SHORTLINK_ID.to_string())
    }

    /// Inserts a new shortlink or returns the existing one if the URL already exists.
    ///
    /// # Arguments
    ///
    /// - `ShortlinkInsertRequest`: The request containing the target URL.
    ///
    /// # Returns
    ///
    /// - `Result<String, String>`: The encoded shortlink ID, or an error if the URL is empty.
    #[instrument_trace]
    pub async fn insert_shortlink(request: ShortlinkInsertRequest) -> Result<String, String> {
        if request.get_url().is_empty() {
            return Err(ERROR_URL_CANNOT_BE_EMPTY.to_string());
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

    /// Queries a shortlink record by its encoded ID and returns the decoded record.
    ///
    /// # Arguments
    ///
    /// - `String`: The encoded shortlink ID string.
    ///
    /// # Returns
    ///
    /// - `Result<Option<ShortlinkRecord>, String>`: The shortlink record if found, or `None`.
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
                            .map(|dt: NaiveDateTime| dt.format("%Y-%m-%d %H:%M:%S").to_string())
                            .unwrap_or_default(),
                    );
                Ok(Some(record))
            }
            None => Ok(None),
        }
    }
}
