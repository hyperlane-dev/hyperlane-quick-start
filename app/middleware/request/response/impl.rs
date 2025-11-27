use super::*;

impl ServerHook for ResponseHeaderMiddleware {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[epilogue_macros(
        response_header(DATE => gmt()),
        response_header(SERVER => HYPERLANE),
        response_header(CONNECTION => KEEP_ALIVE),
        response_header(CONTENT_TYPE => content_type),
        response_header("SocketAddr" => socket_addr_string),
        response_header(TRACE => uuid::Uuid::new_v4().to_string())
    )]
    async fn handle(self, ctx: &Context) {
        let socket_addr_string: String = ctx.get_socket_addr_string().await;
        let content_type: String = ContentType::format_content_type_with_charset(TEXT_HTML, UTF8);
    }
}

impl ServerHook for ResponseStatusCodeMiddleware {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[response_status_code(200)]
    async fn handle(self, ctx: &Context) {}
}

impl ServerHook for ResponseBodyMiddleware {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[response_body(INDEX_HTML.replace("{{ time }}", &time()))]
    async fn handle(self, ctx: &Context) {}
}
