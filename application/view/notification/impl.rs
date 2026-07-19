use super::*;

/// Implementation of `NotificationViewRoute` for `ServerHook`.
impl ServerHook for NotificationViewRoute {
    #[instrument_trace]
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(
        methods(get, post),
        response_status_code(302),
        response_header(LOCATION => NOTIFICATION_VIEW_REDIRECT_PATH)
    )]
    #[instrument_trace]
    async fn handle(self, _stream: &mut Stream, ctx: &mut Context) -> Status {
        Status::Continue
    }
}
