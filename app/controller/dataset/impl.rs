use super::*;

impl ServerHook for DatasetRoute {
    #[instrument_trace]
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(get_method)]
    #[instrument_trace]
    async fn handle(self, ctx: &Context) {
        match DatasetService::fetch_dataset().await {
            Ok(dataset_content) => {
                ctx.set_response_header(
                    CONTENT_TYPE,
                    ContentType::format_content_type_with_charset(TEXT_PLAIN, UTF8),
                )
                .await
                .set_response_body(&dataset_content)
                .await;
            }
            Err(error) => {
                let error_response: ApiResponse<()> =
                    ApiResponse::error_with_code(ResponseCode::InternalError, error);
                ctx.set_response_body(&error_response.to_json_bytes()).await;
            }
        }
    }
}
