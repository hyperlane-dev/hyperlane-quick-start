use super::*;

impl ServerHook for OpenApiViewRoute {
    #[instrument_trace]
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(
        methods(get, post),
        response_header(CONTENT_TYPE => TEXT_HTML)
    )]
    #[instrument_trace]
    async fn handle(self, ctx: &Context) {
        SwaggerUi::new("/openapi/{file}").url("/openapi/openapi.json", ApiDoc::openapi());
        let res: String =
            RapiDoc::with_openapi("/openapi/openapi.json", ApiDoc::openapi()).to_html();
        ctx.set_response_body(&res).await;
    }
}
