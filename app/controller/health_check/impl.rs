use super::*;

impl ServerHook for HealthCheckRoute {
    #[instrument_trace]
    async fn new(_ctx: &Context) -> Self {
        Self
    }

    #[prologue_macros(get)]
    #[instrument_trace]
    async fn handle(self, ctx: &Context) {}
}
