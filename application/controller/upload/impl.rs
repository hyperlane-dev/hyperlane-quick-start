use super::*;

/// Implementation of `RegisterRoute` for `ServerHook`.
impl ServerHook for RegisterRoute {
    #[instrument_trace]
    /// Builds the `ServerHook` state for the `RegisterRoute` route.
    ///
    /// # Arguments
    ///
    /// - `&mut Stream` - The inbound request stream.
    /// - `&mut Context` - The mutable request and response context.
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[is_post_method]
    #[instrument_trace]
    /// Handles the `RegisterRoute` route request and writes the JSON
    /// response envelope into the request context.
    ///
    /// The request carries a multipart body: the identifier of the session
    /// and the number of chunks arrive as text fields, and the name of the
    /// uploaded file arrives with the file part of the body.
    ///
    /// # Arguments
    ///
    /// - `Self` - The route handler instance.
    /// - `&mut Stream` - The inbound request stream.
    /// - `&mut Context` - The mutable request and response context.
    ///
    /// # Returns
    ///
    /// - `Status` - The hook status that tells the server how to continue.
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        let file_chunk_data_opt: Option<FileChunkData> =
            UploadService::get_register_file_chunk_data(ctx).await;
        if file_chunk_data_opt.is_none() {
            return Status::Continue;
        }
        let file_chunk_data: FileChunkData = file_chunk_data_opt.unwrap_or_default();
        UploadService::add_file_id_map(&file_chunk_data).await;
        UploadService::set_common_success_response_body(ctx, EMPTY_STR).await;
        Status::Continue
    }
}

/// Implementation of `SaveRoute` for `ServerHook`.
impl ServerHook for SaveRoute {
    #[instrument_trace]
    /// Builds the `ServerHook` state for the `SaveRoute` route.
    ///
    /// # Arguments
    ///
    /// - `&mut Stream` - The inbound request stream.
    /// - `&mut Context` - The mutable request and response context.
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(is_post_method)]
    #[instrument_trace]
    /// Handles the `SaveRoute` route request and writes the JSON
    /// response envelope into the request context.
    ///
    /// The request carries a multipart body: the identifier of the session
    /// and the index of the chunk arrive as text fields, and the bytes of the
    /// chunk arrive with the file part of the body.
    ///
    /// # Arguments
    ///
    /// - `Self` - The route handler instance.
    /// - `&mut Stream` - The inbound request stream.
    /// - `&mut Context` - The mutable request and response context.
    ///
    /// # Returns
    ///
    /// - `Status` - The hook status that tells the server how to continue.
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        let save_data_opt: Option<(FileChunkData, RequestBody)> =
            UploadService::get_save_file_chunk_data(ctx).await;
        if save_data_opt.is_none() {
            return Status::Continue;
        }
        let (file_chunk_data, chunk_data): (FileChunkData, RequestBody) =
            save_data_opt.unwrap_or_default();
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
    /// Builds the `ServerHook` state for the `MergeRoute` route.
    ///
    /// # Arguments
    ///
    /// - `&mut Stream` - The inbound request stream.
    /// - `&mut Context` - The mutable request and response context.
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(is_post_method)]
    #[instrument_trace]
    /// Handles the `MergeRoute` route request and writes the JSON
    /// response envelope into the request context.
    ///
    /// The request carries a multipart body whose text field holds the
    /// identifier of the session whose chunks are merged.
    ///
    /// # Arguments
    ///
    /// - `Self` - The route handler instance.
    /// - `&mut Stream` - The inbound request stream.
    /// - `&mut Context` - The mutable request and response context.
    ///
    /// # Returns
    ///
    /// - `Status` - The hook status that tells the server how to continue.
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        let file_id_opt: Option<String> = UploadService::get_merge_file_id(ctx).await;
        if file_id_opt.is_none() {
            return Status::Continue;
        }
        let file_id: String = file_id_opt.unwrap_or_default();
        let file_chunk_data_opt: Option<FileChunkData> =
            UploadService::get_merge_file_chunk_data(file_id).await;
        if file_chunk_data_opt.is_none() {
            UploadService::set_common_error_response_body(
                ctx,
                ERROR_UPLOAD_NOT_REGISTERED.to_string(),
            )
            .await;
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
