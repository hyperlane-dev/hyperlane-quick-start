use super::*;

impl ServerHook for AccountBookingViewRoute {
    #[instrument_trace]
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        methods(get, post),
        response_status_code(302),
        response_header(LOCATION => "/static/account_booking/index.html")
    )]
    #[instrument_trace]
    async fn handle(self, _ctx: &mut Context) {}
}
