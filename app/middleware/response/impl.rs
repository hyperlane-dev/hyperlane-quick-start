use super::*;

impl ServerHook for SendMiddleware {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(
        http,
        reject(ctx.get_request_upgrade_type().await.is_ws()),
        send
    )]
    async fn handle(self, ctx: &Context) {}
}

impl ServerHook for LogMiddleware {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    async fn handle(self, ctx: &Context) {
        let request: String = ctx.get_request().await.get_string();
        let response: String = ctx.get_response().await.get_string();
        info!("{request}");
        info!("{response}");
    }
}
