use super::*;

impl ServerHook for LogMiddleware {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    async fn handle(self, ctx: &Context) {
        let request: String = ctx.get_request().await.get_string();
        let response: String = ctx.get_response().await.get_string();
        log_info(request).await;
        log_info(response).await
    }
}
