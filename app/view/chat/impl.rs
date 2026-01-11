use super::*;

impl ServerHook for ChatViewRoute {
    async fn new(_ctx: &Context) -> Self {
        trace!("ChatViewRoute new");
        Self
    }

    #[prologue_macros(
        methods(get, post),
        response_status_code(302),
        response_header(LOCATION => "/static/chat/index.html")
    )]
    async fn handle(self, ctx: &Context) {
        trace!("ChatViewRoute handle");
    }
}
