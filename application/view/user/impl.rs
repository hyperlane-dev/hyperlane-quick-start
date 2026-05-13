use super::*;

impl ServerHook for UserViewRoute {
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        methods(get, post),
        response_status_code(302),
        response_header(LOCATION => "/static/user/index.html")
    )]
    #[instrument_trace]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        let is_authenticated: bool = ctx.get_request().try_get_cookie(TOKEN).is_some();
        if !is_authenticated {
            let redirect_url: String = build_auth_redirect_url(ctx);
            ctx.get_mut_response().set_header(LOCATION, redirect_url);
        }
        Status::Continue
    }
}
