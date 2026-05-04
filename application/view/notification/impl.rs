use super::*;

impl ServerHook for NotificationViewRoute {
    #[instrument_trace]
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        methods(get, post),
        response_status_code(302),
        response_header(LOCATION => "/static/notification/index.html")
    )]
    #[instrument_trace]
    async fn handle(self, _ctx: &mut Context) {}
}
