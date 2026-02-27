use super::*;

impl ServerHook for DatasetRoute {
    #[instrument_trace]
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(get_method)]
    #[instrument_trace]
    async fn handle(self, ctx: &mut Context) {
        match DatasetService::fetch_dataset().await {
            Ok(dataset_content) => {
                ctx.get_mut_response()
                    .set_header(
                        CONTENT_TYPE,
                        ContentType::format_content_type_with_charset(TEXT_PLAIN, UTF8),
                    )
                    .set_body(&dataset_content);
            }
            Err(error) => {
                let error_response: ApiResponse<()> =
                    ApiResponse::error_with_code(ResponseCode::InternalError, error);
                ctx.get_mut_response()
                    .set_body(error_response.to_json_bytes());
            }
        }
    }
}
