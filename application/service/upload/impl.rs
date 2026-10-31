use super::*;

/// Implementation of methods for `UploadService`.
impl UploadService {
    /// Generates a date-time based directory path for file storage (year/month/day/hour/minute).
    ///
    /// # Returns
    ///
    /// - `String` - The formatted directory path string.
    #[instrument_trace]
    pub fn get_base_file_dir() -> String {
        let (year, month, day, hour, minute, _, _, _) = calculate_time();
        let full_dir: String = format!("{year}/{month}/{day}/{hour}/{minute}");
        full_dir
    }

    /// Reads the parts of a multipart form data body.
    ///
    /// The parts are decoded by the framework itself, so the file name, the
    /// media type and the raw content of every part come from the request
    /// rather than from a hand written parser.
    ///
    /// # Arguments
    ///
    /// - `&mut Context` - The request context holding the request.
    ///
    /// # Returns
    ///
    /// - `Option<MultipartFormDataParts>` - The decoded parts, or `None` when the body carries no multipart form data.
    #[instrument_trace]
    async fn get_request_parts(ctx: &mut Context) -> Option<MultipartFormDataParts> {
        if ctx.get_request().try_get_content_type().as_deref() != Some(UPLOAD_FORM_CONTENT_TYPE) {
            Self::set_common_error_response_body(ctx, ERROR_INVALID_MULTIPART.to_string()).await;
            return None;
        }
        let parts: Option<MultipartFormDataParts> = ctx.get_request().try_get_body_form_data();
        if parts.is_none() {
            Self::set_common_error_response_body(ctx, ERROR_INVALID_MULTIPART.to_string()).await;
        }
        parts
    }

    /// Reads the file name of the uploaded file part.
    ///
    /// The name is taken from the part itself, so it is neither URL encoded
    /// nor URL decoded on the way.
    ///
    /// # Arguments
    ///
    /// - `&[BodyPart]` - The parts of the multipart form data body.
    ///
    /// # Returns
    ///
    /// - `Option<String>` - The file name of the uploaded file, or `None` when the body carries no file part.
    #[instrument_trace]
    fn get_part_file_name(parts: &[BodyPart]) -> Option<String> {
        let part: &BodyPart = MultipartFormData::try_get_part(parts, PART_FILE)?;
        part.try_get_file_name().cloned()
    }

    /// Reads the raw bytes of the uploaded file part.
    ///
    /// The bytes are the content of the file part, so a chunk of an upload
    /// is read through the file object of the framework rather than from the
    /// raw body of the request.
    ///
    /// # Arguments
    ///
    /// - `&[BodyPart]` - The parts of the multipart form data body.
    ///
    /// # Returns
    ///
    /// - `Option<RequestBody>` - The raw bytes of the file part, or `None` when the body carries no file part.
    #[instrument_trace]
    fn get_file_part_data(parts: &[BodyPart]) -> Option<RequestBody> {
        let part: &BodyPart = MultipartFormData::try_get_part(parts, PART_FILE)?;
        part.try_get_file_name()?;
        let data: &[u8] = part.try_get_data().unwrap_or_default();
        Some(data.to_vec())
    }

    /// Validates that a file ID is present.
    ///
    /// # Arguments
    ///
    /// - `Option<String>` - The optional file ID from the request body.
    ///
    /// # Returns
    ///
    /// - `Result<String, String>` - The file ID if present, or the error message.
    #[instrument_trace]
    fn validate_file_id(file_id_opt: Option<String>) -> Result<String, String> {
        match file_id_opt {
            Some(id)
                if !id.is_empty()
                    && id.chars().all(|value: char| {
                        value.is_ascii_alphanumeric() || value == '-' || value == '_'
                    }) =>
            {
                Ok(id)
            }
            Some(_) => Err(ERROR_INVALID_FILE_ID.to_string()),
            None => Err(ERROR_MISSING_FILE_ID.to_string()),
        }
    }

    /// Validates that the total chunks field is present and parseable.
    ///
    /// # Arguments
    ///
    /// - `Option<String>` - The optional total chunks value from the request body.
    ///
    /// # Returns
    ///
    /// - `Result<usize, String>` - The parsed total chunks count, or the error message.
    #[instrument_trace]
    fn validate_total_chunks(total_chunks_opt: Option<String>) -> Result<usize, String> {
        match total_chunks_opt {
            Some(total) => match total.parse::<usize>() {
                Ok(count) if count > 0 => Ok(count),
                _ => Err(ERROR_INVALID_TOTAL_CHUNKS.to_string()),
            },
            None => Err(ERROR_MISSING_TOTAL_CHUNKS.to_string()),
        }
    }

    /// Validates that the file name of the uploaded file is present.
    ///
    /// # Arguments
    ///
    /// - `Option<String>` - The optional file name of the uploaded file part.
    ///
    /// # Returns
    ///
    /// - `Result<String, String>` - The file name, or the error message.
    #[instrument_trace]
    fn validate_file_name(file_name_opt: Option<String>) -> Result<String, String> {
        match file_name_opt {
            Some(name)
                if !name.is_empty()
                    && name != "."
                    && name != ".."
                    && !name.contains(['/', '\\', '\0']) =>
            {
                Ok(name)
            }
            Some(_) => Err(ERROR_INVALID_DIRECTORY_OR_FILE_NAME.to_string()),
            None => Err(ERROR_MISSING_FILE_NAME.to_string()),
        }
    }

    /// Validates the index of the chunk stored by a request.
    ///
    /// # Arguments
    ///
    /// - `Option<String>` - The optional chunk index value from the request body.
    ///
    /// # Returns
    ///
    /// - `Result<usize, String>` - The parsed chunk index, or the error message.
    #[instrument_trace]
    fn validate_chunk_index(chunk_index_opt: Option<String>) -> Result<usize, String> {
        match chunk_index_opt {
            Some(index) => match index.parse::<usize>() {
                Ok(value) => Ok(value),
                Err(_) => Err(ERROR_INVALID_CHUNK_INDEX.to_string()),
            },
            None => Err(ERROR_MISSING_CHUNK_INDEX.to_string()),
        }
    }

    /// Validates a directory field, falling back to the base file dir if invalid or missing.
    ///
    /// # Arguments
    ///
    /// - `Option<String>` - The optional directory from the request body.
    ///
    /// # Returns
    ///
    /// - `String` - The directory path, or the default base file dir.
    #[instrument_trace]
    fn validate_directory(base_file_dir_opt: Option<String>) -> String {
        match base_file_dir_opt {
            Some(dir) => {
                if Self::is_valid_directory_path(&dir) {
                    dir
                } else {
                    Self::get_base_file_dir()
                }
            }
            None => Self::get_base_file_dir(),
        }
    }

    /// Checks if a directory path is valid (non-empty, no path traversal, digits and slashes only).
    ///
    /// # Arguments
    ///
    /// - `&str` - The directory path to validate.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if the path is valid, `false` otherwise.
    #[instrument_trace]
    fn is_valid_directory_path(path: &str) -> bool {
        !path.is_empty()
            && path.split('/').all(|component: &str| {
                !component.is_empty() && component.chars().all(|value: char| value.is_ascii_digit())
            })
    }

    /// Extracts and validates file chunk data from the multipart body of a chunk upload registration.
    ///
    /// The file name comes from the file part of the body, the session
    /// identifier and the number of chunks come from its text fields.
    ///
    /// # Arguments
    ///
    /// - `&mut Context` - The request context.
    ///
    /// # Returns
    ///
    /// - `Option<FileChunkData>` - The validated file chunk data, or `None` if validation fails.
    #[instrument_trace]
    pub async fn get_register_file_chunk_data(ctx: &mut Context) -> Option<FileChunkData> {
        let parts: MultipartFormDataParts = Self::get_request_parts(ctx).await?;
        let file_id: String = match Self::validate_file_id(MultipartFormData::try_get_part_text(
            &parts,
            FIELD_FILE_ID,
        )) {
            Ok(id) => id,
            Err(error) => {
                Self::set_common_error_response_body(ctx, error).await;
                return None;
            }
        };
        let total_chunks: usize = match Self::validate_total_chunks(
            MultipartFormData::try_get_part_text(&parts, FIELD_TOTAL_CHUNKS),
        ) {
            Ok(count) => count,
            Err(error) => {
                Self::set_common_error_response_body(ctx, error).await;
                return None;
            }
        };
        let file_name: String = match Self::validate_file_name(Self::get_part_file_name(&parts)) {
            Ok(name) => name,
            Err(error) => {
                Self::set_common_error_response_body(ctx, error).await;
                return None;
            }
        };
        let base_file_dir: String = Self::validate_directory(MultipartFormData::try_get_part_text(
            &parts,
            FIELD_DIRECTORY,
        ));
        let mut data: FileChunkData = FileChunkData::default();
        data.set_file_id(file_id)
            .set_file_name(file_name)
            .set_chunk_index(0)
            .set_total_chunks(total_chunks)
            .set_base_file_dir(base_file_dir);
        Some(data)
    }

    /// Extracts and validates file chunk data for saving a chunk, including the chunk index.
    ///
    /// The chunk index comes from a text field of the multipart body, and the
    /// raw bytes of the chunk come from the file part of the same body.
    ///
    /// # Arguments
    ///
    /// - `&mut Context` - The request context for setting error responses.
    ///
    /// # Returns
    ///
    /// - `Option<(FileChunkData, RequestBody)>` - The validated file chunk data with chunk index and the raw bytes of the chunk, or `None`.
    #[instrument_trace]
    pub async fn get_save_file_chunk_data(
        ctx: &mut Context,
    ) -> Option<(FileChunkData, RequestBody)> {
        let parts: MultipartFormDataParts = Self::get_request_parts(ctx).await?;
        let file_id: String = match Self::validate_file_id(MultipartFormData::try_get_part_text(
            &parts,
            FIELD_FILE_ID,
        )) {
            Ok(id) => id,
            Err(error) => {
                Self::set_common_error_response_body(ctx, error).await;
                return None;
            }
        };
        let mut data: FileChunkData = match Self::get_merge_file_chunk_data(file_id).await {
            Some(data) => data,
            None => {
                Self::set_common_error_response_body(ctx, ERROR_UPLOAD_NOT_REGISTERED.to_string())
                    .await;
                return None;
            }
        };
        let chunk_index: usize = match Self::validate_chunk_index(
            MultipartFormData::try_get_part_text(&parts, FIELD_CHUNK_INDEX),
        ) {
            Ok(index) => index,
            Err(error) => {
                Self::set_common_error_response_body(ctx, error).await;
                return None;
            }
        };
        if chunk_index >= data.get_total_chunks() {
            Self::set_common_error_response_body(ctx, ERROR_INVALID_CHUNK_INDEX.to_string()).await;
            return None;
        }
        data.set_chunk_index(chunk_index);
        let chunk_data: RequestBody = match Self::get_file_part_data(&parts) {
            Some(bytes) => bytes,
            None => {
                Self::set_common_error_response_body(ctx, ERROR_MISSING_FILE_PART.to_string())
                    .await;
                return None;
            }
        };
        Some((data, chunk_data))
    }

    /// Adds a file chunk data entry to the in-memory file ID map via the repository.
    ///
    /// # Arguments
    ///
    /// - `&FileChunkData` - The file chunk data to register.
    #[instrument_trace]
    pub async fn add_file_id_map(data: &FileChunkData) {
        let _: () = FileChunkRepository::add_file_id_map(data).await;
    }

    /// Removes a file chunk data entry from the in-memory file ID map via the repository.
    ///
    /// # Arguments
    ///
    /// - `&str` - The file ID to unregister.
    #[instrument_trace]
    pub async fn remove_file_id_map(file_id: &str) {
        let _: () = FileChunkRepository::remove_file_id_map(file_id).await;
    }

    /// Retrieves file chunk data for merging by file ID from the in-memory map.
    ///
    /// # Arguments
    ///
    /// - `String` - The file ID of the upload session.
    ///
    /// # Returns
    ///
    /// - `Option<FileChunkData>` - The file chunk data if found, or `None`.
    #[instrument_trace]
    pub async fn get_merge_file_chunk_data(file_id: String) -> Option<FileChunkData> {
        let result: Option<FileChunkData> =
            FileChunkRepository::get_merge_file_chunk_data(&file_id).await;
        result
    }

    /// Extracts the file ID of an upload session from the multipart body.
    ///
    /// # Arguments
    ///
    /// - `&mut Context` - The request context for setting error responses.
    ///
    /// # Returns
    ///
    /// - `Option<String>` - The file ID, or `None` when the body carries none.
    #[instrument_trace]
    pub async fn get_merge_file_id(ctx: &mut Context) -> Option<String> {
        let parts: MultipartFormDataParts = Self::get_request_parts(ctx).await?;
        let file_id: String = match Self::validate_file_id(MultipartFormData::try_get_part_text(
            &parts,
            FIELD_FILE_ID,
        )) {
            Ok(id) => id,
            Err(error) => {
                Self::set_common_error_response_body(ctx, error).await;
                return None;
            }
        };
        Some(file_id)
    }

    /// Sets a common success response with status 200, the uploaded file URL, and a JSON body.
    ///
    /// # Arguments
    ///
    /// - `&mut Context` - The request context to set the response on.
    /// - `&str` - The URL of the uploaded file.
    #[instrument_trace]
    pub async fn set_common_success_response_body(ctx: &mut Context, url: &str) {
        ctx.get_mut_response().set_status_code(200);
        let mut data: UploadResponse<'_> = UploadResponse::default();
        data.set_code(200).set_msg(OK).set_url(url);
        let data_json: ResponseBody = serde_json::to_vec(&data).unwrap_or_default();
        ctx.get_mut_response().set_body(&data_json);
    }

    /// Sets a common error response with status 200 and the error message in a JSON body.
    ///
    /// # Arguments
    ///
    /// - `&mut Context` - The request context to set the response on.
    /// - `String` - The error message.
    #[instrument_trace]
    pub async fn set_common_error_response_body(ctx: &mut Context, error: String) {
        ctx.get_mut_response().set_status_code(200);
        let mut data: UploadResponse<'_> = UploadResponse::default();
        data.set_msg(&error);
        let data_json: ResponseBody = serde_json::to_vec(&data).unwrap_or_default();
        ctx.get_mut_response().set_body(&data_json);
    }

    /// Serves a static file by decoding the directory and file name, reading the file from disk.
    ///
    /// # Arguments
    ///
    /// - `&str` - The encoded directory path.
    ///
    /// # Returns
    ///
    /// - `Result<(Vec<u8>, String), String>` - The file data bytes and content type, or an error.
    #[instrument_trace]
    pub async fn serve_static_file(dir: &str, file: &str) -> Result<(Vec<u8>, String), String> {
        let decode_dir: String = Decode::execute(charsets(), dir).unwrap_or_default();
        let decode_file: String = Decode::execute(charsets(), file).unwrap_or_default();
        if decode_dir.is_empty() || decode_file.is_empty() {
            return Err(ERROR_INVALID_DIRECTORY_OR_FILE_NAME.to_string());
        }
        let path: String = format!("{UPLOAD_DIR}/{decode_dir}/{decode_file}");
        let extension_name: String = FileExtension::get_extension_name(&decode_file);
        let file_type: &str = FileExtension::parse(&extension_name).get_content_type();
        let content_type: String = ContentType::format_content_type_with_charset(file_type, UTF8);
        let data: Vec<u8> = async_read_from_file(&path).await.unwrap_or_default();
        if data.is_empty() {
            return Err(ERROR_FILE_NOT_FOUND_OR_EMPTY.to_string());
        }
        Ok((data, content_type))
    }

    /// Parses an HTTP Range header into a `RangeRequest` specifying start and optional end positions.
    ///
    /// # Arguments
    ///
    /// - `&str` - The raw Range header value (e.g., "bytes=0-1023").
    /// - `u64` - The total file size in bytes.
    ///
    /// # Returns
    ///
    /// - `Result<RangeRequest, String>` - The parsed range request, or an error if the format is
    ///     invalid.
    ///
    /// A zero-length representation is rejected for every range form: the inclusive
    /// `(start, end)` representation used here cannot express a range covering no octets, so
    /// the caller falls back to serving the whole resource.
    #[instrument_trace]
    pub fn parse_range_header(range_header: &str, file_size: u64) -> Result<RangeRequest, String> {
        if !range_header.starts_with(RANGE_HEADER_PREFIX) {
            return Err(ERROR_INVALID_RANGE_HEADER_FORMAT.to_string());
        }
        let range_spec: &str = &range_header[RANGE_HEADER_PREFIX.len()..];
        let parts: Vec<&str> = range_spec.split('-').collect();
        if parts.len() != 2 {
            return Err(ERROR_INVALID_RANGE_SPECIFICATION.to_string());
        }
        let start_str: &str = parts[0];
        let end_str: &str = parts[1];
        if start_str.is_empty() && end_str.is_empty() {
            return Err(ERROR_INVALID_RANGE_BOTH_EMPTY.to_string());
        }
        if file_size == 0 {
            return Err(ERROR_RANGE_START_EXCEEDS_FILE_SIZE.to_string());
        }
        let is_digit_sequence: fn(&str) -> bool = |value: &str| -> bool {
            !value.is_empty() && value.bytes().all(|byte: u8| byte.is_ascii_digit())
        };
        let last_byte_index: u64 = file_size - 1;
        let is_suffix_range: bool = start_str.is_empty();
        let start: u64 = if is_suffix_range {
            if !is_digit_sequence(end_str) {
                return Err(ERROR_INVALID_END_RANGE.to_string());
            }
            let suffix_length: u64 = end_str
                .parse()
                .map_err(|_: ParseIntError| ERROR_INVALID_END_RANGE)?;
            if suffix_length == 0 {
                return Err(ERROR_INVALID_RANGE_ZERO_LENGTH_SUFFIX.to_string());
            }
            file_size - suffix_length.min(file_size)
        } else {
            if !is_digit_sequence(start_str) {
                return Err(ERROR_INVALID_START_RANGE.to_string());
            }
            start_str
                .parse()
                .map_err(|_: ParseIntError| ERROR_INVALID_START_RANGE)?
        };
        let resolved_end: u64 = if end_str.is_empty() || is_suffix_range {
            last_byte_index
        } else {
            if !is_digit_sequence(end_str) {
                return Err(ERROR_INVALID_END_RANGE.to_string());
            }
            end_str
                .parse::<u64>()
                .map_err(|_: ParseIntError| ERROR_INVALID_END_RANGE)?
                .min(last_byte_index)
        };
        if start >= file_size {
            return Err(ERROR_RANGE_START_EXCEEDS_FILE_SIZE.to_string());
        }
        if start > resolved_end {
            return Err(ERROR_INVALID_RANGE_START_GREATER_THAN_END.to_string());
        }
        let mut range_request: RangeRequest = RangeRequest::default();
        range_request
            .set_start(start)
            .set_end(if end_str.is_empty() && !is_suffix_range {
                None
            } else {
                Some(resolved_end)
            });
        Ok(range_request)
    }

    /// Reads a specific range of bytes from a file on disk.
    ///
    /// # Arguments
    ///
    /// - `&str` - The file path.
    /// - `u64` - The starting byte offset.
    ///
    /// # Returns
    ///
    /// - `Result<Vec<u8>, String>` - The read byte buffer, or an error if the file cannot be opened
    ///     or read.
    #[instrument_trace]
    pub async fn read_file_range(path: &str, start: u64, length: u64) -> Result<Vec<u8>, String> {
        let mut file: File =
            File::open(path).map_err(|error: Error| format!("Failed to open file {error}"))?;
        file.seek(SeekFrom::Start(start))
            .map_err(|error: Error| format!("Failed to seek file {error}"))?;
        let mut buffer: Vec<u8> = vec![0; length as usize];
        let bytes_read: usize = file
            .read(&mut buffer)
            .map_err(|error: Error| format!("Failed to read file {error}"))?;
        buffer.truncate(bytes_read);
        Ok(buffer)
    }

    /// Validates and URL-decodes the directory and file name from encoded parameters.
    ///
    /// # Arguments
    ///
    /// - `&str` - The encoded directory path.
    ///
    /// # Returns
    ///
    /// - `Result<(String, String), String>` - The decoded (directory, file name) tuple, or an
    ///     error.
    #[instrument_trace]
    fn validate_file_paths(dir: &str, file: &str) -> Result<(String, String), String> {
        let decode_dir: String = Decode::execute(charsets(), dir).unwrap_or_default();
        let decode_file: String = Decode::execute(charsets(), file).unwrap_or_default();
        if decode_dir.is_empty() || decode_file.is_empty() {
            return Err(ERROR_INVALID_DIRECTORY_OR_FILE_NAME.to_string());
        }
        Ok((decode_dir, decode_file))
    }

    /// Retrieves file metadata and determines the content type based on the file extension.
    ///
    /// # Arguments
    ///
    /// - `&str` - The file path on disk.
    ///
    /// # Returns
    ///
    /// - `Result<(Metadata, String), String>` - The file metadata and content type, or an
    ///     error.
    #[instrument_trace]
    fn get_file_metadata_and_content_type(
        path: &str,
        decode_file: &str,
    ) -> Result<(Metadata, String), String> {
        let file_metadata: Metadata =
            metadata(path).map_err(|_: Error| ERROR_FILE_NOT_FOUND.to_string())?;
        if file_metadata.len() == 0 {
            return Err(ERROR_FILE_IS_EMPTY.to_string());
        }
        let extension_name: String = FileExtension::get_extension_name(decode_file);
        let mut file_type: &str = FileExtension::parse(&extension_name).get_content_type();
        if file_type.is_empty() {
            file_type = FileExtension::FileExtensionText.get_content_type();
        }
        let content_type: String = ContentType::format_content_type_with_charset(file_type, UTF8);
        Ok((file_metadata, content_type))
    }

    /// Handles a range request by reading the specified byte range and building a partial content response.
    ///
    /// # Arguments
    ///
    /// - `&str` - The file path.
    /// - `RangeRequest` - The range specification.
    /// - `u64` - The total file size.
    /// - `String` - The content type.
    ///
    /// # Returns
    ///
    /// - `Result<(PartialContent, String), String>` - The partial content and content type.
    #[instrument_trace]
    async fn handle_range_request(
        path: &str,
        range: RangeRequest,
        file_size: u64,
        content_type: String,
    ) -> Result<(PartialContent, String), String> {
        let start: u64 = range.get_start();
        let end: u64 = range
            .try_get_end()
            .unwrap_or(file_size - 1)
            .min(file_size - 1);
        if start > end {
            return Err(ERROR_INVALID_RANGE_START_GREATER_THAN_END.to_string());
        }
        let content_length: u64 = end - start + 1;
        let data: Vec<u8> = Self::read_file_range(path, start, content_length).await?;
        let content_range: String = format!("bytes {start}-{end}/{file_size}");
        let mut partial_content: PartialContent = PartialContent::default();
        partial_content
            .set_data(data)
            .set_content_range(content_range)
            .set_content_length(content_length)
            .set_total_size(file_size);
        Ok((partial_content, content_type))
    }

    /// Handles a full file request by reading the entire file and building a partial content response.
    ///
    /// # Arguments
    ///
    /// - `&str` - The file path.
    /// - `u64` - The total file size.
    /// - `String` - The content type.
    ///
    /// # Returns
    ///
    /// - `Result<(PartialContent, String), String>` - The full file content and content type.
    #[instrument_trace]
    async fn handle_full_file_request(
        path: &str,
        file_size: u64,
        content_type: String,
    ) -> Result<(PartialContent, String), String> {
        let data: Vec<u8> = async_read_from_file(path).await.unwrap_or_default();
        if data.is_empty() {
            return Err(ERROR_FILE_NOT_FOUND_OR_EMPTY.to_string());
        }
        let content_range: String = format!("bytes 0-{}/{file_size}", file_size - 1);
        let mut partial_content: PartialContent = PartialContent::default();
        partial_content
            .set_data(data)
            .set_content_range(content_range)
            .set_content_length(file_size)
            .set_total_size(file_size);
        Ok((partial_content, content_type))
    }

    /// Serves a static file with optional range request support for partial content delivery.
    ///
    /// # Arguments
    ///
    /// - `&str` - The encoded directory path.
    /// - `Option<RangeRequest>` - The optional range request for partial content.
    ///
    /// # Returns
    ///
    /// - `Result<(PartialContent, String), String>` - The file content (partial or full) and
    ///     content type.
    #[instrument_trace]
    pub async fn serve_static_file_with_range(
        dir: &str,
        file: &str,
        range_request: Option<RangeRequest>,
    ) -> Result<(PartialContent, String), String> {
        let (decode_dir, decode_file) = Self::validate_file_paths(dir, file)?;
        let path: String = format!("{UPLOAD_DIR}/{decode_dir}/{decode_file}");
        let (file_metadata, content_type) =
            Self::get_file_metadata_and_content_type(&path, &decode_file)?;
        let file_size: u64 = file_metadata.len();
        match range_request {
            Some(range) => Self::handle_range_request(&path, range, file_size, content_type).await,
            None => Self::handle_full_file_request(&path, file_size, content_type).await,
        }
    }

    /// Saves a file chunk to disk using the configured chunk strategy.
    ///
    /// # Arguments
    ///
    /// - `&FileChunkData` - The file chunk metadata.
    /// - `Vec<u8>` - The chunk data bytes.
    ///
    /// # Returns
    ///
    /// - `Result<String, String>` - The save directory path on success, or an error message.
    #[instrument_trace]
    pub async fn save_file_chunk(
        file_chunk_data: &FileChunkData,
        chunk_data: Vec<u8>,
    ) -> Result<String, String> {
        if chunk_data.is_empty() {
            return Err(ChunkStrategyError::EmptyChunkData.to_string());
        }
        let file_id: &str = file_chunk_data.get_file_id();
        let file_name: &str = file_chunk_data.get_file_name();
        let chunk_index: usize = file_chunk_data.get_chunk_index();
        let total_chunks: usize = file_chunk_data.get_total_chunks();
        let base_file_dir: &str = file_chunk_data.get_base_file_dir();
        let save_upload_dir: String = format!("{UPLOAD_DIR}/{base_file_dir}/{file_id}");
        let upload_strategy: ChunkStrategy = ChunkStrategy::new(
            0,
            &save_upload_dir,
            file_id,
            file_name,
            total_chunks,
            |str_1: &str, size_1: usize| format!("{str_1}.{size_1}"),
        )
        .map_err(|error: ChunkStrategyError| error.to_string())?;
        upload_strategy
            .save_chunk(&chunk_data, chunk_index)
            .await
            .map_err(|error: ChunkStrategyError| error.to_string())?;
        Ok(save_upload_dir.clone())
    }

    /// Merges all file chunks into the final file and returns the file URL.
    ///
    /// # Arguments
    ///
    /// - `&FileChunkData` - The file chunk metadata containing file ID, name, and total chunks.
    ///
    /// # Returns
    ///
    /// - `Result<(String, String), String>` - A tuple of (save directory path, URL) on success, or
    ///     an error.
    #[instrument_trace]
    pub async fn merge_file_chunks(
        file_chunk_data: &FileChunkData,
    ) -> Result<(String, String), String> {
        let file_id: &str = file_chunk_data.get_file_id();
        let file_name: &str = file_chunk_data.get_file_name();
        let total_chunks: usize = file_chunk_data.get_total_chunks();
        let base_file_dir: &str = file_chunk_data.get_base_file_dir();
        let save_upload_dir: String = format!("{UPLOAD_DIR}/{base_file_dir}/{file_id}");
        let upload_strategy: ChunkStrategy = ChunkStrategy::new(
            0,
            &save_upload_dir,
            file_id,
            file_name,
            total_chunks,
            |str_1: &str, size_1: usize| format!("{str_1}.{size_1}"),
        )
        .map_err(|error: ChunkStrategyError| error.to_string())?;
        let url_encode_dir: String =
            Encode::execute(charsets(), &format!("{base_file_dir}/{file_id}")).unwrap_or_default();
        let url_encode_dir_file_name: String =
            Encode::execute(charsets(), file_name).unwrap_or_default();
        let url: String = format!("/{STATIC_ROUTE}/{url_encode_dir}/{url_encode_dir_file_name}");
        upload_strategy
            .merge_chunks()
            .await
            .map_err(|error: ChunkStrategyError| error.to_string())?;
        Self::remove_file_id_map(file_id).await;
        Ok((save_upload_dir.clone(), url))
    }
}
