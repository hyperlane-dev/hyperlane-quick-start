use super::*;

/// Implementation of `TraceViewRoute` for `ServerHook`.
impl ServerHook for TraceViewRoute {
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
    #[prologue_macros(methods(get, post))]
    #[instrument_trace]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        serve_page(ctx, TRACE_VIEW_PAGE_NAME)
    }
}
