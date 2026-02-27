use super::*;

impl ServerHook for OpenApiViewRoute {
    #[instrument_trace]
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        methods(get, post),
        response_header(CONTENT_TYPE => TEXT_HTML)
    )]
    #[instrument_trace]
    async fn handle(self, ctx: &mut Context) {
        SwaggerUi::new("/openapi/{file}").url("/openapi/openapi.json", ApiDoc::openapi());
        let res: String =
            RapiDoc::with_openapi("/openapi/openapi.json", ApiDoc::openapi()).to_html();
        ctx.get_mut_response().set_body(&res);
    }
}
