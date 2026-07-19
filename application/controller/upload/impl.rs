use super::*;

/// Implementation of `RegisterRoute` for `ServerHook`.
impl ServerHook for RegisterRoute {
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[is_post_method]
    #[instrument_trace]
    async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
        let file_chunk_data_opt: Option<FileChunkData> =
            UploadService::get_register_file_chunk_data(stream, ctx).await;
        if file_chunk_data_opt.is_none() {
            return Status::Continue;
        }
        let file_chunk_data: FileChunkData = file_chunk_data_opt.unwrap_or_default();
        UploadService::add_file_id_map(&file_chunk_data).await;
        UploadService::set_common_success_response_body(ctx, "").await;
        Status::Continue
    }
}

/// Implementation of `SaveRoute` for `ServerHook`.
impl ServerHook for SaveRoute {
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(is_post_method)]
    #[try_get_request_header(HEADER_X_FILE_ID => file_id_opt)]
    #[try_get_request_header(HEADER_X_CHUNK_INDEX => chunk_index_opt)]
    #[instrument_trace]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        let file_chunk_data_opt: Option<FileChunkData> =
            UploadService::get_save_file_chunk_data(ctx, file_id_opt, chunk_index_opt).await;
        if file_chunk_data_opt.is_none() {
            return Status::Continue;
        }
        let file_chunk_data: FileChunkData = file_chunk_data_opt.unwrap_or_default();
        let chunk_data: RequestBody = ctx.get_request().get_body().clone();
        match UploadService::save_file_chunk(&file_chunk_data, chunk_data).await {
            Ok(save_upload_dir) => {
                ctx.get_mut_response()
                    .set_header(HEADER_SAVE_UPLOAD_DIR, save_upload_dir);
                UploadService::set_common_success_response_body(ctx, EMPTY_STR).await;
            }
            Err(error) => {
                UploadService::set_common_error_response_body(ctx, error).await;
            }
        }
        Status::Continue
    }
}

/// Implementation of `MergeRoute` for `ServerHook`.
impl ServerHook for MergeRoute {
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(is_post_method)]
    #[try_get_request_header(HEADER_X_FILE_ID => file_id_opt)]
    #[instrument_trace]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        let file_chunk_data_opt: Option<FileChunkData> =
            UploadService::get_merge_file_chunk_data(ctx, file_id_opt).await;
        if file_chunk_data_opt.is_none() {
            return Status::Continue;
        }
        let file_chunk_data: FileChunkData = file_chunk_data_opt.unwrap_or_default();
        match UploadService::merge_file_chunks(&file_chunk_data).await {
            Ok((save_upload_dir, url)) => {
                ctx.get_mut_response()
                    .set_header(HEADER_SAVE_UPLOAD_DIR, save_upload_dir);
                UploadService::set_common_success_response_body(ctx, &url).await;
            }
            Err(error) => {
                UploadService::set_common_error_response_body(ctx, error).await;
            }
        }
        Status::Continue
    }
}
