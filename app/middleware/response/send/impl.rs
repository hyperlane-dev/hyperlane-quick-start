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
