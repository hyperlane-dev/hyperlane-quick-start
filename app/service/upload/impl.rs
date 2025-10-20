use super::*;

impl UploadService {
    pub fn get_base_file_dir() -> String {
        let (year, month, day, hour, minute, _, _, _) = calculate_time();
        let full_dir: String = format!("{year}/{month}/{day}/{hour}/{minute}");
        full_dir
    }

    fn validate_file_id(file_id_opt: Option<String>, ctx: &Context) -> Result<String, ()> {
        match file_id_opt {
            Some(id) => Ok(id),
            None => {
                tokio::spawn({
                    let ctx = ctx.clone();
                    async move {
                        Self::set_common_error_response_body(
                            &ctx,
                            ChunkStrategyError::MissingFileId.to_string(),
                        )
                        .await;
                    }
                });
                Err(())
            }
        }
    }

    fn validate_total_chunks(total_chunks_opt: Option<String>, ctx: &Context) -> Result<usize, ()> {
        match total_chunks_opt {
            Some(total) => match total.parse::<usize>() {
                Ok(t) => Ok(t),
                Err(_) => {
                    tokio::spawn({
                        let ctx = ctx.clone();
                        async move {
                            Self::set_common_error_response_body(
                                &ctx,
                                ChunkStrategyError::InvalidTotalChunks.to_string(),
                            )
                            .await;
                        }
                    });
                    Err(())
                }
            },
            None => {
                tokio::spawn({
                    let ctx = ctx.clone();
                    async move {
                        Self::set_common_error_response_body(
                            &ctx,
                            ChunkStrategyError::MissingTotalChunks.to_string(),
                        )
                        .await;
                    }
                });
                Err(())
            }
        }
    }

    fn validate_file_name(file_name_opt: Option<String>, ctx: &Context) -> Result<String, ()> {
        match file_name_opt {
            Some(name) => Ok(urlencoding::decode(&name).unwrap_or_default().into_owned()),
            None => {
                tokio::spawn({
                    let ctx = ctx.clone();
                    async move {
                        Self::set_common_error_response_body(
                            &ctx,
                            ChunkStrategyError::MissingFileName.to_string(),
                        )
                        .await;
                    }
                });
                Err(())
            }
        }
    }

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

    fn is_valid_directory_path(path: &str) -> bool {
        !path.is_empty()
            && !path.contains("../")
            && path.chars().all(|c| c.is_ascii_digit() || c == '/')
    }

    #[request_header(CHUNKIFY_FILE_ID_HEADER => file_id_opt)]
    #[request_header(CHUNKIFY_TOTAL_CHUNKS_HEADER => total_chunks_opt)]
    #[request_header(CHUNKIFY_FILE_NAME_HEADER => file_name_opt)]
    #[request_header(CHUNKIFY_DIRECTORY_HEADER => base_file_dir_opt)]
    pub async fn get_register_file_chunk_data<'a>(ctx: &'a Context) -> OptionFileChunkData {
        let file_id: String = Self::validate_file_id(file_id_opt, ctx).ok()?;
        let total_chunks: usize = Self::validate_total_chunks(total_chunks_opt, ctx).ok()?;
        let file_name: String = Self::validate_file_name(file_name_opt, ctx).ok()?;
        let base_file_dir: String = Self::validate_and_decode_directory(base_file_dir_opt);
        let mut data: FileChunkData = FileChunkData::default();
        data.set_file_id(file_id)
            .set_file_name(file_name)
            .set_chunk_index(0)
            .set_total_chunks(total_chunks)
            .set_base_file_dir(base_file_dir);
        Some(data)
    }

    pub async fn get_save_file_chunk_data<'a>(
        ctx: &'a Context,
        file_id_opt: Option<String>,
        chunk_index_opt: Option<String>,
    ) -> OptionFileChunkData {
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

    pub async fn add_file_id_map(data: &FileChunkData) {
        write_file_id_map()
            .await
            .insert(data.get_file_id().to_owned(), data.clone());
    }

    pub async fn remove_file_id_map(file_id: &str) {
        write_file_id_map().await.remove(file_id);
    }

    pub async fn get_merge_file_chunk_data<'a>(
        ctx: &Context,
        file_id_opt: Option<String>,
    ) -> OptionFileChunkData {
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
        read_file_id_map()
            .await
            .get(&file_id).cloned()
    }

    #[response_status_code(200)]
    pub async fn set_common_success_response_body<'a>(ctx: &'a Context, url: &'a str) {
        let mut data: UploadResponse<'_> = UploadResponse {
            code: 0,
            url: "",
            msg: "",
        };
        data.set_code(200).set_msg(OK).set_url(url);
        let data_json: ResponseBody = serde_json::to_vec(&data).unwrap_or_default();
        let _ = ctx.set_response_body(&data_json).await;
    }

    #[response_status_code(200)]
    pub async fn set_common_error_response_body<'a>(ctx: &'a Context, error: String) {
        let mut data: UploadResponse<'_> = UploadResponse {
            code: 0,
            url: "",
            msg: "",
        };
        data.set_msg(&error);
        let data_json: ResponseBody = serde_json::to_vec(&data).unwrap_or_default();
        let _ = ctx.set_response_body(&data_json).await;
    }

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

    pub fn parse_range_header(range_header: &str, file_size: u64) -> Result<RangeRequest, String> {
        if !range_header.starts_with("bytes=") {
            return Err("Invalid range header format".to_string());
        }
        let range_spec: &str = &range_header[6..];
        let parts: Vec<&str> = range_spec.split('-').collect();
        if parts.len() != 2 {
            return Err("Invalid range specification".to_string());
        }
        let start_str: &str = parts[0];
        let end_str: &str = parts[1];
        if start_str.is_empty() && end_str.is_empty() {
            return Err("Invalid range: both start and end are empty".to_string());
        }
        let start: u64 = if start_str.is_empty() {
            let suffix_length: u64 = end_str.parse().map_err(|_| "Invalid end range")?;
            file_size.saturating_sub(suffix_length)
        } else {
            start_str.parse().map_err(|_| "Invalid start range")?
        };
        let end: Option<u64> = if end_str.is_empty() {
            None
        } else {
            Some(end_str.parse().map_err(|_| "Invalid end range")?)
        };
        if start >= file_size {
            return Err("Range start exceeds file size".to_string());
        }
        let mut range_request: RangeRequest = RangeRequest::default();
        range_request.set_start(start).set_end(end);
        Ok(range_request)
    }

    pub async fn read_file_range(path: &str, start: u64, length: u64) -> Result<Vec<u8>, String> {
        use std::io::{Read, Seek, SeekFrom};
        let mut file: std::fs::File =
            std::fs::File::open(path).map_err(|error| format!("Failed to open file: {error}"))?;
        file.seek(SeekFrom::Start(start))
            .map_err(|error| format!("Failed to seek file: {error}"))?;
        let mut buffer: Vec<u8> = vec![0; length as usize];
        let bytes_read: usize = file
            .read(&mut buffer)
            .map_err(|error| format!("Failed to read file: {error}"))?;
        buffer.truncate(bytes_read);
        Ok(buffer)
    }

    fn validate_file_paths(dir: &str, file: &str) -> Result<(String, String), String> {
        let decode_dir: String = Decode::execute(CHARSETS, dir).unwrap_or_default();
        let decode_file: String = Decode::execute(CHARSETS, file).unwrap_or_default();

        if decode_dir.is_empty() || decode_file.is_empty() {
            return Err("Invalid directory or file name".to_string());
        }

        Ok((decode_dir, decode_file))
    }

    fn get_file_metadata_and_content_type(
        path: &str,
        decode_file: &str,
    ) -> Result<(std::fs::Metadata, String), String> {
        let file_metadata: std::fs::Metadata =
            std::fs::metadata(path).map_err(|_| "File not found".to_string())?;
        if file_metadata.len() == 0 {
            return Err("File is empty".to_string());
        }
        let extension_name: String = FileExtension::get_extension_name(decode_file);
        let mut file_type: &str = FileExtension::parse(&extension_name).get_content_type();
        if file_type.is_empty() {
            file_type = FileExtension::FileExtensionText.get_content_type();
        }
        let content_type: String = ContentType::format_content_type_with_charset(file_type, UTF8);
        Ok((file_metadata, content_type))
    }

    async fn handle_range_request(
        path: &str,
        range: RangeRequest,
        file_size: u64,
        content_type: String,
    ) -> Result<(PartialContent, String), String> {
        let start: u64 = *range.get_start();
        let end: u64 = range.get_end().unwrap_or(file_size - 1).min(file_size - 1);
        if start > end {
            return Err("Invalid range: start > end".to_string());
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

    pub async fn save_file_chunk(
        file_chunk_data: &FileChunkData,
        chunk_data: Vec<u8>,
    ) -> Result<String, String> {
        if chunk_data.is_empty() {
            return Err(ChunkStrategyError::EmptyChunkData.to_string());
        }
        let file_id: &str = file_chunk_data.get_file_id();
        let file_name: &str = file_chunk_data.get_file_name();
        let chunk_index: &usize = file_chunk_data.get_chunk_index();
        let total_chunks: &usize = file_chunk_data.get_total_chunks();
        let base_file_dir: &str = file_chunk_data.get_base_file_dir();
        let save_upload_dir: String = format!("{UPLOAD_DIR}/{base_file_dir}/{file_id}");
        let upload_strategy: ChunkStrategy = ChunkStrategy::new(
            0,
            &save_upload_dir,
            file_id,
            file_name,
            *total_chunks,
            |a, b| format!("{a}.{b}"),
        )
        .map_err(|error| error.to_string())?;
        upload_strategy
            .save_chunk(&chunk_data, *chunk_index)
            .await
            .map_err(|error| error.to_string())?;
        Ok(save_upload_dir.clone())
    }

    pub async fn merge_file_chunks(
        file_chunk_data: &FileChunkData,
    ) -> Result<(String, String), String> {
        let file_id: &str = file_chunk_data.get_file_id();
        let file_name: &str = file_chunk_data.get_file_name();
        let total_chunks: &usize = file_chunk_data.get_total_chunks();
        let base_file_dir: &str = file_chunk_data.get_base_file_dir();
        let save_upload_dir: String = format!("{UPLOAD_DIR}/{base_file_dir}/{file_id}");
        let upload_strategy: ChunkStrategy = ChunkStrategy::new(
            0,
            &save_upload_dir,
            file_id,
            file_name,
            *total_chunks,
            |a, b| format!("{a}.{b}"),
        )
        .map_err(|error| error.to_string())?;
        let url_encode_dir: String =
            Encode::execute(CHARSETS, &format!("{base_file_dir}/{file_id}")).unwrap_or_default();
        let url_encode_dir_file_name: String =
            Encode::execute(CHARSETS, file_name).unwrap_or_default();
        let url: String = format!("/{STATIC_ROUTE}/{url_encode_dir}/{url_encode_dir_file_name}");
        upload_strategy
            .merge_chunks()
            .await
            .map_err(|error| error.to_string())?;
        Self::remove_file_id_map(file_id).await;
        Ok((save_upload_dir.clone(), url))
    }
}
