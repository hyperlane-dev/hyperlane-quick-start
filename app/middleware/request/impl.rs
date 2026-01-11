use super::*;

impl ServerHook for HttpRequestMiddleware {
    async fn new(_ctx: &Context) -> Self {
        trace!("HttpRequestMiddleware new");
        Self
    }

    #[prologue_macros(
        reject(ctx.get_request_is_http().await),
        send,
    )]
    async fn handle(self, ctx: &Context) {
        trace!("HttpRequestMiddleware handle");
        ctx.closed().await;
    }
}

impl ServerHook for CrossMiddleware {
    async fn new(_ctx: &Context) -> Self {
        trace!("CrossMiddleware new");
        Self
    }

    #[response_version(HttpVersion::Http1_1)]
    #[response_header(ACCESS_CONTROL_ALLOW_ORIGIN => WILDCARD_ANY)]
    #[response_header(ACCESS_CONTROL_ALLOW_METHODS => ALL_METHODS)]
    #[response_header(ACCESS_CONTROL_ALLOW_HEADERS => WILDCARD_ANY)]
    async fn handle(self, ctx: &Context) {
        trace!("CrossMiddleware handle");
    }
}

impl ServerHook for ResponseHeaderMiddleware {
    async fn new(_ctx: &Context) -> Self {
        trace!("ResponseHeaderMiddleware new");
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
    async fn handle(self, ctx: &Context) {
        trace!("ResponseHeaderMiddleware handle");
        let socket_addr_string: String = ctx.get_socket_addr_string().await;
        let content_type: String = ContentType::format_content_type_with_charset(TEXT_HTML, UTF8);
    }
}

impl ServerHook for ResponseStatusCodeMiddleware {
    async fn new(_ctx: &Context) -> Self {
        trace!("ResponseStatusCodeMiddleware new");
        Self
    }

    #[response_status_code(200)]
    async fn handle(self, ctx: &Context) {
        trace!("ResponseStatusCodeMiddleware handle");
    }
}

impl ServerHook for ResponseBodyMiddleware {
    async fn new(_ctx: &Context) -> Self {
        trace!("ResponseBodyMiddleware new");
        Self
    }

    #[response_body(INDEX_HTML.replace("{{ time }}", &time()))]
    async fn handle(self, ctx: &Context) {
        trace!("ResponseBodyMiddleware handle");
    }
}

impl ServerHook for OptionMethodMiddleware {
    async fn new(_ctx: &Context) -> Self {
        trace!("OptionMethodMiddleware new");
        Self
    }

    #[prologue_macros(
        filter(ctx.get_request_is_options().await),
        send
    )]
    async fn handle(self, ctx: &Context) {
        trace!("OptionMethodMiddleware handle");
        ctx.aborted().await;
    }
}

impl ServerHook for UpgradeMiddleware {
    async fn new(_ctx: &Context) -> Self {
        trace!("UpgradeMiddleware new");
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
    async fn handle(self, ctx: &Context) {
        trace!("UpgradeMiddleware handle");
    }
}
