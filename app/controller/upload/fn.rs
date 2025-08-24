use super::*;

#[route("/static/{upload_dir}/{upload_file}")]
#[utoipa::path(
    get,
    post,
    path = "/static/{upload_dir}/{upload_file}",
    responses(
        (status = 200, description = "Static resources", body = String),
        (status = 206, description = "Partial content", body = String)
    )
)]
#[prologue_hooks[
    methods(get, post),
    route_param(UPLOAD_DIR_KEY => dir_opt),
    route_param(UPLOAD_FILE_KEY => file_opt),
    request_header(RANGE => range_header_opt)
]]
pub async fn static_file(ctx: Context) {
    let dir: String = dir_opt.unwrap_or_default();
    let file: String = file_opt.unwrap_or_default();
    let has_range_request: bool = range_header_opt.is_some();
    let range_request: Option<RangeRequest> = match &range_header_opt {
        Some(range_header) => {
            let file_path: String = format!(
                "{UPLOAD_DIR}/{}/{}",
                Decode::execute(CHARSETS, &dir).unwrap_or_default(),
                Decode::execute(CHARSETS, &file).unwrap_or_default()
            );
            match std::fs::metadata(&file_path) {
                Ok(metadata) => {
                    let file_size: u64 = metadata.len();
                    match parse_range_header(&range_header, file_size) {
                        Ok(range) => Some(range),
                        Err(_) => None,
                    }
                }
                Err(_) => None,
            }
        }
        None => None,
    };
    match serve_static_file_with_range(&dir, &file, range_request).await {
        Ok((partial_content, content_type)) => {
            ctx.set_response_header(CONTENT_TYPE, content_type).await;
            ctx.set_response_header(ACCEPT_RANGES, BYTES).await;
            ctx.set_response_header(CACHE_CONTROL, CACHE_CONTROL_STATIC_ASSETS)
                .await;
            ctx.set_response_header(EXPIRES, EXPIRES_FAR_FUTURE).await;
            if has_range_request {
                ctx.set_response_status_code(HttpStatus::PartialContent.code())
                    .await;
                ctx.set_response_header(CONTENT_RANGE, partial_content.content_range)
                    .await;
                ctx.set_response_header(CONTENT_LENGTH, partial_content.content_length.to_string())
                    .await;
            } else {
                ctx.set_response_status_code(200).await;
                ctx.set_response_header(CONTENT_LENGTH, partial_content.total_size.to_string())
                    .await;
            }
            ctx.set_response_body(partial_content.data).await;
        }
        Err(_) => {
            return;
        }
    }
}

#[route("/upload")]
#[utoipa::path(
    get,
    post,
    path = "/upload/index.html",   
    responses(
        (status = 200, description = "File chunk upload frontend interface", body = String)
    )
)]
#[prologue_hooks[
    methods(get, post),
    response_status_code(200),
    response_body(UPLOAD_HTML),
    response_header(CONTENT_ENCODING => GZIP)
]]
pub async fn html(ctx: Context) {}

#[route("/api/upload/register")]
#[utoipa::path(
    post,
    path = "/api/upload/register",   
    responses(
        (status = 200, description = "File chunk upload - register API", body = UploadResponse)
    )
)]
#[post]
pub async fn register(ctx: Context) {
    let file_chunk_data_opt: OptionFileChunkData = get_register_file_chunk_data(&ctx).await;
    if file_chunk_data_opt.is_none() {
        return;
    }
    let file_chunk_data: FileChunkData = file_chunk_data_opt.unwrap_or_default();
    add_file_id_map(&file_chunk_data).await;
    set_common_success_response_body(&ctx, "").await;
}

#[route("/api/upload/save")]
#[utoipa::path(
    post,
    path = "/api/upload/save",   
    responses(
        (status = 200, description = "File chunk upload - save API", body = UploadResponse)
    )
)]
#[prologue_hooks[
    post,
    request_header(CHUNKIFY_FILE_ID_HEADER => file_id_opt),
    request_header(CHUNKIFY_CHUNK_INDEX_HEADER => chunk_index_opt)
]]
pub async fn save(ctx: Context) {
    let file_chunk_data_opt: OptionFileChunkData =
        get_save_file_chunk_data(&ctx, file_id_opt, chunk_index_opt).await;
    if file_chunk_data_opt.is_none() {
        return;
    }
    let file_chunk_data: FileChunkData = file_chunk_data_opt.unwrap_or_default();
    let chunk_data: Vec<u8> = ctx.get_request_body().await;
    match save_file_chunk(&file_chunk_data, chunk_data).await {
        Ok(save_upload_dir) => {
            ctx.set_response_header("X-File-Path", save_upload_dir)
                .await;
            set_common_success_response_body(&ctx, EMPTY_STR).await;
        }
        Err(error) => {
            set_common_error_response_body(&ctx, error).await;
        }
    }
}

#[route("/api/upload/merge")]
#[utoipa::path(
    post,
    path = "/api/upload/merge",   
    responses(
        (status = 200, description = "File chunk upload - merge API", body = UploadResponse)
    )
)]
#[prologue_hooks[
    post,
    request_header(CHUNKIFY_FILE_ID_HEADER => file_id_opt)
]]
pub async fn merge(ctx: Context) {
    let file_chunk_data_opt: OptionFileChunkData =
        get_merge_file_chunk_data(&ctx, file_id_opt).await;
    if file_chunk_data_opt.is_none() {
        return;
    }
    let file_chunk_data: FileChunkData = file_chunk_data_opt.unwrap_or_default();
    match merge_file_chunks(&file_chunk_data).await {
        Ok((save_upload_dir, url)) => {
            ctx.set_response_header("file", save_upload_dir).await;
            set_common_success_response_body(&ctx, &url).await;
        }
        Err(error) => {
            set_common_error_response_body(&ctx, error).await;
        }
    }
}
