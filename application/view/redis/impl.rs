use super::*;

impl ServerHook for RedisViewRoute {
    #[instrument_trace]
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(
        methods(get, post),
        response_status_code(302),
        response_header(LOCATION => "/static/redis/index.html")
    )]
    #[instrument_trace]
    async fn handle(self, ctx: &Context) {}
}
