use super::*;
impl ServerHook for DiffViewRoute {
    async fn new(_ctx: &Context) -> Self {
        trace!("DiffViewRoute new");
        Self
    }

    #[prologue_macros(
        methods(get, post),
        response_status_code(302),
        response_header(LOCATION => "/static/diff/index.html")
    )]
    async fn handle(self, ctx: &Context) {
        trace!("DiffViewRoute handle");
    }
}
