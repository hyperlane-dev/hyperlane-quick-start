use super::*;

/// Implementation of methods for `UploadService`.
impl UploadService {
    /// Generates a date-time based directory path for file storage (year/month/day/hour/minute).
    ///
    /// # Returns
    ///
    /// - `String`: The formatted directory path string.
    #[instrument_trace]
    pub fn get_base_file_dir() -> String {
        let (year, month, day, hour, minute, _, _, _) = calculate_time();
        let full_dir: String = format!("{year}/{month}/{day}/{hour}/{minute}");
        full_dir
    }

    /// Validates that a file ID is present, setting an error response if missing.
    ///
    /// # Arguments
    ///
    /// - `Option<String>`: The optional file ID from the request header.
    /// - `&mut Context`: The request context for setting error responses.
    ///
    /// # Returns
    ///
    /// - `Result<String, ()>`: The file ID if present, or an error.
    #[instrument_trace]
    async fn validate_file_id(
        file_id_opt: Option<String>,
        ctx: &mut Context,
    ) -> Result<String, ()> {
        match file_id_opt {
            Some(id) => Ok(id),
            None => {
                Self::set_common_error_response_body(
                    ctx,
                    ChunkStrategyError::MissingFileId.to_string(),
                )
                .await;
                Err(())
            }
        }
    }

    /// Validates that the total chunks header is present and parseable, setting an error response if invalid.
    ///
    /// # Arguments
    ///
    /// - `Option<String>`: The optional total chunks value from the request header.
    /// - `&mut Context`: The request context for setting error responses.
    ///
    /// # Returns
    ///
    /// - `Result<usize, ()>`: The parsed total chunks count, or an error.
    #[instrument_trace]
    async fn validate_total_chunks(
        total_chunks_opt: Option<String>,
        ctx: &mut Context,
    ) -> Result<usize, ()> {
        match total_chunks_opt {
            Some(total) => match total.parse::<usize>() {
                Ok(t) => Ok(t),
                Err(_) => {
                    Self::set_common_error_response_body(
                        ctx,
                        ChunkStrategyError::InvalidTotalChunks.to_string(),
                    )
                    .await;
                    Err(())
                }
            },
            None => {
                Self::set_common_error_response_body(
                    ctx,
                    ChunkStrategyError::MissingTotalChunks.to_string(),
                )
                .await;
                Err(())
            }
        }
    }

    /// Validates that the file name header is present, URL-decoding it, setting an error response if missing.
    ///
    /// # Arguments
    ///
    /// - `Option<String>`: The optional file name from the request header.
    /// - `&mut Context`: The request context for setting error responses.
    ///
    /// # Returns
    ///
    /// - `Result<String, ()>`: The decoded file name, or an error.
    #[instrument_trace]
    async fn validate_file_name(
        file_name_opt: Option<String>,
        ctx: &mut Context,
    ) -> Result<String, ()> {
        match file_name_opt {
            Some(name) => Ok(urlencoding::decode(&name).unwrap_or_default().into_owned()),
            None => {
                Self::set_common_error_response_body(
                    ctx,
                    ChunkStrategyError::MissingFileName.to_string(),
                )
                .await;
                Err(())
            }
        }
    }

    /// Validates and URL-decodes a directory header, falling back to the base file dir if invalid or missing.
    ///
    /// # Arguments
    ///
    /// - `Option<String>`: The optional URL-encoded directory from the request header.
    ///
    /// # Returns
    ///
    /// - `String`: The decoded directory path, or the default base file dir.
    #[instrument_trace]
    fn validate_and_decode_directory(base_file_dir_opt: Option<String>) -> String {
        match base_file_dir_opt {
            Some(encode_dir) => {
                let decode_dir: String = urlencoding::decode(&encode_dir)
                    .unwrap_or_default()
                    .into_owned();
                if Self::is_valid_directory_path(&decode_dir) {
                    decode_dir
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
    /// - `&str`: The directory path to validate.
    ///
    /// # Returns
    ///
    /// - `bool`: `true` if the path is valid, `false` otherwise.
    #[instrument_trace]
    fn is_valid_directory_path(path: &str) -> bool {
        !path.is_empty()
            && !path.contains("../")
            && path.chars().all(|c: char| c.is_ascii_digit() || c == '/')
    }

    /// Extracts and validates file chunk data from request headers for a chunk upload registration.
    ///
    /// # Returns
    ///
    /// - `Option<FileChunkData>`: The validated file chunk data, or `None` if validation fails.
    #[try_get_request_header(HEADER_X_FILE_ID => file_id_opt)]
    #[try_get_request_header(HEADER_X_TOTAL_CHUNKS => total_chunks_opt)]
    #[try_get_request_header(HEADER_X_FILE_NAME => file_name_opt)]
    #[try_get_request_header(HEADER_X_DIRECTORY => base_file_dir_opt)]
    #[instrument_trace]
    pub async fn get_register_file_chunk_data<'a>(
        _stream: &mut Stream,
        ctx: &mut Context,
    ) -> Option<FileChunkData> {
        let file_id: String = Self::validate_file_id(file_id_opt, ctx).await.ok()?;
        let total_chunks: usize = Self::validate_total_chunks(total_chunks_opt, ctx)
            .await
            .ok()?;
        let file_name: String = Self::validate_file_name(file_name_opt, ctx).await.ok()?;
        let base_file_dir: String = Self::validate_and_decode_directory(base_file_dir_opt);
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
    /// # Arguments
    ///
    /// - `&mut Context`: The request context for setting error responses.
    /// - `Option<String>`: The optional file ID.
    /// - `Option<String>`: The optional chunk index.
    ///
    /// # Returns
    ///
    /// - `Option<FileChunkData>`: The validated file chunk data with chunk index, or `None`.
    #[instrument_trace]
    pub async fn get_save_file_chunk_data(
        ctx: &mut Context,
        file_id_opt: Option<String>,
        chunk_index_opt: Option<String>,
    ) -> Option<FileChunkData> {
        let mut data: FileChunkData = Self::get_merge_file_chunk_data(ctx, file_id_opt).await?;
        let chunk_index: usize = match chunk_index_opt {
            Some(idx) => match idx.parse::<usize>() {
                Ok(i) => i,
                Err(_) => {
                    Self::set_common_error_response_body(
                        ctx,
                        ChunkStrategyError::InvalidChunkIndex.to_string(),
                    )
                    .await;
                    return None;
                }
            },
            None => {
                Self::set_common_error_response_body(
                    ctx,
                    ChunkStrategyError::MissingChunkIndex.to_string(),
                )
                .await;
                return None;
            }
        };
        data.set_chunk_index(chunk_index);
        Some(data)
    }

    /// Adds a file chunk data entry to the in-memory file ID map via the repository.
    ///
    /// # Arguments
    ///
    /// - `&FileChunkData`: The file chunk data to register.
    #[instrument_trace]
    pub async fn add_file_id_map(data: &FileChunkData) {
        let _: () = FileChunkRepository::add_file_id_map(data).await;
    }

    /// Removes a file chunk data entry from the in-memory file ID map via the repository.
    ///
    /// # Arguments
    ///
    /// - `&str`: The file ID to unregister.
    #[instrument_trace]
    pub async fn remove_file_id_map(file_id: &str) {
        let _: () = FileChunkRepository::remove_file_id_map(file_id).await;
    }

    /// Retrieves file chunk data for merging by file ID from the in-memory map.
    ///
    /// # Arguments
    ///
    /// - `&mut Context`: The request context for setting error responses.
    /// - `Option<String>`: The optional file ID.
    ///
    /// # Returns
    ///
    /// - `Option<FileChunkData>`: The file chunk data if found, or `None`.
    #[instrument_trace]
    pub async fn get_merge_file_chunk_data(
        ctx: &mut Context,
        file_id_opt: Option<String>,
    ) -> Option<FileChunkData> {
        let file_id: String = match file_id_opt {
            Some(id) => id,
            None => {
                Self::set_common_error_response_body(
                    ctx,
                    ChunkStrategyError::MissingFileId.to_string(),
                )
                .await;
                return None;
            }
        };
        let result: Option<FileChunkData> =
            FileChunkRepository::get_merge_file_chunk_data(&file_id).await;
        result
    }

    /// Sets a common success response with status 200, the uploaded file URL, and a JSON body.
    ///
    /// # Arguments
    ///
    /// - `&mut Context`: The request context to set the response on.
    /// - `&str`: The URL of the uploaded file.
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
    /// - `&mut Context`: The request context to set the response on.
    /// - `String`: The error message.
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
    /// - `&str`: The encoded directory path.
    /// - `&str`: The encoded file name.
    ///
    /// # Returns
    ///
    /// - `Result<(Vec<u8>, String), String>`: The file data bytes and content type, or an error.
    #[instrument_trace]
    pub async fn serve_static_file(dir: &str, file: &str) -> Result<(Vec<u8>, String), String> {
        let decode_dir: String = Decode::execute(CHARSETS, dir).unwrap_or_default();
        let decode_file: String = Decode::execute(CHARSETS, file).unwrap_or_default();
        if decode_dir.is_empty() || decode_file.is_empty() {
            return Err("Invalid directory or file name".to_string());
        }
        let path: String = format!("{UPLOAD_DIR}/{decode_dir}/{decode_file}");
        let extension_name: String = FileExtension::get_extension_name(&decode_file);
        let file_type: &str = FileExtension::parse(&extension_name).get_content_type();
        let content_type: String = ContentType::format_content_type_with_charset(file_type, UTF8);
        let data: Vec<u8> = async_read_from_file(&path).await.unwrap_or_default();
        if data.is_empty() {
            return Err("File not found or empty".to_string());
        }
        Ok((data, content_type))
    }

    /// Parses an HTTP Range header into a `RangeRequest` specifying start and optional end positions.
    ///
    /// # Arguments
    ///
    /// - `&str`: The raw Range header value (e.g., "bytes=0-1023").
    /// - `u64`: The total file size in bytes.
    ///
    /// # Returns
    ///
    /// - `Result<RangeRequest, String>`: The parsed range request, or an error if the format is invalid.
    #[instrument_trace]
    pub fn parse_range_header(range_header: &str, file_size: u64) -> Result<RangeRequest, String> {
        if !range_header.starts_with(RANGE_HEADER_PREFIX) {
            return Err("Invalid range header format".to_string());
        }
        let range_spec: &str = &range_header[6..];
        let parts: Vec<&str> = range_spec.split('-').collect();
        if parts.len() != 2 {
            return Err(ERROR_INVALID_RANGE_SPECIFICATION.to_string());
        }
        let start_str: &str = parts[0];
        let end_str: &str = parts[1];
        if start_str.is_empty() && end_str.is_empty() {
            return Err("Invalid range: both start and end are empty".to_string());
        }
        let start: u64 = if start_str.is_empty() {
            let suffix_length: u64 = end_str
                .parse()
                .map_err(|_: ParseIntError| ERROR_INVALID_END_RANGE)?;
            file_size.saturating_sub(suffix_length)
        } else {
            start_str
                .parse()
                .map_err(|_: ParseIntError| ERROR_INVALID_START_RANGE)?
        };
        let end: Option<u64> = if end_str.is_empty() {
            None
        } else {
            Some(
                end_str
                    .parse()
                    .map_err(|_: ParseIntError| ERROR_INVALID_END_RANGE)?,
            )
        };
        if start >= file_size {
            return Err(ERROR_RANGE_START_EXCEEDS_FILE_SIZE.to_string());
        }
        let mut range_request: RangeRequest = RangeRequest::default();
        range_request.set_start(start).set_end(end);
        Ok(range_request)
    }

    /// Reads a specific range of bytes from a file on disk.
    ///
    /// # Arguments
    ///
    /// - `&str`: The file path.
    /// - `u64`: The starting byte offset.
    /// - `u64`: The number of bytes to read.
    ///
    /// # Returns
    ///
    /// - `Result<Vec<u8>, String>`: The read byte buffer, or an error if the file cannot be opened or read.
    #[instrument_trace]
    pub async fn read_file_range(path: &str, start: u64, length: u64) -> Result<Vec<u8>, String> {
        use std::io::{Read, Seek, SeekFrom};
        let mut file: std::fs::File = std::fs::File::open(path)
            .map_err(|error: std::io::Error| format!("Failed to open file {error}"))?;
        file.seek(SeekFrom::Start(start))
            .map_err(|error: std::io::Error| format!("Failed to seek file {error}"))?;
        let mut buffer: Vec<u8> = vec![0; length as usize];
        let bytes_read: usize = file
            .read(&mut buffer)
            .map_err(|error: std::io::Error| format!("Failed to read file {error}"))?;
        buffer.truncate(bytes_read);
        Ok(buffer)
    }

    /// Validates and URL-decodes the directory and file name from encoded parameters.
    ///
    /// # Arguments
    ///
    /// - `&str`: The encoded directory path.
    /// - `&str`: The encoded file name.
    ///
    /// # Returns
    ///
    /// - `Result<(String, String), String>`: The decoded (directory, file name) tuple, or an error.
    #[instrument_trace]
    fn validate_file_paths(dir: &str, file: &str) -> Result<(String, String), String> {
        let decode_dir: String = Decode::execute(CHARSETS, dir).unwrap_or_default();
        let decode_file: String = Decode::execute(CHARSETS, file).unwrap_or_default();
        if decode_dir.is_empty() || decode_file.is_empty() {
            return Err("Invalid directory or file name".to_string());
        }
        Ok((decode_dir, decode_file))
    }

    /// Retrieves file metadata and determines the content type based on the file extension.
    ///
    /// # Arguments
    ///
    /// - `&str`: The file path on disk.
    /// - `&str`: The decoded file name for extension parsing.
    ///
    /// # Returns
    ///
    /// - `Result<(std::fs::Metadata, String), String>`: The file metadata and content type, or an error.
    #[instrument_trace]
    fn get_file_metadata_and_content_type(
        path: &str,
        decode_file: &str,
    ) -> Result<(std::fs::Metadata, String), String> {
        let file_metadata: std::fs::Metadata =
            std::fs::metadata(path).map_err(|_: std::io::Error| "File not found".to_string())?;
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
    /// - `&str`: The file path.
    /// - `RangeRequest`: The range specification.
    /// - `u64`: The total file size.
    /// - `String`: The content type.
    ///
    /// # Returns
    ///
    /// - `Result<(PartialContent, String), String>`: The partial content and content type.
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
    /// - `&str`: The file path.
    /// - `u64`: The total file size.
    /// - `String`: The content type.
    ///
    /// # Returns
    ///
    /// - `Result<(PartialContent, String), String>`: The full file content and content type.
    #[instrument_trace]
    async fn handle_full_file_request(
        path: &str,
        file_size: u64,
        content_type: String,
    ) -> Result<(PartialContent, String), String> {
        let data: Vec<u8> = async_read_from_file(path).await.unwrap_or_default();
        if data.is_empty() {
            return Err("File not found or empty".to_string());
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
    /// - `&str`: The encoded directory path.
    /// - `&str`: The encoded file name.
    /// - `Option<RangeRequest>`: The optional range request for partial content.
    ///
    /// # Returns
    ///
    /// - `Result<(PartialContent, String), String>`: The file content (partial or full) and content type.
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
    /// - `&FileChunkData`: The file chunk metadata.
    /// - `Vec<u8>`: The chunk data bytes.
    ///
    /// # Returns
    ///
    /// - `Result<String, String>`: The save directory path on success, or an error message.
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
    /// - `&FileChunkData`: The file chunk metadata containing file ID, name, and total chunks.
    ///
    /// # Returns
    ///
    /// - `Result<(String, String), String>`: A tuple of (save directory path, URL) on success, or an error.
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
            Encode::execute(CHARSETS, &format!("{base_file_dir}/{file_id}")).unwrap_or_default();
        let url_encode_dir_file_name: String =
            Encode::execute(CHARSETS, file_name).unwrap_or_default();
        let url: String = format!("/{STATIC_ROUTE}/{url_encode_dir}/{url_encode_dir_file_name}");
        upload_strategy
            .merge_chunks()
            .await
            .map_err(|error: ChunkStrategyError| error.to_string())?;
        Self::remove_file_id_map(file_id).await;
        Ok((save_upload_dir.clone(), url))
    }
}
