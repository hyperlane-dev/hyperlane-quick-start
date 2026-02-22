use super::*;

impl ServerHook for SendMiddleware {
    #[instrument_trace]
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        reject(ctx.get_request().is_ws_upgrade_type()),
        try_send
    )]
    #[instrument_trace]
    async fn handle(self, ctx: &mut Context) {}
}

impl ServerHook for LogMiddleware {
    #[instrument_trace]
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[instrument_trace]
    async fn handle(self, ctx: &mut Context) {
        let request_json: String = get_request_json(ctx).await;
        let response_json: String = get_response_json(ctx).await;
        info!("{request_json}");
        info!("{response_json}");
    }
}
