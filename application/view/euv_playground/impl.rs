use super::*;

/// Implementation of `EuvPlaygroundViewRoute` for `ServerHook`.
///
/// `GET /euv-playground` serves the SPA shell. Unauthenticated visitors
/// are redirected to `/auth?location=/euv-playground` (with the original
/// path URL-encoded into the `location` query parameter) so the post-login
/// page can send them back here. Authenticated visitors get the SPA
/// 302-redirected to its static HTML.
impl ServerHook for EuvPlaygroundViewRoute {
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        methods(get, post),
        response_status_code(302),
        response_header(LOCATION => EUV_PLAYGROUND_VIEW_REDIRECT_PATH)
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
