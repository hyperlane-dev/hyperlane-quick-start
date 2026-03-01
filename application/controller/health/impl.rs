use super::*;

impl ServerHook for HealthCheckRoute {
    #[instrument_trace]
    async fn new(_ctx: &mut Context) -> Self {
        Self
    }

    #[prologue_macros(get_method)]
    #[instrument_trace]
    async fn handle(self, ctx: &mut Context) {}
}
