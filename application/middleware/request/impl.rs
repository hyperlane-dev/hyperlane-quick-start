use super::*;

impl ServerHook for HttpRequestMiddleware {
    #[instrument_trace]
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        reject(ctx.get_request().get_version().is_http()),
        send,
    )]
    #[instrument_trace]
    async fn handle(self, ctx: &mut Context) {
        ctx.set_closed(true);
    }
}

impl ServerHook for CrossMiddleware {
    #[instrument_trace]
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[response_version(HttpVersion::Http1_1)]
    #[response_header(ACCESS_CONTROL_ALLOW_ORIGIN => WILDCARD_ANY)]
    #[response_header(ACCESS_CONTROL_ALLOW_METHODS => ALL_METHODS)]
    #[response_header(ACCESS_CONTROL_ALLOW_HEADERS => WILDCARD_ANY)]
    #[instrument_trace]
    async fn handle(self, ctx: &mut Context) {}
}

impl ServerHook for ResponseHeaderMiddleware {
    #[instrument_trace]
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[response_header(DATE => gmt())]
    #[response_header(SERVER => HYPERLANE)]
    #[response_header(CONNECTION => KEEP_ALIVE)]
    #[response_header(TRACE => uuid::Uuid::new_v4().to_string())]
    #[epilogue_macros(
        response_header(CONTENT_TYPE => content_type),
        response_header("SocketAddr" => socket_addr_string)
    )]
    #[instrument_trace]
    async fn handle(self, ctx: &mut Context) {
        let socket_addr_string: String = ctx.get_socket_addr_string().await;
        let content_type: String = ContentType::format_content_type_with_charset(TEXT_HTML, UTF8);
    }
}

impl ServerHook for ResponseStatusCodeMiddleware {
    #[instrument_trace]
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[response_status_code(200)]
    #[instrument_trace]
    async fn handle(self, ctx: &mut Context) {}
}

impl ServerHook for ResponseBodyMiddleware {
    #[instrument_trace]
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[epilogue_macros(response_body(TEMPLATES_INDEX_HTML.replace("{{ time }}", &time())))]
    #[instrument_trace]
    async fn handle(self, ctx: &mut Context) {}
}

impl ServerHook for OptionMethodMiddleware {
    #[instrument_trace]
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        filter(ctx.get_request().get_method().is_options()),
        send
    )]
    #[instrument_trace]
    async fn handle(self, ctx: &mut Context) {
        ctx.set_aborted(true);
    }
}

impl ServerHook for UpgradeMiddleware {
    #[instrument_trace]
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        ws_upgrade_type,
        response_version(HttpVersion::Http1_1),
        response_status_code(101),
        response_body(&vec![]),
        response_header(UPGRADE => WEBSOCKET),
        response_header(CONNECTION => UPGRADE),
        response_header(SEC_WEBSOCKET_ACCEPT => WebSocketFrame::generate_accept_key(ctx.get_request().get_header_back(SEC_WEBSOCKET_KEY))),
        send
    )]
    #[instrument_trace]
    async fn handle(self, ctx: &mut Context) {}
}
