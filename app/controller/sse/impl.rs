use super::*;

impl ServerHook for SseRoute {
    async fn new(_ctx: &Context) -> Self {
        trace!("SseRoute new");
        Self
    }

    #[prologue_macros(
        methods(get, post),
        response_body(EMPTY_STR),
        response_header(CONTENT_TYPE => TEXT_EVENT_STREAM)
    )]
    async fn handle(self, ctx: &Context) {
        trace!("SseRoute handle");
        ctx.send().await;
        for i in 0..10 {
            ctx.set_response_body(&format!("data:{i}{HTTP_DOUBLE_BR}"))
                .await
                .send_body()
                .await;
        }
        ctx.closed().await;
    }
}
