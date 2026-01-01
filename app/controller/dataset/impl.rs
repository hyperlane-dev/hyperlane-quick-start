use super::*;

impl ServerHook for DatasetRoute {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(get)]
    async fn handle(self, ctx: &Context) {
        let dataset_url = "https://raw.githubusercontent.com/hyperlane-dev/hyperlane-ai/refs/heads/master/dataset/dataset.md";
        let mut request_builder: BoxAsyncRequestTrait = RequestBuilder::new()
            .get(dataset_url)
            .redirect()
            .http1_1_only()
            .build_async();
        match request_builder.send().await {
            Ok(response) => {
                let response_text: String = response.text().get_body();
                ctx.set_response_body(&response_text).await;
            }
            Err(error) => {
                let error_message = format!("Failed to fetch dataset: {}", error);
                let error_response: ApiResponse<()> =
                    ApiResponse::error_with_code(ResponseCode::InternalError, error_message);
                ctx.set_response_body(&error_response.to_json_bytes()).await;
            }
        }
    }
}
