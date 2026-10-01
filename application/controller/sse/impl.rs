use super::*;

/// Implementation of `SseRoute` for `ServerHook`.
impl ServerHook for SseRoute {
    #[instrument_trace]
    /// Builds the `ServerHook` state for the `SseRoute` route.
    ///
    /// # Arguments
    ///
    /// - `&mut Stream` - The inbound request stream.
    /// - `&mut Context` - The mutable request and response context.
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        methods(get, post),
        response_body(EMPTY_STR),
        response_header(CONTENT_TYPE => TEXT_EVENT_STREAM),
        try_send
    )]
    #[instrument_trace]
    /// Handles the `SseRoute` route request and writes the JSON
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
    async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
        for i in 0..SSE_DEMO_ITERATION_COUNT {
            let data: String = format!("data:{i}{HTTP_DOUBLE_BR}");
            if stream.try_send(&data).await.is_err() {
                break;
            }
        }
        stream.set_closed(true);
        Status::Reject
    }
}
