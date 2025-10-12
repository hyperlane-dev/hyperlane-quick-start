use super::*;

#[route("/api/upload/register")]
#[utoipa::path(
    post,
    path = "/api/upload/register",
    description = "Register file chunk upload session",
    responses(
        (status = 200, description = "Successfully registered upload session", body = UploadResponse)
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
#[prologue_macros(
    post,
    request_header(CHUNKIFY_FILE_ID_HEADER => file_id_opt),
    request_header(CHUNKIFY_CHUNK_INDEX_HEADER => chunk_index_opt)
)]
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
            ctx.set_response_header("save_upload_dir", save_upload_dir)
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
#[prologue_macros(
    post,
    request_header(CHUNKIFY_FILE_ID_HEADER => file_id_opt)
)]
pub async fn merge(ctx: Context) {
    let file_chunk_data_opt: OptionFileChunkData =
        get_merge_file_chunk_data(&ctx, file_id_opt).await;
    if file_chunk_data_opt.is_none() {
        return;
    }
    let file_chunk_data: FileChunkData = file_chunk_data_opt.unwrap_or_default();
    match merge_file_chunks(&file_chunk_data).await {
        Ok((save_upload_dir, url)) => {
            ctx.set_response_header("save_upload_dir", save_upload_dir)
                .await;
            set_common_success_response_body(&ctx, &url).await;
        }
        Err(error) => {
            set_common_error_response_body(&ctx, error).await;
        }
    }
}
