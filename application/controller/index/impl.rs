use super::*;

/// Implementation of `IndexRoute` for `ServerHook`.
impl ServerHook for IndexRoute {
    #[instrument_trace]
    /// Builds the `ServerHook` state for the `IndexRoute` route.
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
      response_header(LOCATION => INDEX_REDIRECT_URL)
    )]
    #[instrument_trace]
    /// Handles the `IndexRoute` route request and writes the JSON
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
