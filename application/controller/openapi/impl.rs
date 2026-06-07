use super::*;

/// Implementation of `OpenApiRoute` for `ServerHook`.
impl ServerHook for OpenApiRoute {
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(methods(get, post), response_status_code(200))]
    #[instrument_trace]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        if let Ok(json_data) = ApiDoc::openapi().to_json() {
            ctx.get_mut_response().set_body(&json_data);
        }
        Status::Continue
    }
}
