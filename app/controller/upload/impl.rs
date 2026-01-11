use super::*;

impl ServerHook for RegisterRoute {
    async fn new(_ctx: &Context) -> Self {
        trace!("UploadRegisterRoute new");
        Self
    }

    #[prologue_macros(post)]
    async fn handle(self, ctx: &Context) {
        trace!("RegisterRoute handle");
        let file_chunk_data_opt: OptionFileChunkData =
            UploadService::get_register_file_chunk_data(ctx).await;
        if file_chunk_data_opt.is_none() {
            return;
        }
        let file_chunk_data: FileChunkData = file_chunk_data_opt.unwrap_or_default();
        UploadService::add_file_id_map(&file_chunk_data).await;
        UploadService::set_common_success_response_body(ctx, "").await;
    }
}

impl ServerHook for SaveRoute {
    async fn new(_ctx: &Context) -> Self {
        trace!("SaveRoute new");
        Self
    }

    #[prologue_macros(
        post,
        request_header_option(CHUNKIFY_FILE_ID_HEADER => file_id_opt),
        request_header_option(CHUNKIFY_CHUNK_INDEX_HEADER => chunk_index_opt)
    )]
    async fn handle(self, ctx: &Context) {
        trace!("SaveRoute handle");
        let file_chunk_data_opt: OptionFileChunkData =
            UploadService::get_save_file_chunk_data(ctx, file_id_opt, chunk_index_opt).await;
        if file_chunk_data_opt.is_none() {
            return;
        }
        let file_chunk_data: FileChunkData = file_chunk_data_opt.unwrap_or_default();
        let chunk_data: Vec<u8> = ctx.get_request_body().await;
        match UploadService::save_file_chunk(&file_chunk_data, chunk_data).await {
            Ok(save_upload_dir) => {
                ctx.set_response_header("save_upload_dir", save_upload_dir)
                    .await;
                UploadService::set_common_success_response_body(ctx, EMPTY_STR).await;
            }
            Err(error) => {
                UploadService::set_common_error_response_body(ctx, error).await;
            }
        }
    }
}

impl ServerHook for MergeRoute {
    async fn new(_ctx: &Context) -> Self {
        trace!("MergeRoute new");
        Self
    }

    #[prologue_macros(
        post,
        request_header_option(CHUNKIFY_FILE_ID_HEADER => file_id_opt)
    )]
    async fn handle(self, ctx: &Context) {
        trace!("MergeRoute handle");
        let file_chunk_data_opt: OptionFileChunkData =
            UploadService::get_merge_file_chunk_data(ctx, file_id_opt).await;
        if file_chunk_data_opt.is_none() {
            return;
        }
        let file_chunk_data: FileChunkData = file_chunk_data_opt.unwrap_or_default();
        match UploadService::merge_file_chunks(&file_chunk_data).await {
            Ok((save_upload_dir, url)) => {
                ctx.set_response_header("save_upload_dir", save_upload_dir)
                    .await;
                UploadService::set_common_success_response_body(ctx, &url).await;
            }
            Err(error) => {
                UploadService::set_common_error_response_body(ctx, error).await;
            }
        }
    }
}
