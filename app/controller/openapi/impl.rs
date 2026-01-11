use super::*;

impl ServerHook for OpenApiRoute {
    #[instrument_trace]
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(methods(get, post), response_status_code(200))]
    #[instrument_trace]
    async fn handle(self, ctx: &Context) {
        if let Ok(json_data) = ApiDoc::openapi().to_json() {
            ctx.set_response_body(&json_data).await;
        }
    }
}
