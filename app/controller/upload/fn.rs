use super::*;

#[methods(get, post)]
#[utoipa::path(
    get,
    post,
    path = "/static/{upload_dir}/{upload_file}",   
    responses(
        (status = 200, description = "静态资源", body = String)
    )
)]
#[route_param(UPLOAD_DIR_KEY => dir_opt)]
#[route_param(UPLOAD_FILE_KEY => file_opt)]
#[response_status_code(200)]
#[response_header(CACHE_CONTROL => "public, max-age=31536000, immutable")]
#[response_header(EXPIRES => "Wed, 1 Apr 8888 00:00:00 GMT")]
pub async fn static_file(ctx: Context) {
    let dir: String = dir_opt.unwrap_or_default();
    let file: String = file_opt.unwrap_or_default();
    match serve_static_file(&dir, &file).await {
        Ok((data, content_type)) => {
            ctx.set_response_header(CONTENT_TYPE, content_type)
                .await
                .set_response_body(data)
                .await;
        }
        Err(_) => {
            return;
        }
    }
}

#[methods(get, post)]
#[utoipa::path(
    get,
    post,
    path = "/upload/index.html",   
    responses(
        (status = 200, description = "文件分块上传前端界面", body = String)
    )
)]
#[response_status_code(200)]
#[response_header(CONTENT_ENCODING => GZIP)]
#[response_body(UPLOAD_HTML)]
pub async fn html(ctx: Context) {}

#[post]
#[utoipa::path(
    post,
    path = "/api/upload/register",   
    responses(
        (status = 200, description = "文件分块上传-注册接口", body = UploadResponse)
    )
)]
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
#[utoipa::path(
    post,
    path = "/api/upload/save",   
    responses(
        (status = 200, description = "文件分块上传-保存接口", body = UploadResponse)
    )
)]
pub async fn save(ctx: Context) {
    let file_chunk_data_opt: OptionFileChunkData = get_save_file_chunk_data(&ctx).await;
    if file_chunk_data_opt.is_none() {
        return;
    }
    let file_chunk_data: FileChunkData = file_chunk_data_opt.unwrap_or_default();
    let chunk_data: Vec<u8> = ctx.get_request_body().await;
    match save_file_chunk(&file_chunk_data, chunk_data).await {
        Ok(save_upload_dir) => {
            ctx.set_response_header("file", save_upload_dir).await;
            set_common_success_response_body(&ctx, "").await;
        }
        Err(error) => {
            set_common_error_response_body(&ctx, error).await;
        }
    }
}

#[post]
#[utoipa::path(
    post,
    path = "/api/upload/merge",   
    responses(
        (status = 200, description = "文件分块上传-合并接口", body = UploadResponse)
    )
)]
pub async fn merge(ctx: Context) {
    let file_chunk_data_opt: OptionFileChunkData = get_merge_file_chunk_data(&ctx).await;
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
