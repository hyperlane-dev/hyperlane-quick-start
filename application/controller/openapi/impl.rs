use super::*;

/// Implementation of `OpenApiRoute` for `ServerHook`.
impl ServerHook for OpenApiRoute {
    #[instrument_trace]
    /// Builds the `ServerHook` state for the `OpenApiRoute` route.
    ///
    /// # Arguments
    ///
    /// - `&mut Stream` - The inbound request stream.
    /// - `&mut Context` - The mutable request and response context.
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        methods(get, post),
        response_status_code(200),
        response_header(CONTENT_TYPE => APPLICATION_JSON)
    )]
    #[instrument_trace]
    /// Handles the `OpenApiRoute` route request and writes the JSON
    /// response envelope into the request context.
    ///
    /// # Arguments
    ///
    /// - `Self` - The route handler instance.
    /// - `&mut Stream` - The inbound request stream.
    /// - `&mut Context` - The mutable request and response context.
    ///
    /// # Returns
    ///
    /// - `Status` - The hook status that tells the server how to continue.
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        if let Ok(json_data) = ApiDoc::openapi().to_json() {
            ctx.get_mut_response().set_body(&json_data);
        }
        Status::Continue
    }
}
