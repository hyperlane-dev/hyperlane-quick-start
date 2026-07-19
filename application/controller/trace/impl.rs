use super::*;

/// Implementation of `TraceRoute` for `ServerHook`.
impl ServerHook for TraceRoute {
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        is_get_method,
        response_header(CONTENT_TYPE => ContentType::format_content_type_with_charset(TEXT_PLAIN, UTF8)),
        try_get_route_param("trace" => trace_opt)
    )]
    #[instrument_trace]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        let trace: String = trace_opt.unwrap_or_default();
        let decoded_trace: String = decode(&trace)
            .unwrap_or_else(|_: std::string::FromUtf8Error| trace.clone().into())
            .into_owned();
        let result: String = TraceService::search_trace(&decoded_trace).await;
        ctx.get_mut_response().set_body(&result);
        Status::Continue
    }
}
