use super::*;

/// Implementation of `ServerHook` for `FaviconRoute`, redirecting favicon requests to the logo image URL.
impl ServerHook for FaviconRoute {
    /// Creates a new `FaviconRoute` instance from the incoming stream and context.
    ///
    /// # Arguments
    ///
    /// - `&mut Stream`: The incoming connection stream.
    /// - `&mut Context`: The request context.
    ///
    /// # Returns
    ///
    /// - `FaviconRoute`: The newly created favicon route handler.
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    /// Handles the favicon request by redirecting to the logo image URL with a 302 status.
    ///
    /// # Arguments
    ///
    /// - `Self`: The favicon route handler.
    /// - `&mut Stream`: The incoming connection stream.
    /// - `&mut Context`: The request context.
    ///
    /// # Returns
    ///
    /// - `Status::Continue`: Always returns continue after setting redirect headers.
    #[prologue_macros(
        is_get_method,
        response_status_code(302),
        response_header(LOCATION => LOGO_IMG_URL)
    )]
    #[instrument_trace]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        Status::Continue
    }
}
