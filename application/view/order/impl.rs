use super::*;

impl ServerHook for OrderViewRoute {
    #[instrument_trace]
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        methods(get, post),
        response_status_code(302),
        response_header(LOCATION => "/static/order/index.html")
    )]
    #[instrument_trace]
    async fn handle(self, ctx: &mut Context) {
        let is_authenticated: bool = ctx.get_request().try_get_cookie(TOKEN).is_some();
        if !is_authenticated {
            ctx.get_mut_response()
                .set_header(LOCATION, format!("/auth?{LOCATION}=/order"));
        }
    }
}
