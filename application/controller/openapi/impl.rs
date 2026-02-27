use super::*;

impl ServerHook for OpenApiRoute {
    #[instrument_trace]
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(methods(get, post), response_status_code(200))]
    #[instrument_trace]
    async fn handle(self, ctx: &mut Context) {
        if let Ok(json_data) = ApiDoc::openapi().to_json() {
            ctx.get_mut_response().set_body(&json_data);
        }
    }
}
