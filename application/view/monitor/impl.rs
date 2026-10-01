use super::*;

/// Implementation of `MonitorViewRoute` for `ServerHook`.
impl ServerHook for MonitorViewRoute {
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
        response_status_code(302),
        response_header(LOCATION => MONITOR_VIEW_REDIRECT_PATH)
    )]
    #[instrument_trace]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        Status::Continue
    }
}
