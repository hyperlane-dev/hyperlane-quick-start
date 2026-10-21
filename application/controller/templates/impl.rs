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
      methods(get, post),
      response_status_code(302),
      response_header(LOCATION => TEMPLATES_VIEW_REDIRECT_PATH)
    )]
    #[instrument_trace]
    /// Handles the `TemplatesRoute` route request and redirects to the
    /// landing page.
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
    async fn handle(self, _stream: &mut Stream, _ctx: &mut Context) -> Status {
        Status::Continue
    }
}
