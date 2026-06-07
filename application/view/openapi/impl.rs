use super::*;

/// Implementation of `OpenApiViewRoute` for `ServerHook`.
impl ServerHook for OpenApiViewRoute {
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        methods(get, post),
        response_header(CONTENT_TYPE => TEXT_HTML)
    )]
    #[instrument_trace]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        SwaggerUi::new("/openapi/{file}").url("/openapi/openapi.json", ApiDoc::openapi());
        let res: String =
            RapiDoc::with_openapi("/openapi/openapi.json", ApiDoc::openapi()).to_html();
        ctx.get_mut_response().set_body(&res);
        Status::Continue
    }
}
