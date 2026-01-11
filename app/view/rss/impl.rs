use super::*;

impl ServerHook for RssViewRoute {
    async fn new(_ctx: &Context) -> Self {
        trace!("RssViewRoute new");
        Self
    }

    #[prologue_macros(
        methods(get, post),
        response_status_code(302),
        response_header(LOCATION => "/static/rss/index.html")
    )]
    async fn handle(self, ctx: &Context) {
        trace!("RssViewRoute handle");
    }
}
