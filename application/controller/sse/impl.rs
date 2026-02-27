use super::*;

impl ServerHook for SseRoute {
    #[instrument_trace]
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        methods(get, post),
        response_body(EMPTY_STR),
        response_header(CONTENT_TYPE => TEXT_EVENT_STREAM)
    )]
    #[instrument_trace]
    async fn handle(self, ctx: &mut Context) {
        ctx.send().await;
        for i in 0..10 {
            ctx.get_mut_response()
                .set_body(format!("data:{i}{HTTP_DOUBLE_BR}"));
            ctx.send_body().await;
        }
        ctx.set_closed(true);
    }
}
