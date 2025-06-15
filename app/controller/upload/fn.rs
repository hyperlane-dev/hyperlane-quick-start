use super::*;

#[methods(get, post)]
pub async fn html(ctx: Context) {
    let _ = ctx
        .set_response_status_code(200)
        .await
        .set_response_header(CONTENT_ENCODING, GZIP)
        .await
        .set_response_body(UPLOAD_HTML)
        .await;
}

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

#[post]
pub async fn save(ctx: Context) {
    let file_chunk_data_opt: OptionFileChunkData = get_save_file_chunk_data(&ctx).await;
    if file_chunk_data_opt.is_none() {
        return;
    }
    let file_chunk_data: FileChunkData = file_chunk_data_opt.unwrap_or_default();
    let file_id: &str = file_chunk_data.get_file_id();
    let file_name: &str = file_chunk_data.get_file_name();
    let chunk_index: &usize = file_chunk_data.get_chunk_index();
    let total_chunks: &usize = file_chunk_data.get_total_chunks();
    let base_file_dir: &str = file_chunk_data.get_base_file_dir();
    let chunk_data: Vec<u8> = ctx.get_request_body().await;
    if chunk_data.is_empty() {
        set_common_error_response_body(&ctx, ChunkStrategyError::EmptyChunkData.to_string()).await;
        return;
    }
    let save_upload_dir: String = format!("{UPLOAD_DIR}/{base_file_dir}/{file_id}");
    let upload_strategy: ChunkStrategy = ChunkStrategy::new(
        0,
        &save_upload_dir,
        &file_id,
        &file_name,
        *total_chunks,
        |a, b| format!("{a}.{b}"),
    )
    .unwrap();
    match upload_strategy.save_chunk(&chunk_data, *chunk_index).await {
        Ok(_) => {
            set_common_success_response_body(&ctx, "").await;
        }
        Err(error) => {
            set_common_error_response_body(&ctx, error.to_string()).await;
        }
    }
}

#[post]
pub async fn merge(ctx: Context) {
    let file_chunk_data_opt: OptionFileChunkData = get_merge_file_chunk_data(&ctx).await;
    if file_chunk_data_opt.is_none() {
        return;
    }
    let file_chunk_data: FileChunkData = file_chunk_data_opt.unwrap_or_default();
    let file_id: &str = file_chunk_data.get_file_id();
    let file_name: &str = file_chunk_data.get_file_name();
    let total_chunks: &usize = file_chunk_data.get_total_chunks();
    let base_file_dir: &str = file_chunk_data.get_base_file_dir();
    let save_upload_dir: String = format!("{UPLOAD_DIR}/{base_file_dir}/{file_id}");
    let upload_strategy: ChunkStrategy = ChunkStrategy::new(
        0,
        &save_upload_dir,
        &file_id,
        &file_name,
        *total_chunks,
        |a, b| format!("{a}.{b}"),
    )
    .unwrap();
    let url_encode_dir: String =
        Encode::execute(CHARSETS, &format!("{base_file_dir}/{file_id}")).unwrap_or_default();
    let url_encode_dir_file_name: String =
        Encode::execute(CHARSETS, &file_name).unwrap_or_default();
    let url: String = format!("/{STATIC_ROUTE}/{url_encode_dir}/{url_encode_dir_file_name}");
    match upload_strategy.merge_chunks().await {
        Ok(_) => {
            remove_file_id_map(&file_id).await;
            set_common_success_response_body(&ctx, &url).await;
        }
        Err(error) => {
            set_common_error_response_body(&ctx, error.to_string()).await;
        }
    }
}
