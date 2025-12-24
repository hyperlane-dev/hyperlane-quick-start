use super::*;

impl ServerHook for TraceRoute {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(
        get,
        response_header(CONTENT_TYPE => ContentType::format_content_type_with_charset(TEXT_PLAIN, UTF8)),
        route_param_option("trace" => trace_opt)
    )]
    async fn handle(self, ctx: &Context) {
        let trace: String = trace_opt.unwrap_or_default();
        let decoded_trace: String = decode(&trace)
            .unwrap_or_else(|_| trace.clone().into())
            .into_owned();
        let result: String = TraceService::search_trace(&decoded_trace).await;
        ctx.set_response_body(&result).await;
    }
}
