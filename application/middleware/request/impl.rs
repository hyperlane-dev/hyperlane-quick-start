use super::*;

/// Implementation of `ServerHook` for `HttpRequestMiddleware`, rejecting non-HTTP protocol requests.
impl ServerHook for HttpRequestMiddleware {
    /// Creates a new `HttpRequestMiddleware` instance from the incoming stream and context.
    ///
    /// # Arguments
    ///
    /// - `&mut Stream`: The incoming connection stream.
    /// - `&mut Context`: The request context.
    ///
    /// # Returns
    ///
    /// - `HttpRequestMiddleware`: The newly created HTTP request middleware handler.
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    /// Rejects non-HTTP protocol requests by closing the stream.
    ///
    /// # Arguments
    ///
    /// - `Self`: The HTTP request middleware handler.
    /// - `&mut Stream`: The incoming connection stream.
    /// - `&mut Context`: The request context.
    ///
    /// # Returns
    ///
    /// - `Status::Continue`: Always returns continue after closing the stream.
    #[prologue_macros(
        reject(ctx.get_request().get_version().is_http()),
        send,
    )]
    #[instrument_trace]
    async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
        stream.set_closed(true);
        Status::Continue
    }
}

/// Implementation of `ServerHook` for `CrossMiddleware`, adding CORS cross-origin headers to the response.
impl ServerHook for CrossMiddleware {
    /// Creates a new `CrossMiddleware` instance from the incoming stream and context.
    ///
    /// # Arguments
    ///
    /// - `&mut Stream`: The incoming connection stream.
    /// - `&mut Context`: The request context.
    ///
    /// # Returns
    ///
    /// - `CrossMiddleware`: The newly created CORS middleware handler.
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    /// Adds CORS cross-origin headers to the response, allowing all origins, methods, and headers.
    ///
    /// # Arguments
    ///
    /// - `Self`: The CORS middleware handler.
    /// - `&mut Stream`: The incoming connection stream.
    /// - `&mut Context`: The request context.
    ///
    /// # Returns
    ///
    /// - `Status::Continue`: Always returns continue after setting CORS headers.
    #[response_version(HttpVersion::Http1_1)]
    #[response_header(ACCESS_CONTROL_ALLOW_ORIGIN => WILDCARD_ANY)]
    #[response_header(ACCESS_CONTROL_ALLOW_METHODS => ALL_METHODS)]
    #[response_header(ACCESS_CONTROL_ALLOW_HEADERS => WILDCARD_ANY)]
    #[instrument_trace]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        Status::Continue
    }
}

/// Implementation of `ServerHook` for `ResponseHeaderMiddleware`, setting default response headers.
impl ServerHook for ResponseHeaderMiddleware {
    /// Creates a new `ResponseHeaderMiddleware` instance from the incoming stream and context.
    ///
    /// # Arguments
    ///
    /// - `&mut Stream`: The incoming connection stream.
    /// - `&mut Context`: The request context.
    ///
    /// # Returns
    ///
    /// - `ResponseHeaderMiddleware`: The newly created response header middleware handler.
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    /// Sets default response headers including date, server, connection, trace, and content type.
    ///
    /// # Arguments
    ///
    /// - `Self`: The response header middleware handler.
    /// - `&mut Stream`: The incoming connection stream.
    /// - `&mut Context`: The request context.
    ///
    /// # Returns
    ///
    /// - `Status::Continue`: Always returns continue after setting response headers.
    #[response_header(DATE => gmt())]
    #[response_header(SERVER => HYPERLANE)]
    #[response_header(CONNECTION => KEEP_ALIVE)]
    #[response_header(TRACE => uuid::Uuid::new_v4().to_string())]
    #[epilogue_macros(response_header(CONTENT_TYPE => content_type))]
    #[instrument_trace]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        let content_type: String = ContentType::format_content_type_with_charset(TEXT_HTML, UTF8);
        Status::Continue
    }
}

/// Implementation of `ServerHook` for `ResponseStatusCodeMiddleware`, setting the default response status code.
impl ServerHook for ResponseStatusCodeMiddleware {
    /// Creates a new `ResponseStatusCodeMiddleware` instance from the incoming stream and context.
    ///
    /// # Arguments
    ///
    /// - `&mut Stream`: The incoming connection stream.
    /// - `&mut Context`: The request context.
    ///
    /// # Returns
    ///
    /// - `ResponseStatusCodeMiddleware`: The newly created response status code middleware handler.
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    /// Sets the default HTTP response status code to 200 OK.
    ///
    /// # Arguments
    ///
    /// - `Self`: The response status code middleware handler.
    /// - `&mut Stream`: The incoming connection stream.
    /// - `&mut Context`: The request context.
    ///
    /// # Returns
    ///
    /// - `Status::Continue`: Always returns continue after setting the status code.
    #[response_status_code(200)]
    #[instrument_trace]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        Status::Continue
    }
}

/// Implementation of `ServerHook` for `OptionMethodMiddleware`, rejecting HTTP OPTIONS preflight requests.
impl ServerHook for OptionMethodMiddleware {
    /// Creates a new `OptionMethodMiddleware` instance from the incoming stream and context.
    ///
    /// # Arguments
    ///
    /// - `&mut Stream`: The incoming connection stream.
    /// - `&mut Context`: The request context.
    ///
    /// # Returns
    ///
    /// - `OptionMethodMiddleware`: The newly created OPTIONS method middleware handler.
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    /// Rejects HTTP OPTIONS preflight requests by filtering and sending an empty response.
    ///
    /// # Arguments
    ///
    /// - `Self`: The OPTIONS method middleware handler.
    /// - `&mut Stream`: The incoming connection stream.
    /// - `&mut Context`: The request context.
    ///
    /// # Returns
    ///
    /// - `Status::Reject`: Always returns reject for OPTIONS requests.
    #[prologue_macros(
        filter(ctx.get_request().get_method().is_options()),
        send
    )]
    #[instrument_trace]
    async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
        Status::Reject
    }
}

/// Implementation of `ServerHook` for `UpgradeMiddleware`, handling WebSocket upgrade handshakes.
impl ServerHook for UpgradeMiddleware {
    /// Creates a new `UpgradeMiddleware` instance from the incoming stream and context.
    ///
    /// # Arguments
    ///
    /// - `&mut Stream`: The incoming connection stream.
    /// - `&mut Context`: The request context.
    ///
    /// # Returns
    ///
    /// - `UpgradeMiddleware`: The newly created upgrade middleware handler.
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    /// Handles WebSocket upgrade handshakes by sending the appropriate 101 Switching Protocols response.
    ///
    /// # Arguments
    ///
    /// - `Self`: The upgrade middleware handler.
    /// - `&mut Stream`: The incoming connection stream.
    /// - `&mut Context`: The request context.
    ///
    /// # Returns
    ///
    /// - `Status::Continue`: Always returns continue after handling the upgrade.
    #[prologue_macros(
        is_ws_upgrade_type,
        response_version(HttpVersion::Http1_1),
        response_status_code(101),
        response_body(&vec![]),
        response_header(UPGRADE => WEBSOCKET),
        response_header(CONNECTION => UPGRADE),
        response_header(SEC_WEBSOCKET_ACCEPT => WebSocketFrame::generate_accept_key(ctx.get_request().get_header_back(SEC_WEBSOCKET_KEY))),
        send
    )]
    #[instrument_trace]
    async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
        Status::Continue
    }
}
