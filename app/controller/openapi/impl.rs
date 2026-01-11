use super::*;

impl ServerHook for OpenApiRoute {
    async fn new(_ctx: &Context) -> Self {
        trace!("OpenApiRoute new");
        Self
    }

    #[prologue_macros(methods(get, post), response_status_code(200))]
    async fn handle(self, ctx: &Context) {
        trace!("OpenApiRoute handle");
        if let Ok(json_data) = ApiDoc::openapi().to_json() {
            ctx.set_response_body(&json_data).await;
        }
    }
}
