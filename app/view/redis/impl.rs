use super::*;

impl ServerHook for RedisViewRoute {
    async fn new(_ctx: &Context) -> Self {
        trace!("RedisViewRoute new");
        Self
    }

    #[prologue_macros(
        methods(get, post),
        response_status_code(302),
        response_header(LOCATION => "/static/redis/index.html")
    )]
    async fn handle(self, ctx: &Context) {
        trace!("RedisViewRoute handle");
    }
}
