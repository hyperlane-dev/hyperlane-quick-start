use super::*;

/// Implementation of `TemplatesRoute` for `ServerHook`.
impl ServerHook for TemplatesRoute {
    #[instrument_trace]
    /// Builds the `ServerHook` state for the `TemplatesRoute` route.
    ///
    /// # Arguments
    ///
    /// - `&mut Stream` - The inbound request stream.
    /// - `&mut Context` - The mutable request and response context.
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        is_get_method,
        response_body(TEMPLATES_INDEX_HTML.replace(TEMPLATE_PLACEHOLDER_TIME, &time()))
    )]
    #[instrument_trace]
    /// Handles the `TemplatesRoute` route request and writes the JSON
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
        Status::Continue
    }
}
