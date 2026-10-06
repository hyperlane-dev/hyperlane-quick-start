use super::*;

/// Implementation of `TraceRoute` for `ServerHook`.
impl ServerHook for TraceRoute {
    #[instrument_trace]
    /// Builds the `ServerHook` state for the `TraceRoute` route.
    ///
    /// # Arguments
    ///
    /// - `&mut Stream` - The inbound request stream.
    /// - `&mut Context` - The mutable request and response context.
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        is_get_method,
        response_header(CONTENT_TYPE => ContentType::format_content_type_with_charset(TEXT_PLAIN, UTF8)),
        try_get_route_param(ROUTE_PARAM_KEY_TRACE => trace_opt)
    )]
    #[instrument_trace]
    /// Handles the `TraceRoute` route request and writes the JSON
    /// response envelope into the request context.
    ///
    /// # Arguments
    ///
    /// - `Self` - The route handler instance.
    /// - `&mut Stream` - The inbound request stream.
    /// - `&mut Context` - The mutable request and response context.
    ///
    /// # Returns
    ///
    /// - `Status` - The hook status that tells the server how to continue.
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        let trace: String = trace_opt.unwrap_or_default();
        let decoded_trace: String = decode(&trace)
            .unwrap_or_else(|_: FromUtf8Error| trace.clone().into())
            .into_owned();
        let result: String = TraceService::search_trace(&decoded_trace).await;
        ctx.get_mut_response().set_body(&result);
        Status::Continue
    }
}
