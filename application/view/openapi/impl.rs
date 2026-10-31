use super::*;

/// Implementation of `OpenApiViewRoute` for `ServerHook`.
impl ServerHook for OpenApiViewRoute {
    /// Creates a new instance.
    ///
    /// # Arguments
    ///
    /// - `&mut Stream` - The .
    /// - `&mut Context` - The .
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    /// Handles one request and writes the response.
    ///
    /// # Arguments
    ///
    /// - `&mut Stream` - The stream.
    /// - `&mut Context` - The request context.
    ///
    /// # Returns
    ///
    /// - `Status` - The handle result.
    #[prologue_macros(
        methods(get, post),
        response_header(CONTENT_TYPE => TEXT_HTML)
    )]
    /// Implements the `handle` operation on [`OpenApiViewRoute`].
    ///
    /// # Arguments
    ///
    /// - `&mut Stream` - Mutable reference to a `Stream` (mutated in place).
    /// - `&mut Context` - Mutable reference to a `Context` (mutated in place).
    ///
    /// # Returns
    ///
    /// - `Status` - A `Status` value.
    #[instrument_trace]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        SwaggerUi::new(ROUTE_OPENAPI_FILE).url(PATH_OPENAPI_SPEC, ApiDoc::openapi());
        let res: String = RapiDoc::with_openapi(PATH_OPENAPI_SPEC, ApiDoc::openapi()).to_html();
        ctx.get_mut_response().set_body(&res);
        Status::Continue
    }
}
