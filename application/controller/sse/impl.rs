use super::*;

/// Implementation of `SseRoute` for `ServerHook`.
impl ServerHook for SseRoute {
    #[instrument_trace]
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
