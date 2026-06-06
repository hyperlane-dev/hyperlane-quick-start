use super::*;

/// Implementation of `ServerHook` for `SendMiddleware`, sending the response body to the client.
impl ServerHook for SendMiddleware {
    /// Creates a new `SendMiddleware` instance from the incoming stream and context.
    ///
    /// # Arguments
    ///
    /// - `&mut Stream`: The incoming connection stream.
    /// - `&mut Context`: The request context.
    ///
    /// # Returns
    ///
    /// - `SendMiddleware`: The newly created send middleware handler.
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    /// Sends the response body to the client, rejecting WebSocket upgrade requests.
    ///
    /// # Arguments
    ///
    /// - `Self`: The send middleware handler.
    /// - `&mut Stream`: The incoming connection stream.
    /// - `&mut Context`: The request context.
    ///
    /// # Returns
    ///
    /// - `Status::Continue`: Always returns continue after attempting to send.
    #[prologue_macros(
        reject(ctx.get_request().is_ws_upgrade_type()),
        try_send
    )]
    #[instrument_trace]
    async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
        Status::Continue
    }
}

/// Implementation of `ServerHook` for `LogMiddleware`, logging request and response details.
impl ServerHook for LogMiddleware {
    /// Creates a new `LogMiddleware` instance from the incoming stream and context.
    ///
    /// # Arguments
    ///
    /// - `&mut Stream`: The incoming connection stream.
    /// - `&mut Context`: The request context.
    ///
    /// # Returns
    ///
    /// - `LogMiddleware`: The newly created log middleware handler.
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    /// Logs request and response details for debugging and monitoring purposes.
    ///
    /// # Arguments
    ///
    /// - `Self`: The log middleware handler.
    /// - `&mut Stream`: The incoming connection stream.
    /// - `&mut Context`: The request context.
    ///
    /// # Returns
    ///
    /// - `Status::Continue`: Always returns continue after logging.
    #[instrument_trace]
    async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status {
        let request_json: String = get_request_json(ctx).await;
        let response_json: String = get_response_json(ctx).await;
        info!("{request_json}");
        info!("{response_json}");
        Status::Continue
    }
}
