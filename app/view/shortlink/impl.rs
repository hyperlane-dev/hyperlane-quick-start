use super::*;

impl ServerHook for ShortlinkViewRoute {
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(
        methods(get, post),
        response_status_code(302),
        response_header(LOCATION => "/static/shortlink/index.html")
    )]
    async fn handle(self, ctx: &Context) {}
}
