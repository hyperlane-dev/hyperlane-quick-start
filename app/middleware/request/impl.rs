use super::*;

impl ServerHook for HttpRequestMiddleware {
    #[instrument_trace]
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(
        reject(ctx.get_request_is_http().await),
        send,
    )]
    #[instrument_trace]
    async fn handle(self, ctx: &Context) {
        ctx.closed().await;
    }
}

impl ServerHook for CrossMiddleware {
    #[instrument_trace]
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[response_version(HttpVersion::Http1_1)]
    #[response_header(ACCESS_CONTROL_ALLOW_ORIGIN => WILDCARD_ANY)]
    #[response_header(ACCESS_CONTROL_ALLOW_METHODS => ALL_METHODS)]
    #[response_header(ACCESS_CONTROL_ALLOW_HEADERS => WILDCARD_ANY)]
    #[instrument_trace]
    async fn handle(self, ctx: &Context) {}
}

impl ServerHook for ResponseHeaderMiddleware {
    #[instrument_trace]
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[response_header(DATE => gmt())]
    #[response_header(SERVER => HYPERLANE)]
    #[response_header(CONNECTION => KEEP_ALIVE)]
    #[epilogue_macros(
        response_header(CONTENT_TYPE => content_type),
        response_header("SocketAddr" => socket_addr_string)
    )]
    #[instrument_trace]
    async fn handle(self, ctx: &Context) {
        let socket_addr_string: String = ctx.get_socket_addr_string().await;
        let content_type: String = ContentType::format_content_type_with_charset(TEXT_HTML, UTF8);
    }
}

impl ServerHook for ResponseStatusCodeMiddleware {
    #[instrument_trace]
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[response_status_code(200)]
    #[instrument_trace]
    async fn handle(self, ctx: &Context) {}
}

impl ServerHook for ResponseBodyMiddleware {
    #[instrument_trace]
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[response_body(include_str!("../../../resources/templates/index/index.html").replace("{{ time }}", &time()))]
    #[instrument_trace]
    async fn handle(self, ctx: &Context) {}
}

impl ServerHook for OptionMethodMiddleware {
    #[instrument_trace]
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(
        filter(ctx.get_request_is_options().await),
        send
    )]
    #[instrument_trace]
    async fn handle(self, ctx: &Context) {
        ctx.aborted().await;
    }
}

impl ServerHook for UpgradeMiddleware {
    #[instrument_trace]
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(
        ws,
        response_version(HttpVersion::Http1_1),
        response_status_code(101),
        response_body(&vec![]),
        response_header(UPGRADE => WEBSOCKET),
        response_header(CONNECTION => UPGRADE),
        response_header(SEC_WEBSOCKET_ACCEPT => WebSocketFrame::generate_accept_key(ctx.try_get_request_header_back(SEC_WEBSOCKET_KEY).await.unwrap())),
        send
    )]
    #[instrument_trace]
    async fn handle(self, ctx: &Context) {}
}
