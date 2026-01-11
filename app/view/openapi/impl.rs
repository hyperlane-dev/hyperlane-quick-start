use super::*;

impl ServerHook for OpenApiViewRoute {
    async fn new(_ctx: &Context) -> Self {
        trace!("OpenApiViewRoute new");
        Self
    }

    #[prologue_macros(
        methods(get, post),
        response_header(CONTENT_TYPE => TEXT_HTML)
    )]
    async fn handle(self, ctx: &Context) {
        trace!("OpenApiViewRoute handle");
        SwaggerUi::new("/openapi/{file}").url("/openapi/openapi.json", ApiDoc::openapi());
        let res: String =
            RapiDoc::with_openapi("/openapi/openapi.json", ApiDoc::openapi()).to_html();
        ctx.set_response_body(&res).await;
    }
}
